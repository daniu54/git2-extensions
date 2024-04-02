use crate::Commit;

pub struct Repository<'r> {
    pub repository: git2::Repository,
    pub repository_path: &'r str,
    pub main_branch: Option<&'r str>,
}

impl<'r> Repository<'r> {
    pub fn new(repository_path: &'r str) -> Self {
        let repository = git2::Repository::open(repository_path).unwrap();

        Self {
            repository,
            repository_path,
            main_branch: None,
        }
    }

    pub fn get_commits_of_current_branch(&'r self) -> Vec<Commit> {
        let mut commits: Vec<Commit> = vec![];

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

        let mut revwalk = self.repository.revwalk().unwrap();
        revwalk.push(top_commit).unwrap();

        for oid in revwalk {
            let commit = self.repository.find_commit(oid.unwrap()).unwrap();

            if commit.id() == merge_base_commit {
                break;
            }

            let mut changed_files: Vec<String> = vec![];

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
                    changed_files.push(format!(
                        "{}/{}",
                        self.repository_path,
                        delta.new_file().path().unwrap().to_str().unwrap()
                    ));
                });
            }

            let extended_commit = Commit {
                commit,
                changed_files,
            };

            commits.push(extended_commit);
        }

        commits
    }

    pub fn set_main_branch(&mut self, main_branch_name: &'r str) {
        self.main_branch = Some(main_branch_name);
    }
}
