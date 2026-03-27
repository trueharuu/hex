use clap::Parser;

use hex::bot::Bot;
use hex::bots::close::CloseBot;
use hex::bots::first::FirstBot;
use hex::bots::random::RandomBot;
use std::sync::Arc;

#[derive(clap::Parser)]
pub struct Program {
    #[clap(short, long, default_value = "100")]
    sample: u32,
    #[clap(short, long)]
    verbosity: u32,
    #[clap(short, long, default_value = "100")]
    turns: u32,
}

pub fn main() -> anyhow::Result<()> {
    let args = Program::parse();

    let bot_factories: Vec<Arc<dyn Fn() -> Box<dyn Bot> + Send + Sync>> = vec![
        Arc::new(|| Box::new(CloseBot::new())),
        Arc::new(|| Box::new(RandomBot::new(3))),
        Arc::new(|| Box::new(FirstBot::new())),
    ];

    std::fs::remove_dir_all(format!("output/sets")).ok();
            
    hex::tournament::tournament(&bot_factories, args.sample, args.turns, args.verbosity)?;
    Ok(())
}
