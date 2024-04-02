use crate::File;

#[derive(Debug)]
pub struct Commit<'r> {
    pub commit: git2::Commit<'r>,
    pub changed_files: Vec<File>,
}
