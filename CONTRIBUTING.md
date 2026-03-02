# Contributing to DevTrail

Thank you for your interest in contributing to DevTrail! This document provides guidelines and information for contributors.

**Languages**: English | [Español](docs/i18n/es/CONTRIBUTING.md)

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Style Guidelines](#style-guidelines)
- [Documentation Standards](#documentation-standards)

---

## Code of Conduct

This project adheres to a Code of Conduct that all contributors are expected to follow. Please be respectful, inclusive, and constructive in all interactions.

### Our Standards

- Be welcoming and inclusive
- Be respectful of differing viewpoints
- Accept constructive criticism gracefully
- Focus on what is best for the community
- Show empathy towards other community members

---

## How Can I Contribute?

### Reporting Bugs

Before creating a bug report, please check existing issues to avoid duplicates.

**When reporting a bug, include:**

- A clear, descriptive title
- Steps to reproduce the behavior
- Expected behavior
- Actual behavior
- Screenshots (if applicable)
- Environment details (OS, AI platform, etc.)

### Suggesting Features

Feature suggestions are welcome! Please include:

- A clear description of the feature
- The problem it solves
- Potential implementation approaches
- Any alternatives you've considered

### Improving Documentation

Documentation improvements are highly valued:

- Fix typos or unclear wording
- Add examples
- Improve explanations
- Translate to other languages

### Submitting Code

Code contributions should:

- Fix a bug or implement a feature
- Include appropriate tests (if applicable)
- Follow the project's style guidelines
- Update documentation as needed

---

## Development Setup

### Prerequisites

- Git
- A text editor (VS Code recommended)
- Bash (for Linux/Mac validation scripts)
- PowerShell (for Windows validation scripts)
- Node.js (optional, for markdownlint)

### Setup Steps

1. **Fork the repository**

   Click "Fork" on the GitHub repository page.

2. **Clone your fork**
   ```bash
   git clone https://github.com/your-username/devtrail.git
   cd devtrail-framework
   ```

3. **Create a branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

4. **Install development tools (optional)**
   ```bash
   npm install -g markdownlint-cli
   ```

5. **Make your changes**

6. **Validate your changes**
   ```bash
   # Linux/Mac
   bash scripts/pre-commit-docs.sh

   # Windows
   .\scripts\validate-docs.ps1
   ```

---

## Pull Request Process

### Before Submitting

- [ ] Run validation scripts successfully
- [ ] Update documentation if needed
- [ ] Add yourself to CONTRIBUTORS.md (if applicable)
- [ ] Write a clear PR description

### PR Title Format

Use conventional commit format:

```
type(scope): description

Examples:
feat(templates): add template for security reviews
fix(validation): correct regex for file naming
docs(readme): clarify installation steps
chore(ci): update GitHub Actions workflow
```

**Types:**
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `chore` - Maintenance tasks
- `refactor` - Code refactoring
- `test` - Test additions or fixes

### PR Description Template

```markdown
## Summary
Brief description of changes

## Motivation
Why is this change needed?

## Changes
- Change 1
- Change 2

## Testing
How were these changes tested?

## Checklist
- [ ] Validation scripts pass
- [ ] Documentation updated
- [ ] No sensitive information included
```

### Review Process

1. A maintainer will review your PR
2. Address any requested changes
3. Once approved, a maintainer will merge

---

## Style Guidelines

### Markdown

- Use ATX-style headers (`#`, `##`, etc.)
- Use fenced code blocks with language identifiers
- Use tables for structured data
- Keep lines under 120 characters when practical
- Use blank lines to separate sections

### YAML Front-matter

```yaml
---
id: TYPE-YYYY-MM-DD-NNN
title: Clear, descriptive title
status: draft | accepted | deprecated
created: YYYY-MM-DD
# Additional fields as needed
---
```

### File Naming

DevTrail documents:
```
[TYPE]-[YYYY-MM-DD]-[NNN]-[description].md
```

- Use lowercase for description
- Use hyphens to separate words
- Keep descriptions concise but clear

### Code in Scripts

- Use clear variable names
- Add comments for complex logic
- Follow shell/PowerShell best practices

---

## Documentation Standards

### Adding a New Document Type

If you're proposing a new document type:

1. **Create the template**
   - Add `TEMPLATE-NEWTYPE.md` to `.devtrail/templates/`
   - Follow existing template patterns

2. **Update governance docs**
   - `.devtrail/00-governance/DOCUMENTATION-POLICY.md`
   - `.devtrail/00-governance/AGENT-RULES.md`
   - `.devtrail/QUICK-REFERENCE.md`

3. **Update agent configs**
   - `CLAUDE.md`
   - `GEMINI.md`
   - `.cursorrules`
   - `.github/copilot-instructions.md`

4. **Update validation**
   - `scripts/pre-commit-docs.sh`
   - `scripts/validate-docs.ps1`
   - `.github/workflows/docs-validation.yml`

5. **Document the change**
   - Create an ADR explaining the new type
   - Update the README if needed

### Writing Templates

Templates should include:

- Complete YAML front-matter with all fields
- Clear section headers
- Placeholder text explaining what goes in each section
- Examples where helpful

### Writing Governance Documents

- Be clear and unambiguous
- Use tables for reference information
- Include examples
- Keep rules actionable

---

## CLI Development

The DevTrail CLI is written in Rust and located in the `cli/` directory.

### Building

```bash
cd cli
cargo build
```

### Running Tests

```bash
cd cli
cargo test
```

### Release Build

```bash
cd cli
cargo build --release
```

The release binary is optimized with LTO and stripped for minimal size.

### Architecture

```
cli/src/
├── main.rs          # Entry point + clap CLI definition
├── commands/
│   ├── init.rs      # devtrail init [path]
│   ├── update.rs    # devtrail update
│   └── remove.rs    # devtrail remove [--full]
├── config.rs        # Config and checksums management
├── download.rs      # GitHub Releases download
├── inject.rs        # Directive file injection (markers)
├── manifest.rs      # dist-manifest.yml parsing
└── utils.rs         # Helpers (hashing, colors, paths)
```

---

## Questions?

If you have questions about contributing:

1. Check existing [Issues](https://github.com/StrangeDaysTech/devtrail/issues)
2. Check [Discussions](https://github.com/StrangeDaysTech/devtrail/discussions)
3. Open a new Discussion for general questions
4. Open an Issue for specific bugs or features

---

## Recognition

Contributors are recognized in:

- GitHub's contributor graph
- Release notes for significant contributions
- CONTRIBUTORS.md (for recurring contributors)

Thank you for helping make DevTrail better!

---

*DevTrail — Because every change tells a story.*

[Strange Days Tech](https://strangedays.tech)
