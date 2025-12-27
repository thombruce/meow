use catfood_bar::{handle_bar_cli, run_bar};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "catfood")]
#[command(about = "A utility suite for system management")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run the system bar
    Bar {
        /// Run without spawning in a kitten panel
        #[arg(long = "no-kitten")]
        no_kitten: bool,
    },
    /// Run the menu system
    Menu {
        #[arg(short, long, help = "Show menu categories")]
        categories: bool,
    },
    /// Run notification system
    Notifications {
        #[arg(short, long, help = "Enable do-not-disturb mode")]
        dnd: bool,
    },
}

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bar { no_kitten } => {
            // Handle common CLI logic
            if handle_bar_cli(no_kitten) {
                return Ok(()); // Process spawned in panel and exited
            }

            // Run directly with existing behavior (--no-kitten case)
            run_bar()?;
        }
        Commands::Menu { categories: _ } => {
            println!("Menu feature coming soon!");
            std::process::exit(0);
        }
        Commands::Notifications { dnd: _ } => {
            println!("Notifications feature coming soon!");
            std::process::exit(0);
        }
    }

    Ok(())
}
