use crate::{Change, Repository};

use git2::{DiffDelta, DiffHunk, DiffLine, DiffOptions};

#[derive(Debug)]
pub struct File {
    pub file_path: String,
    pub changes: Vec<Change>,
}

impl File {
    pub fn from_delta<'r>(delta: git2::DiffDelta<'r>, repository: &Repository<'r>) -> Self {
        let new_file = delta.new_file();
        let old_file = delta.old_file();

        let new_file_path = new_file.path().and_then(|p| p.to_str());

        let file_path = format!("{}/{}", repository.repository_path, new_file_path.unwrap());

        if delta.new_file().is_binary() {
            return File {
                file_path,
                changes: vec![],
            };
        }

        let mut changes = vec![];

        let old_file_path = old_file.path().and_then(|p| p.to_str());

        let blob_new = repository.repository.find_blob(new_file.id()).ok();
        let blob_old = repository.repository.find_blob(old_file.id()).ok();

        let mut line_cb =
            |delta: DiffDelta<'_>, _hunk: Option<DiffHunk<'_>>, line: DiffLine<'_>| {
                if line.origin() == '+' {
                    changes.push(Change::from_deltas(delta, line));
                }
                true
            };

        let mut opts = DiffOptions::new();

        repository
            .repository
            .diff_blobs(
                blob_old.as_ref(),
                old_file_path,
                blob_new.as_ref(),
                new_file_path,
                Some(&mut opts),
                None,
                None,
                None,
                Some(&mut line_cb),
            )
            .unwrap();

        File { file_path, changes }
    }
}
