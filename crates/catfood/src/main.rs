use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
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

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bar => {
            // Check if bar is already running
            if is_bar_running()? {
                eprintln!("catfood bar is already running");
                std::process::exit(1);
            }

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
    let bar_exe = current_exe
        .parent()
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

/// Get the PID file path
fn get_pid_file_path() -> color_eyre::Result<PathBuf> {
    let data_dir = std::env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.local/share", home)
    });

    let catfood_dir = PathBuf::from(data_dir).join("catfood");
    fs::create_dir_all(&catfood_dir)?;

    Ok(catfood_dir.join("bar.pid"))
}

/// Check if bar is already running by checking PID file
fn is_bar_running() -> color_eyre::Result<bool> {
    let pid_file_path = get_pid_file_path()?;

    if !pid_file_path.exists() {
        return Ok(false);
    }

    let pid_content = fs::read_to_string(&pid_file_path)?;
    let pid: u32 = pid_content
        .trim()
        .parse()
        .map_err(|_| color_eyre::eyre::eyre!("Invalid PID in PID file"))?;

    // Check if process exists by sending signal 0
    unsafe {
        if libc::kill(pid as i32, 0) == 0 {
            Ok(true) // Process exists and is alive
        } else {
            // Process doesn't exist, remove stale PID file
            let _ = fs::remove_file(&pid_file_path);
            Ok(false)
        }
    }
}
