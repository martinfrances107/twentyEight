#[derive(Debug, PartialEq)]
pub(crate) struct PacMan {
    // board position.
    position: usize,
    // pixel offset into sprite array.
    animatation_state: usize,
}

impl Default for PacMan {
    fn default() -> Self {
        Self {
            position: 602,
            animatation_state: 0,
        }
    }
}
