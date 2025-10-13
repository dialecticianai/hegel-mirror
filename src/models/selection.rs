/// Current selection state
#[derive(Default, Clone)]
pub struct Selection {
    /// Line-based selection (MVP: no column precision)
    pub start_line: Option<usize>,
    pub end_line: Option<usize>,
    /// Track if we're currently dragging to select
    pub is_dragging: bool,
}

impl Selection {
    pub fn is_active(&self) -> bool {
        self.start_line.is_some() && self.end_line.is_some()
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn start_drag(&mut self, line: usize) {
        self.start_line = Some(line);
        self.end_line = Some(line);
        self.is_dragging = true;
    }

    pub fn update_drag(&mut self, line: usize) {
        if self.is_dragging {
            self.end_line = Some(line);
        }
    }

    pub fn end_drag(&mut self) {
        self.is_dragging = false;
    }

    /// Check if a line is within the selected range
    pub fn contains_line(&self, line: usize) -> bool {
        if let (Some(start), Some(end)) = (self.start_line, self.end_line) {
            let (min_line, max_line) = if start <= end {
                (start, end)
            } else {
                (end, start)
            };
            line >= min_line && line <= max_line
        } else {
            false
        }
    }
}
