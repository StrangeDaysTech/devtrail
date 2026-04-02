# DevTrail — Development Instructions

This is the DevTrail project repository. It contains two main components:

- **Framework** (`dist/`): documentation templates, governance policies, and agent directives
- **CLI** (`cli/`): the `devtrail` Rust binary that manages the framework in user projects

## Project Structure

```
devtrail/
├── cli/                    # Rust CLI source code
│   ├── src/
│   │   ├── main.rs         # Entry point, command routing
│   │   ├── commands/       # Subcommands: init, update, remove, status, repair, validate, new, compliance, metrics, analyze, audit, explore, about
│   │   ├── tui/            # Terminal UI for `explore` (ratatui + crossterm)
│   │   ├── analysis_engine.rs # Code complexity analysis (arborist-metrics)
│   │   ├── config.rs       # DevTrailConfig, Checksums, ComplexityConfig
│   │   ├── download.rs     # GitHub API, ZIP downloads
│   │   ├── inject.rs       # Directive injection system
│   │   ├── manifest.rs     # dist-manifest.yml parser
│   │   ├── platform.rs     # OS/arch detection
│   │   ├── self_update.rs  # CLI auto-update
│   │   └── utils.rs        # Output helpers, file hashing
│   ├── tests/              # Integration tests
│   ├── Cargo.toml
│   └── Cargo.lock
├── dist/                   # Framework distribution files
│   ├── .devtrail/          # Templates, governance, config
│   ├── DEVTRAIL.md         # Unified governance rules
│   └── dist-manifest.yml   # What gets installed
├── docs/                   # Project documentation (EN + ES)
├── .github/workflows/      # CI/CD
│   ├── release-cli.yml     # Build + release CLI binaries
│   └── release-framework.yml
└── README.md
```

## Versioning

DevTrail uses **independent versions** for framework and CLI:

| Component | Tag format | Current | Example |
|-----------|-----------|---------|---------|
| Framework | `fw-X.Y.Z` | fw-4.1.0 | `fw-4.1.0` |
| CLI | `cli-X.Y.Z` | cli-3.0.1 | `cli-3.0.1` |

