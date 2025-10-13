/// Current selection state
#[derive(Default, Clone)]
pub struct Selection {
    /// Index into chunks array
    pub start_chunk: Option<usize>,
    pub end_chunk: Option<usize>,
    /// Character offset within the chunk's text
    pub start_offset: usize,
    pub end_offset: usize,
}

impl Selection {
    pub fn is_active(&self) -> bool {
        self.start_chunk.is_some() && self.end_chunk.is_some()
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn set_single_chunk(&mut self, chunk_idx: usize, text_len: usize) {
        self.start_chunk = Some(chunk_idx);
        self.end_chunk = Some(chunk_idx);
        self.start_offset = 0;
        self.end_offset = text_len;
    }
}
