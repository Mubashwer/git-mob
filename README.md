# `git mob`

> **CLI tool to help you automatically add Co-authored-by trailers to git commits during pair/mob programming**

[![CI status](https://github.com/mubashwer/git-mob/actions/workflows/ci.yml/badge.svg)](https://github.com/Mubashwer/git-mob/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/Mubashwer/git-mob/branch/main/graph/badge.svg?token=R522R7NZH4)](https://codecov.io/gh/Mubashwer/git-mob)
[![CodeQL](https://github.com/Mubashwer/git-mob/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/Mubashwer/git-mob/actions/workflows/github-code-scanning/codeql)
[![crate](https://img.shields.io/crates/v/git-mob-tool?style=round)](https://crates.io/crates/git-mob-tool)
[![crate downloads](https://img.shields.io/crates/d/git-mob-tool?style=round)](https://crates.io/crates/git-mob-tool)
[![github release](https://img.shields.io/github/v/release/mubashwer/git-mob?style=round)](https://github.com/Mubashwer/git-mob/releases/latest)
[![github release downloads](https://img.shields.io/github/downloads/mubashwer/git-mob/total?style=round)](https://github.com/Mubashwer/git-mob/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

You can attribute a git commit to more than one author by adding one or more `Co-authored-by` trailers to the commit's message. Co-authored commits are visible on GitHub. For more information, see [here](https://docs.github.com/en/pull-requests/committing-changes-to-your-project/creating-and-editing-commits/creating-a-commit-with-multiple-authors).

This CLI tool will help you add them automatically and also help you store and manage co-authors for pair/mob programming sessions.

## Features

- Cross-platform
- Co-authors management via CLI
- Multi-select menu to choose co-author(s) for mobbing
- Automatic appending of `Co-authored-by` trailers to commit
- No need of any git aliases

## Installation

### Prerequisites

- [Git v2.32](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) or later

### With Binary Package

- Download and extract the appropriate binary depending on your OS from the [latest GitHub release](https://github.com/Mubashwer/git-mob/releases/latest)
- Ensure the binary is in your `PATH` variable e.g. you may place the file in `C:\Windows` on Windows or `/usr/local/bin` on Linux and macOS
- Ensure the binary is set as executable (Linux and macOS)

  ```console
  $ chmod +x ./git-mob
  ```

### With [Cargo](https://crates.io/crates/git-mob-tool)

- Install [Rust](https://www.rust-lang.org/tools/install)
- If you are using Windows, you will also need to [install certain C++ build tools](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup#install-visual-studio-recommended-or-the-microsoft-c-build-tools)

```console
$ cargo install git-mob-tool
```

## Setup & Configuration

- Set up a global [`prepare-commit-msg`](https://git-scm.com/docs/githooks#_prepare_commit_msg) githook which appends the `Co-authored-by` trailers to the commit message.

  ```console
  $ git mob setup
  ```

  If a repository overrides `core.hooksPath` git configuration variable (e.g when using husky), then you will additionally need to run `git mob setup --local` for each such repository. This will set up a local (repository-specific) `prepare-commit-msg` githook which invokes the global one.

  _If you prefer to set this up manually or encounter any issues with the automated setup process, you can follow steps outlined [here.](./docs/manual_setup.md)_

- Store your team members' details with keys

  ```console
  $ git mob team-member --add lm "Leo Messi" leo.messi@example.com
  $ git mob team-member --add em "Emi Martinez" emi.martinez@example.com
  $ git mob team-member --add sa "Sergio Aguero" sergio.aguero@example.com
  ```

## Usage

- To mob with some team member(s):

  ```console
  $ git mob --with
  ? Select active co-author(s):
  > [ ] Leo Messi <leo.messi@example.com>
    [ ] Emi Martinez <emi.martinez@example.com>
    [ ] Sergio Aguero <sergio.aguero@example.com>
  [↑↓ to move, space to select one, → to all, ← to none, type to filter ]
  ```

  Alternatively, if you remember the team member keys, you may bypass the multi-select menu by running:

  ```console
  $ git mob --with lm em
  ```

  This will start a global mob session. Any git commit made afterwards will have `Co-authored-by` trailers added to the commit message as shown below:

  ```text
  This is an example commit message

  Co-authored-by: Leo Messi <leo.messi@example.com>
  Co-authored-by: Emi Martinez <emi.martinez@example.com>
  ```

- To add a non-team member to the mob session:

  ```console
  $ git mob --add "Diego Maradona" diego.maradona@example.com
  ```

- To clear the mob session:

  ```console
  $ git mob --clear
  ```

- To view the co-authors in the mob session:

  ```console
  $ git mob --list
  ```

- To print help information:

  ```console
  $ git mob help
  $ git mob help team-member
  ```

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Git](https://git-scm.com/downloads) v2.32 or later
- On Windows: [Microsoft C++ Build Tools](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup#install-visual-studio-recommended-or-the-microsoft-c-build-tools)

### Building from Source

```console
$ git clone https://github.com/Mubashwer/git-mob.git
$ cd git-mob
$ cargo build --release
```

### Running Tests

```console
$ cargo test
```

### Code Quality

Format code:

```console
$ cargo fmt --all
```

Run linting:

```console
$ cargo clippy --all-targets -- -D warnings
```

### Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details on:

- Setting up your development environment
- Code style and conventions
- Commit message format (Conventional Commits)
- Testing guidelines
- Pull request process

## Troubleshooting

- When using `git mob --help`, an error may occur because Git looks for man pages for subcommands. To avoid this error, use one of the following alternatives:
  - `git mob help`
  - `git-mob --help`
