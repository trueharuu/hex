use rand::Rng;

use crate::{
    board::Cell,
    bot::Bot,
    game::{Game, Turn},
};

pub struct RandomBot {
    radius: i32,
    n: String,
}

impl RandomBot {
    pub fn new(radius: i32) -> Self {
        Self {
            radius,
            n: format!("random:{}", radius),
        }
    }
}

impl Bot for RandomBot {
    fn name(&self) -> &str {
        &self.n
    }

    fn choose(&mut self, game: &Game, player: Cell) -> Turn {
        let mut candidates = game.board.legal_placements().collect::<Vec<_>>();
        assert!(candidates.len() >= 2);

        // select winning moves first
        if let Some(&win_hex) = candidates
            .iter()
            .find(|&&h| game.board.is_winning_move(h, player))
        {
            // Pick any other legal cell as the second placement.
            let second = candidates.iter().find(|&&h| h != win_hex).copied().unwrap();
            return Turn::Two(win_hex, second);
        }

        // filter candidates by radius from any other cell
        if self.radius >= 0 {
            candidates.retain(|&h| {
                game.board
                    .cells()
                    .any(|(c, _)| h.distance(&c) <= self.radius)
            });
        }
        assert!(candidates.len() >= 2);

        let mut r = rand::rng();

        let i = r.next_u64() % candidates.len() as u64;
        candidates.swap(0, i as usize);
        let j = r.next_u64() % (candidates.len() - 1) as u64;
        candidates.swap(1, j as usize);

        Turn::Two(candidates[0], candidates[1])
    }
}
