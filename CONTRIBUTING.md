# Contributing to git-mob

Thank you for considering contributing to git-mob! This document provides guidelines and information for contributors.

Not all pull requests or suggestions will be accepted, and maintainers reserve the right to decline changes for any reason, including technical direction, maintainability, or other considerations.
By participating in this project, you agree to abide by our Code of Conduct.

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

- Use the GitHub issue tracker to report bugs
- The bug report template will guide you through providing the necessary details

### Suggesting Features

- Use the GitHub issue tracker to suggest new features
- The feature request template will help you structure your proposal

### Pull Requests

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes following the guidelines below
4. Test your changes thoroughly
5. Ensure all CI checks pass
6. Maintain or improve code coverage - new code should be well-tested
7. Submit a pull request with a clear description

## Development Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Git](https://git-scm.com/downloads) v2.32 or later
- On Windows: [Microsoft C++ Build Tools](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup#install-visual-studio-recommended-or-the-microsoft-c-build-tools)

### Getting Started

1. Clone the repository:

   ```bash
   git clone https://github.com/Mubashwer/git-mob.git
   cd git-mob
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run tests:

   ```bash
   cargo test
   ```

4. Run the tool locally:
   ```bash
   cargo run -- --help
   ```

### IDE Setup

#### VS Code (Recommended)

1. Install the official [Rust extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
2. Configure format on save for Rust files by adding to your settings.json:

   ```json
   {
     "[rust]": {
       "editor.formatOnSave": true
     }
   }
   ```

   The rust-analyzer extension will automatically use your project's `rustfmt.toml` configuration if present.

#### Other IDEs

- **RustRover**: Use JetBrains' dedicated [RustRover IDE](https://www.jetbrains.com/rust/)

## Development Guidelines

### Code Style

- **Formatting**: Use `cargo fmt --all` to format your code
- **Linting**: Run `cargo clippy --all-targets -- -D warnings` to check for lints
- **Documentation**: Add doc comments for public APIs using `///`
- **Testing**: Write tests for new functionality and bug fixes
- **Backwards Compatibility**: Strive to maintain backwards compatibility whenever possible. If breaking changes are necessary, clearly document them and provide migration guidance

### Commit Messages

This project uses [Conventional Commits](https://www.conventionalcommits.org/). Format your commit messages as:

```
<type>[optional scope]: <description>
```

**Common types:** `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

**Examples:**

```
feat: add support for adding non-team member to mob session

fix: resolve clippy warnings in test files

docs: update contributing guidelines

chore: update CI to use more comprehensive clippy checks
```

### Testing

- Write unit tests for new functionality
- Integration tests should be placed in the `tests/` directory
- Use descriptive test names that explain what is being tested
- Test both success and failure scenarios
- Run the full test suite before submitting: `cargo test`

### Documentation

- Update the README.md if you add new features or change existing behavior
- Add inline documentation for complex logic
- Update help text and command documentation as needed

## Project Structure

```
├── src/
│   ├── lib.rs           # Library root
│   ├── main.rs          # CLI entry point
│   ├── cli.rs           # Command line interface
│   ├── helpers.rs       # Utility functions
│   ├── commands/        # Command implementations
│   └── repositories/    # Data access layer
├── tests/               # Integration tests
├── docs/                # Documentation
└── target/              # Build artifacts
```

## Code Quality

### Before Submitting

Run these commands to ensure your code meets the project standards:

```bash
# Format code
cargo fmt --all

# Check for lints
cargo clippy --all-targets -- -D warnings

# Run tests
cargo test

# Check documentation
cargo doc --no-deps --open
```

### Continuous Integration

The CI pipeline runs:

- Code formatting checks
- Clippy linting with all warnings as errors
- Tests on multiple platforms (Linux, macOS, Windows)
- Security audit
- Code coverage reporting

## Release Process

Releases are automated using [release-please](https://github.com/googleapis/release-please):

1. **Automatic Release PRs**: release-please creates and maintains a release PR based on conventional commits
2. **Version Bumping**: Versions are automatically determined from commit types (feat = minor, fix = patch, breaking = major)
3. **Changelog Generation**: `CHANGELOG.md` is automatically updated based on conventional commits
4. **Release Creation**: When the release PR is merged:
   - Version is bumped in `Cargo.toml`
   - Git tag is created
   - GitHub release is published
   - Crate is published to crates.io

**For Contributors**: Simply follow the conventional commit format - the release process handles the rest automatically!

## Getting Help

- Check existing issues and discussions
- Ask questions in GitHub discussions
- Join the project's community channels if available

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
