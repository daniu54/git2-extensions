use std::ops::Deref;

use crate::File;

#[derive(Debug)]
pub struct Commit<'r> {
    pub commit: git2::Commit<'r>,
    pub changed_files: Vec<File>,
}

impl<'r> Deref for Commit<'r> {
    type Target = git2::Commit<'r>;

    fn deref(&self) -> &Self::Target {
        &self.commit
    }
}

impl<'r> AsRef<git2::Commit<'r>> for Commit<'r> {
    fn as_ref(&self) -> &git2::Commit<'r> {
        &self.commit
    }
}