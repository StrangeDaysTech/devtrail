use clap::{Parser, Subcommand};
use colored::Colorize;

mod commands;
mod config;
mod download;
mod inject;
mod manifest;
mod platform;
mod self_update;
mod utils;

/// DevTrail CLI - Documentation Governance for AI-Assisted Development
#[derive(Parser)]
#[command(name = "devtrail", version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize DevTrail in a project directory
    Init {
        /// Target directory (default: current directory)
        #[arg(default_value = ".")]
        path: String,
    },
    /// Update DevTrail to the latest version
    Update,
    /// Update the CLI binary to the latest version
    UpdateCli,
    /// Remove DevTrail from the project
    Remove {
        /// Remove everything including user-generated documents (requires confirmation)
        #[arg(long)]
        full: bool,
    },
    /// Show version, author, and license information
    About,
}

fn main() {
    // Clean up leftover binary from previous update
    self_update::cleanup_old_binary();

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init { path } => commands::init::run(&path),
        Commands::Update => commands::update::run(),
        Commands::UpdateCli => commands::update_cli::run(),
        Commands::Remove { full } => commands::remove::run(full),
        Commands::About => commands::about::run(),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}
