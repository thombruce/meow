use clap::{Parser, Subcommand};
use std::process::Command;

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
    Bar,
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
        Commands::Bar => {
            // Always spawn in kitten panel and disown
            spawn_in_panel();
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

/// Spawn bar executable in a kitten panel
fn spawn_in_panel() {
    // Find the bar executable
    let current_exe = std::env::current_exe().unwrap_or_else(|_| "catfood".into());
    let bar_exe = current_exe.parent()
        .unwrap_or(&current_exe)
        .join("catfood-bar");

    // Use shell to properly detach process
    let shell_cmd = format!("kitten panel {} &", bar_exe.display());
    
    match Command::new("sh").arg("-c").arg(&shell_cmd).spawn() {
        Ok(_child) => {
            // Give panel a moment to start then exit parent
            std::thread::sleep(std::time::Duration::from_millis(500));
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Failed to spawn kitten panel: {}", e);
            eprintln!(
                "Make sure Kitty is installed and you're running this in a Kitty environment."
            );
            std::process::exit(1);
        }
    }
}