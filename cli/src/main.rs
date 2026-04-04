use clap::{Parser, Subcommand};
use colored::Colorize;

#[cfg(feature = "analyze")]
mod analysis_engine;
mod audit_engine;
mod commands;
mod compliance;
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
    Update {
        /// Update method for the CLI binary: auto, github, or cargo
        #[arg(long, default_value = "auto", value_parser = ["auto", "github", "cargo"])]
        method: String,
    },
    /// Update the DevTrail framework to the latest version
    UpdateFramework,
    /// Update the CLI binary to the latest version
    UpdateCli {
        /// Update method: auto (detect), github (prebuilt binary), or cargo (compile from source)
        #[arg(long, default_value = "auto", value_parser = ["auto", "github", "cargo"])]
        method: String,
    },
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
        /// Validate only git-staged files (for pre-commit hooks)
        #[arg(long)]
        staged: bool,
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
    /// Create a new DevTrail document from a template
    New {
        /// Target directory (default: current directory)
        #[arg(default_value = ".")]
        path: String,
        /// Document type (e.g., ailog, adr, sec)
        #[arg(long, short = 't')]
        doc_type: Option<String>,
        /// Document title
        #[arg(long)]
        title: Option<String>,
    },
    /// Show version, author, and license information
    About,
    /// Analyze code complexity using cognitive and cyclomatic metrics
    #[cfg(feature = "analyze")]
    Analyze {
        /// Target directory or file (default: current directory)
        #[arg(default_value = ".")]
        path: String,
        /// Cognitive complexity threshold (default: from config or 8)
        #[arg(long)]
        threshold: Option<u32>,
        /// Output format
        #[arg(long, default_value = "text", value_parser = ["text", "json", "markdown"])]
        output: String,
        /// Show only top N most complex functions
        #[arg(long)]
        top: Option<usize>,
    },
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
        Commands::Update { method } => commands::update::run(&method),
        Commands::UpdateFramework => commands::update_framework::run(),
        Commands::UpdateCli { method } => commands::update_cli::run(&method),
        Commands::Remove { full } => commands::remove::run(full),
        Commands::Validate { path, fix, staged } => commands::validate::run(&path, fix, staged),
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
        Commands::New {
            path,
            doc_type,
            title,
        } => commands::new::run(&path, doc_type.as_deref(), title.as_deref()),
        Commands::Status { path } => commands::status::run(&path),
        Commands::Repair { path } => commands::repair::run(&path),
        Commands::About => commands::about::run(),
        #[cfg(feature = "analyze")]
        Commands::Analyze {
            path,
            threshold,
            output,
            top,
        } => commands::analyze::run(&path, threshold, &output, top),
        #[cfg(feature = "tui")]
        Commands::Explore { path } => commands::explore::run(&path),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}