Follow [semver](https://semver.org/):
- **Major**: breaking changes
- **Minor**: new features (e.g., new command)
- **Patch**: bug fixes, small improvements

## Release Workflow — CLI

### Step 1: Bump version

Edit `cli/Cargo.toml`:
```toml
version = "X.Y.Z"
```

Run `cargo check` in `cli/` to update `Cargo.lock`.

Update version references in all docs that mention version numbers:
- `docs/adopters/CLI-REFERENCE.md` (EN — versioning table + example outputs)
- `docs/i18n/es/adopters/CLI-REFERENCE.md` (ES — same)
- `README.md` (versioning table)
- `docs/i18n/es/README.md` (ES — versioning table)

### Step 2: Commit and merge

```bash
git checkout -b chore/bump-cli-X.Y.Z
git add cli/Cargo.toml cli/Cargo.lock docs/
git commit -m "chore: bump CLI version to X.Y.Z"
# Push, create PR, merge to main
```

### Step 3: Create and push tag

```bash
git tag cli-X.Y.Z
git push origin cli-X.Y.Z
```

The `release-cli.yml` workflow triggers automatically:

1. Verifies `Cargo.toml` version matches the tag
2. Compiles for 4 platforms in parallel:
   - `x86_64-unknown-linux-gnu` (Ubuntu)
   - `x86_64-apple-darwin` (macOS Intel)
   - `aarch64-apple-darwin` (macOS ARM)
   - `x86_64-pc-windows-msvc` (Windows)
3. Packages each as `.tar.gz` (Unix) or `.zip` (Windows)
4. Creates the GitHub release and uploads all binaries

**If CI needs re-running**, trigger manually:

```bash
gh workflow run release-cli.yml -f tag=cli-X.Y.Z
```

### Step 4: Verify

```bash
gh release view cli-X.Y.Z --json assets --jq '.assets[].name'
# Should show 4 binaries
```

Users can now run `devtrail update-cli` to get the new version.

## Release Workflow — Framework

Framework releases are automated via `release-framework.yml`. The workflow triggers on tag push (`fw-*`), packages `dist/` as a ZIP, and creates the GitHub release with the asset.

### Step 1: Bump version

Edit `dist/dist-manifest.yml`:
```yaml
version: "X.Y.Z"
```

Update version references in docs:
- `docs/adopters/CLI-REFERENCE.md` (EN — versioning table)
- `docs/i18n/es/adopters/CLI-REFERENCE.md` (ES — versioning table)
- `README.md` and `docs/i18n/es/README.md` (versioning tables)
- `dist/.devtrail/00-governance/QUICK-REFERENCE.md` (EN + ES footer)
- `dist/.devtrail/00-governance/AGENT-RULES.md` (EN + ES footer)
- `dist/.devtrail/00-governance/DOCUMENTATION-POLICY.md` (EN + ES footer)
- `dist/.devtrail/00-governance/C4-DIAGRAM-GUIDE.md` (EN + ES footer)

### Step 2: Commit and merge

```bash
git checkout -b chore/bump-fw-X.Y.Z
git add dist/ docs/ README.md
git commit -m "chore: bump Framework version to X.Y.Z"
# Push, create PR, merge to main
```

### Step 3: Create and push tag

```bash
git tag fw-X.Y.Z
git push origin fw-X.Y.Z
```

The `release-framework.yml` workflow triggers automatically:
1. Verifies `dist-manifest.yml` version matches the tag
2. Packages `dist/` contents into `devtrail-fw-X.Y.Z.zip`
3. Creates the GitHub release with the ZIP as asset

**If CI needs re-running**, trigger manually:

```bash
gh workflow run release-framework.yml -f tag=fw-X.Y.Z
```

### Step 4: Verify

```bash
gh release view fw-X.Y.Z --json assets --jq '.assets[].name'
# Should show: devtrail-fw-X.Y.Z.zip
```

Users can now run `devtrail update-framework` to get the new version.

## CLI Commands Reference

| Command | Description |
|---------|-------------|
| `devtrail init [path]` | Initialize DevTrail in a project |
| `devtrail update` | Update both framework and CLI |
| `devtrail update-framework` | Update only the framework |
| `devtrail update-cli` | Update the CLI binary |
| `devtrail remove [--full]` | Remove DevTrail from project |
| `devtrail status [path]` | Show installation health and doc stats |
| `devtrail repair [path]` | Restore missing directories and framework files |
| `devtrail validate [path] [--staged]` | Validate documents for compliance and correctness |
| `devtrail new [path] [-t type] [--title]` | Create a new DevTrail document from a template |
| `devtrail compliance [path]` | Check regulatory compliance (EU AI Act, ISO 42001, NIST) |
| `devtrail metrics [path]` | Show governance metrics and documentation statistics |
| `devtrail analyze [path]` | Analyze code complexity (cognitive + cyclomatic metrics) |
| `devtrail audit [path]` | Generate audit trail reports with timeline and traceability |
| `devtrail explore [path]` | Interactive TUI documentation browser |
| `devtrail about` | Show version and license info |

## Development

### Build

```bash
cd cli
cargo build              # Debug
cargo build --release    # Release
cargo build --no-default-features  # Without TUI
```

### Test

```bash
cargo test    # All 121 tests
```

### Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `tui` | Yes | Terminal UI for `explore` (ratatui + crossterm + pulldown-cmark) |

### Key Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `colored` | Terminal colors |
| `ratatui` | TUI framework (optional) |
| `crossterm` | Terminal backend (optional) |
| `pulldown-cmark` | Markdown parser (optional) |
| `reqwest` | HTTP client for downloads |
| `serde_yaml` | YAML parsing |
| `chrono` | Date parsing (metrics, audit) |
| `anyhow` | Error handling |
