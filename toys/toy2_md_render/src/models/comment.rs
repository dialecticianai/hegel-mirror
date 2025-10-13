/// A review comment with source position
#[derive(Clone)]
pub struct Comment {
    pub text: String,
    pub line_start: usize,
    pub col_start: usize,
    pub line_end: usize,
    pub col_end: usize,
}

impl Comment {
    pub fn new(
        text: String,
        line_start: usize,
        col_start: usize,
        line_end: usize,
        col_end: usize,
    ) -> Self {
        Self {
            text,
            line_start,
            col_start,
            line_end,
            col_end,
        }
    }

    pub fn format(&self) -> String {
        format!(
            "[L{}:C{} → L{}:C{}] {}",
            self.line_start, self.col_start, self.line_end, self.col_end, self.text
        )
    }
}
