use hex::board::WIN_LENGTH;
use hex::coords::Hex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
fn parse_cube_coord(s: &str) -> Option<Hex> {
    let s = s.trim();
    let s = s.strip_prefix('(')?.strip_suffix(')')?.trim();
    let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
    if parts.len() != 3 {
        return None;
    }
    let x = parts[0].parse::<i32>().ok()?;
    let y = parts[1].parse::<i32>().ok()?;
    let z = parts[2].parse::<i32>().ok()?;
    // accept cube coords (x,y,z) with x+y+z==0; map to axial (q = x, r = z)
    if x + y + z != 0 {
        return None;
    }
    Some(Hex::new(x, z))
}

fn compute_winning_cells(map: &HashMap<Hex, char>) -> HashSet<Hex> {
    let mut winners = HashSet::new();
    for (&h, &player) in map.iter() {
        for axis in 0..3 {
            let (fwd, bwd) = Hex::AXES[axis];
            // forward
            let mut cur = h;
            let mut fwd_cells = Vec::new();
            loop {
                let next = cur + fwd;
                match map.get(&next) {
                    Some(&p) if p == player => {
                        fwd_cells.push(next);
                        cur = next;
                    }
                    _ => break,
                }
            }
            // backward
            let mut cur = h;
            let mut bwd_cells = Vec::new();
            loop {
                let next = cur + bwd;
                match map.get(&next) {
                    Some(&p) if p == player => {
                        bwd_cells.push(next);
                        cur = next;
                    }
                    _ => break,
                }
            }
            let total_len = 1 + fwd_cells.len() + bwd_cells.len();
            if total_len as i32 >= WIN_LENGTH {
                winners.insert(h);
                for &c in &fwd_cells {
                    winners.insert(c);
                }
                for &c in &bwd_cells {
                    winners.insert(c);
                }
            }
        }
    }
    winners
}

fn print_board(map: &HashMap<Hex, char>) {
    if map.is_empty() {
        println!("(empty board)");
        return;
    }

    let min_q = map.keys().map(|h| h.q).min().unwrap();
    let max_q = map.keys().map(|h| h.q).max().unwrap();
    let min_r = map.keys().map(|h| h.r).min().unwrap();
    let max_r = map.keys().map(|h| h.r).max().unwrap();

    let winners = compute_winning_cells(map);

    // Print from top (max_r) down to min_r so the output looks intuitive.
    for r in (min_r..=max_r).rev() {
        // indent rows to give a hex-like staggered appearance
        let pad = (max_r - r) as usize;
        print!("{}", "  ".repeat(pad));
        for q in min_q..=max_q {
            let h = Hex::new(q, r);
            let ch = map.get(&h).copied().unwrap_or('.');
            let out = if q == 0 && r == 0 {
                match ch {
                    'X' => "\x1b[1;32mX\x1b[0m", // green X
                    'O' => "\x1b[1;32mO\x1b[0m", // green O
                    _ => "?",
                }
            } else {
                match ch {
                    'X' => {
                        if winners.contains(&h) {
                            "\x1b[1;31mX\x1b[0m" // bold red X
                        } else {
                            "\x1b[31mX\x1b[0m"
                        }
                    }
                    'O' => {
                        if winners.contains(&h) {
                            "\x1b[1;34mO\x1b[0m" // bold blue O
                        } else {
                            "\x1b[34mO\x1b[0m"
                        }
                    }
                    '.' => "\x1b[30m \x1b[0m",
                    _ => "?",
                }
            };
            print!("{} ", out);
        }
        println!();
    }
}

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let path = args.next();

    let reader: Box<dyn BufRead> = match path {
        Some(p) => Box::new(BufReader::new(File::open(p)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut map: HashMap<Hex, char> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let s = line.trim();
        if s.is_empty() {
            continue;
        }
        if s.starts_with('#') {
            continue;
        }

        // format: PLAYER;(x,y,z);(x,y,z)
        let mut parts = s.split(';').map(|p| p.trim());
        let player = parts.next().and_then(|p| p.chars().next()).unwrap_or('?');
        for coord in parts {
            if coord.is_empty() {
                continue;
            }
            if let Some(h) = parse_cube_coord(coord) {
                map.insert(h, player.to_ascii_uppercase());
            }
        }
    }

    print_board(&map);

    Ok(())
}
