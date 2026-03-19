use anyhow::Result;
use colored::Colorize;

use crate::manifest::DistManifest;

pub fn run() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    let description = env!("CARGO_PKG_DESCRIPTION");
    let authors = env!("CARGO_PKG_AUTHORS");
    let license = env!("CARGO_PKG_LICENSE");
    let repository = env!("CARGO_PKG_REPOSITORY");
    let homepage = env!("CARGO_PKG_HOMEPAGE");

    println!();
    println!(
        "  {} {}",
        "DevTrail CLI".bold(),
        format!("v{version}").dimmed()
    );

    // Show framework version if installed
    if let Ok(cwd) = std::env::current_dir() {
        let manifest_path = cwd.join(".devtrail/dist-manifest.yml");
        if let Ok(manifest) = DistManifest::load(&manifest_path) {
            println!(
                "  {} {}",
                "Framework:".dimmed(),
                format!("v{}", manifest.version).dimmed()
            );
        }
    }

    println!("  {}", description.dimmed());
    println!();
    println!("  {}  {}", "Author:".cyan(), authors);
    println!("  {} {}", "License:".cyan(), license);
    println!("  {}    {}", "Repo:".cyan(), repository);
    println!("  {}     {}", "Web:".cyan(), homepage);
    println!();

    Ok(())
}
