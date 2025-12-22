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
        #[arg(short, long, help = "Run in panel mode")]
        panel: bool,
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

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bar { panel: _ } => {
            if let Err(e) = catfood_bar::run() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
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
