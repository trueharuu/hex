use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Diag {
    GameOver,
    InvalidTurn,
    CellOutOfBounds,
    CellOccupied,
}

impl std::error::Error for Diag {}
impl Display for Diag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Diag::GameOver => write!(f, "game over"),
            Diag::InvalidTurn => write!(f, "invalid turn"),
            Diag::CellOutOfBounds => write!(f, "cell out of bounds"),
            Diag::CellOccupied => write!(f, "cell occupied"),
        }
    }
}
