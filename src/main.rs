mod args;
mod dice;
mod game;

fn main() {
    pretty_env_logger::init();

    let args = args::Args::parse_args();

    let game = game::Game::new(args.get_dice().expect("Failed to parse dice"), args.target);

    log::info!("Starting game with target: {}", game.target);
    log::info!("Using dice: {:?}", game.dice);

    let mut wins = 0;

    for _ in 0..args.iterations {
        if game.play() {
            wins += 1;
        }
    }

    log::info!("Win Rate: {:.2}% across {} iterations", (wins as f64 / args.iterations as f64) * 100.0, args.iterations);
}
