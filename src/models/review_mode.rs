/// Review mode: immediate vs batched
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewMode {
    /// Each comment saves immediately to disk
    Immediate,
    /// Comments queued in memory, atomic write on submit
    Batched,
}

impl Default for ReviewMode {
    fn default() -> Self {
        Self::Immediate
    }
}
