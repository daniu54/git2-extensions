use crate::{Commit, File};

use std::convert::AsRef;
use std::ops::Deref;

pub struct Repository<'r> {
    repository: git2::Repository,
    pub repository_path: &'r str,
    pub main_branch: Option<&'r str>,
}

impl<'r> Deref for Repository<'r> {
    type Target = git2::Repository;

    fn deref(&self) -> &Self::Target {
        &self.repository
    }
}

impl<'r> AsRef<git2::Repository> for Repository<'r> {
    fn as_ref(&self) -> &git2::Repository {
        &self.repository
    }
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

        let head = self.head().unwrap();
        let top_commit = head.target().unwrap();

        let main = self
            .find_branch(self.main_branch.unwrap(), git2::BranchType::Local)
            .unwrap();

        let main_top_commit = main.get().peel_to_commit().unwrap();

        let merge_base_commit = self.merge_base(top_commit, main_top_commit.id()).unwrap();

        let mut revwalk = self.revwalk().unwrap();
        revwalk.push(top_commit).unwrap();

        for oid in revwalk {
            let commit = self.find_commit(oid.unwrap()).unwrap();

            if commit.id() == merge_base_commit {
                break;
            }

            let mut changed_files: Vec<File> = vec![];

            for parent in commit.parents() {
                let diff = self
                    .diff_tree_to_tree(
                        Some(&parent.tree().unwrap()),
                        Some(&commit.tree().unwrap()),
                        None,
                    )
                    .unwrap();

                changed_files.append(
                    &mut diff
                        .deltas()
                        .map(|delta| File::from_delta(delta, self))
                        .collect(),
                );
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
