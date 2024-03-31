use crate::ExtendedCommit;

pub struct ExtendedRepository<'r> {
    pub repository: git2::Repository,
    pub repository_path: &'r str,
    pub main_branch: Option<&'r str>,
}

impl<'r> ExtendedRepository<'r> {
    pub fn new(repository_path: &'r str) -> Self {
        let repository = git2::Repository::open(repository_path).unwrap();

        Self {
            repository,
            repository_path,
            main_branch: None,
        }
    }

    // FIXME does not work as expected, example usage:
    // let diff_options = &mut DiffOptions::new();

    // diff_options.include_untracked(true);
    // diff_options.recurse_untracked_dirs(true);

    // let changed_files = repo.get_changed_files(diff_options);
    // println!("Added files on current branch:");
    // for file in changed_files {
    //     println!("{}", file);
    // }
    pub(crate) fn _get_changed_files(
        &'r self,
        diff_options: &mut git2::DiffOptions,
    ) -> Vec<String> {
        let mut files: Vec<String> = vec![];

        // TODO need more precision here
        files.append(
            &mut self
                .repository
                .diff_index_to_workdir(None, Some(diff_options))
                .unwrap()
                .deltas()
                .map(|f| f.new_file().path().unwrap().to_str().unwrap().to_string())
                .collect::<Vec<String>>(),
        );

        // TODO need more precision here
        files.append(
            &mut self
                .repository
                .diff_index_to_workdir(None, Some(diff_options))
                .unwrap()
                .deltas()
                .map(|f| f.old_file().path().unwrap().to_str().unwrap().to_string())
                .collect::<Vec<String>>(),
        );

        files
    }

    pub fn get_commits_of_current_branch(&'r self) -> Vec<ExtendedCommit> {
        // TODO rename
        let mut files_to_check: Vec<ExtendedCommit> = vec![];

        let head = self.repository.head().unwrap();
        let top_commit = head.target().unwrap();

        let main = self
            .repository
            .find_branch(self.main_branch.unwrap(), git2::BranchType::Local)
            .unwrap();

        let main_top_commit = main.get().peel_to_commit().unwrap();

        let merge_base_commit = self
            .repository
            .merge_base(top_commit, main_top_commit.id())
            .unwrap();

        // FIXME probably don't need to iterate and manually check
        // FIXME instead use https://docs.rs/git2/latest/git2/struct.Revwalk.html#method.push_range with top_commit + main
        let mut revwalk = self.repository.revwalk().unwrap();
        revwalk.push(top_commit).unwrap();

        for oid in revwalk {
            let commit = self.repository.find_commit(oid.unwrap()).unwrap();

            if commit.id() == merge_base_commit {
                break;
            }

            // println!("{:?}", commit);

            let mut new_files_to_check: Vec<String> = vec![];

            for parent in commit.parents() {
                let diff = self
                    .repository
                    .diff_tree_to_tree(
                        Some(&parent.tree().unwrap()),
                        Some(&commit.tree().unwrap()),
                        None,
                    )
                    .unwrap();

                diff.deltas().for_each(|delta| {
                    new_files_to_check.push(format!(
                        "{}/{}",
                        self.repository_path,
                        delta.new_file().path().unwrap().to_str().unwrap()
                    ));
                });
            }

            let extended_commit = ExtendedCommit {
                commit,
                changed_files: new_files_to_check,
            };

            files_to_check.push(extended_commit);
        }

        files_to_check
    }

    pub fn set_main_branch(&mut self, main_branch_name: &'r str) {
        self.main_branch = Some(main_branch_name);
    }
}
