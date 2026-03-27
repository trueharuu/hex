use rustc_hash::{FxHashMap, FxHashSet};

use crate::coords::Hex;

pub const WIN_LENGTH: i32 = 6;
pub const MAX_PLACE_DIST: i32 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Cell {
    X,
    O,
}

impl Cell {
    pub fn opponent(self) -> Cell {
        match self {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    cells: FxHashMap<Hex, Cell>,
    occupied: FxHashSet<Hex>,
    candidates: FxHashSet<Hex>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: FxHashMap::default(),
            occupied: FxHashSet::default(),
            candidates: FxHashSet::default(),
        }
    }

    pub fn get(&self, h: Hex) -> Option<Cell> {
        self.cells.get(&h).copied()
    }

    pub fn is_empty(&self, h: Hex) -> bool {
        !self.occupied.contains(&h)
    }

    pub fn occupied_count(&self) -> usize {
        self.occupied.len()
    }

    /// Place a piece for `player` at `h`.  Panics if the cell is occupied.
    /// Does NOT validate the distance rule — call `is_legal_placement` first.
    pub fn place(&mut self, h: Hex, player: Cell) {
        assert!(self.is_empty(h), "Cell {h} is already occupied");
        self.cells.insert(h, player);
        self.occupied.insert(h);
        self.candidates.remove(&h);
        self.expand_zone(h);
    }

    pub fn cells(&self) -> impl Iterator<Item = (&Hex, &Cell)> {
        self.cells.iter()
    }

    fn expand_zone(&mut self, origin: Hex) {
        // Expand both zones in a single pass over the bounding box.
        let d = MAX_PLACE_DIST;
        for dq in -d..=d {
            for dr in (-d).max(-d - dq)..=d.min(d - dq) {
                let h = Hex::new(origin.q + dq, origin.r + dr);
                let dist = origin.distance(&h);
                if dist <= d && self.is_empty(h) {
                    self.candidates.insert(h);
                }
            }
        }
    }

    /// Is placing at `h` legal?  Cell must be empty and within MAX_PLACE_DIST
    /// of at least one existing piece.
    ///
    /// Note: the very first placement (0,0) is handled directly by `Game` before
    /// this is ever called, so we never need to special-case an empty board here.
    pub fn is_legal_placement(&self, h: Hex) -> bool {
        self.is_empty(h) && self.candidates.contains(&h)
    }

    /// All currently legal placement targets (empty cells within MAX_PLACE_DIST
    /// of any existing piece).  Does not include (0,0) on an empty board —
    /// `Game` handles the first move specially.
    pub fn legal_placements(&self) -> impl Iterator<Item = Hex> + '_ {
        self.candidates.iter().copied()
    }

    /// Count consecutive `player` pieces from `start` in direction `dir`.
    fn run_len(&self, start: Hex, dir: Hex, player: Cell) -> i32 {
        let mut h = start + dir;
        let mut len = 0;
        while self.get(h) == Some(player) {
            len += 1;
            h = h + dir;
        }
        len
    }

    /// Length of the maximal straight line through `h` along `axis` for `player`.
    pub fn line_through(&self, h: Hex, axis_idx: usize, player: Cell) -> i32 {
        let (fwd, bwd) = Hex::AXES[axis_idx];
        1 + self.run_len(h, fwd, player) + self.run_len(h, bwd, player)
    }

    /// Check if placing the last piece at `h` for `player` created a win.
    /// Only need to check lines through `h`.
    pub fn is_winning_move(&self, h: Hex, player: Cell) -> bool {
        (0..3).any(|axis| self.line_through(h, axis, player) >= WIN_LENGTH)
    }

    /// Full board scan for a winner (slower, use only for verification).
    pub fn find_winner(&self) -> Option<Cell> {
        for (&h, &player) in &self.cells {
            for axis in 0..3 {
                if self.line_through(h, axis, player) >= WIN_LENGTH {
                    return Some(player);
                }
            }
        }
        None
    }

    /// Heuristic score for placing `player`'s piece at `h` on the current board.
    ///
    /// Returns a tuple `(attack, defence)` where:
    ///   `attack`  = longest line `player` would have through `h` after placing
    ///   `defence` = longest line `opponent` currently has through `h` (what we'd block)
    ///
    /// Higher is better for both dimensions. Callers combine them as they see fit.
    pub fn score_cell(&self, h: Hex, player: Cell) -> (i32, i32) {
        let opponent = player.opponent();
        let mut attack = 0i32;
        let mut defence = 0i32;
        for axis in 0..3 {
            // Attack: how long would our line be if we placed here?
            // line_through counts `h` itself + neighbours of same colour.
            // We treat h as if it were already our colour by counting the runs.
            let (fwd, bwd) = Hex::AXES[axis];
            let our_run = 1 + self.run_len(h, fwd, player) + self.run_len(h, bwd, player);
            attack = attack.max(our_run);

            // Defence: how long is the opponent's line through h right now?
            let opp_run = self.run_len(h, fwd, opponent) + self.run_len(h, bwd, opponent);
            // If we place here we break their line, so the threat value is how
            // many of their pieces we sever (opp_run, not +1 since h is empty).
            defence = defence.max(opp_run);
        }
        (attack, defence)
    }

    /// Best (longest) line length for `player` across the whole board.
    pub fn max_line_len(&self, player: Cell) -> i32 {
        self.cells
            .iter()
            .filter(|(_, c)| **c == player)
            .flat_map(|(&h, _)| (0..3).map(move |ax| (h, ax)))
            .map(|(h, ax)| self.line_through(h, ax, player))
            .max()
            .unwrap_or(0)
    }
}
