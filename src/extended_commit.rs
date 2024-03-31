#[derive(Debug)]
pub struct ExtendedCommit<'r> {
    pub commit: git2::Commit<'r>,
    pub changed_files: Vec<String>,
}
