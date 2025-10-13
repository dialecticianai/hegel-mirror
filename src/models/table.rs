use pulldown_cmark::Alignment;

/// Represents a markdown table
#[derive(Clone, Debug)]
pub struct Table {
    /// Column alignments
    pub alignments: Vec<Alignment>,
    /// Header row (if present)
    pub header: Vec<String>,
    /// Body rows
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(alignments: Vec<Alignment>) -> Self {
        Self {
            alignments,
            header: Vec::new(),
            rows: Vec::new(),
        }
    }
}
