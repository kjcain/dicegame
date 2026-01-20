mod args;
mod dice;
mod game;

use rayon::prelude::*;

fn main() {
    pretty_env_logger::init();

    let args = args::Args::parse_args();

    let game = game::Game::new(args.get_dice().expect("Failed to parse dice"), args.target);

    log::info!("Starting game with target: {}", game.target);
    log::info!("Using dice: {:?}", game.dice);
    log::info!("Running {} iterations in parallel across all CPU cores", args.iterations);

    let wins: u32 = (0..args.iterations)
        .into_par_iter()
        .map(|_| if game.play() { 1 } else { 0 })
        .sum();

    log::info!("Win Rate: {:.2}% across {} iterations", (wins as f64 / args.iterations as f64) * 100.0, args.iterations);
}
