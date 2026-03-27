use clap::{Parser, Subcommand};
use colored::Colorize;

mod audit_engine;
mod commands;
mod compliance;
mod complexity;
mod config;
mod document;
mod download;
mod inject;
mod manifest;
mod metrics_engine;
mod platform;
mod self_update;
#[cfg(feature = "tui")]
mod tui;
mod utils;
mod validation;

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
    /// Update both framework and CLI to the latest version
    Update,
    /// Update the DevTrail framework to the latest version
    UpdateFramework,
    /// Update the CLI binary to the latest version
    UpdateCli,
    /// Remove DevTrail from the project
    Remove {
        /// Remove everything including user-generated documents (requires confirmation)
        #[arg(long)]
        full: bool,
    },
    /// Show DevTrail installation status and documentation statistics
    Status {
        /// Target directory (default: current directory)
        #[arg(default_value = ".")]
        path: String,
    },
    /// Repair DevTrail structure by restoring missing directories and files
    Repair {
        /// Target directory (default: current directory)
        #[arg(default_value = ".")]
        path: String,
    },
    /// Validate DevTrail documents for compliance and correctness
    Validate {
        /// Target directory (default: current directory)
        #[arg(default_value = ".")]
        path: String,
        /// Automatically fix simple issues
        #[arg(long)]
        fix: bool,
    },
    /// Check regulatory compliance (EU AI Act, ISO 42001, NIST AI RMF)
    Compliance {
        /// Target directory (default: current directory)
        #[arg(default_value = ".")]
        path: String,
        /// Check a specific standard
        #[arg(long, value_parser = ["eu-ai-act", "iso-42001", "nist-ai-rmf"])]
        standard: Option<String>,
        /// Check all standards
        #[arg(long)]
        all: bool,
        /// Output format
        #[arg(long, default_value = "text", value_parser = ["text", "markdown", "json"])]
        output: String,
    },
    /// Generate audit trail reports with timeline and traceability
    Audit {
        /// Target directory (default: current directory)
        #[arg(default_value = ".")]
        path: String,
        /// Start date for audit period (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,
        /// End date for audit period (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
        /// Filter by system/component name
        #[arg(long)]
        system: Option<String>,
        /// Output format
        #[arg(long, default_value = "text", value_parser = ["text", "markdown", "json", "html"])]
        output: String,
    },
    /// Show governance metrics and documentation statistics
    Metrics {
        /// Target directory (default: current directory)
        #[arg(default_value = ".")]
        path: String,
        /// Time period for metrics
        #[arg(long, default_value = "last-30-days", value_parser = ["last-7-days", "last-30-days", "last-90-days", "all"])]
        period: String,
        /// Output format
        #[arg(long, default_value = "text", value_parser = ["text", "markdown", "json"])]
        output: String,
    },
    /// Show version, author, and license information
    About,
    /// Explore DevTrail documentation interactively
    #[cfg(feature = "tui")]
    Explore {
        /// Target directory (default: current directory)
        #[arg(default_value = ".")]
        path: String,
    },
}

fn main() {
    // Clean up leftover binary from previous update
    self_update::cleanup_old_binary();

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init { path } => commands::init::run(&path),
        Commands::Update => commands::update::run(),
        Commands::UpdateFramework => commands::update_framework::run(),
        Commands::UpdateCli => commands::update_cli::run(),
        Commands::Remove { full } => commands::remove::run(full),
        Commands::Validate { path, fix } => commands::validate::run(&path, fix),
        Commands::Audit {
            path,
            from,
            to,
            system,
            output,
        } => commands::audit::run(&path, from.as_deref(), to.as_deref(), system.as_deref(), &output),
        Commands::Compliance {
            path,
            standard,
            all,
            output,
        } => commands::compliance::run(&path, standard.as_deref(), all, &output),
        Commands::Metrics {
            path,
            period,
            output,
        } => commands::metrics::run(&path, &period, &output),
        Commands::Status { path } => commands::status::run(&path),
        Commands::Repair { path } => commands::repair::run(&path),
        Commands::About => commands::about::run(),
        #[cfg(feature = "tui")]
        Commands::Explore { path } => commands::explore::run(&path),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}
