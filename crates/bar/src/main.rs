use catfood_bar::{handle_bar_cli, run_bar};
use clap::Parser;

#[derive(Parser)]
#[command(name = "catfood-bar")]
#[command(about = "A system bar component of the catfood utility suite")]
struct Cli {
    /// Run without spawning in a kitten panel
    #[arg(long = "no-kitten")]
    no_kitten: bool,
}

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    // Handle common CLI logic
    if handle_bar_cli(cli.no_kitten) {
        return Ok(()); // Process spawned in panel and exited
    }

    // Run directly with existing behavior (--no-kitten case)
    run_bar()
}
