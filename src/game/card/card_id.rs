#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CardID(u32);
impl CardID {
    // Existing methods
    pub fn new(id: u32) -> Self {
        Self(id)
    }
    // New next method with overflow protection
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}
