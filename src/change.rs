use std::ops::Range;

use git2::{DiffDelta, DiffLine, Oid};

#[derive(Debug)]
pub enum ChangeType {
    Add,
}

#[derive(Debug)]
pub struct Change {
    pub change_type: ChangeType,
    pub content: String,
    pub line_number: usize,
    pub blob_oid: Oid,
    pub content_range: Range<usize>,
}

impl Change {
    pub fn from_deltas<'c>(delta: DiffDelta<'c>, line: DiffLine<'c>) -> Self {
        let change_type = match line.origin() {
            '+' => ChangeType::Add,
            _ => todo!(),
        };

        let blob_oid = delta.new_file().id();

        let line_content = String::from_utf8(line.content().to_vec()).unwrap();

        let line_number_new = line.new_lineno().unwrap() as usize;
        let line_offset = line.content_offset() as usize;
        let line_len = line_content.len();
        let line_end = line_offset + line_len;

        let line_range = line_offset..line_end;

        Self {
            change_type,
            content: line_content,
            line_number: line_number_new,
            blob_oid,
            content_range: line_range,
        }
    }
}
