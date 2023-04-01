# `git mob`

> **CLI app to help you automatically add Co-authored-by trailers to git commits during pair/mob programming**

[![Crates.io](https://img.shields.io/crates/v/git-mob-tool?style=round)](https://crates.io/crates/git-mob-tool)
[![Crates.io](https://img.shields.io/crates/d/git-mob-tool?style=round)](https://crates.io/crates/git-mob-tool)
[![License](https://img.shields.io/badge/license-MIT-blue?style=round)](LICENSE-MIT)
[![Build Status](https://github.com/mubashwer/git-mob/actions/workflows/build.yml/badge.svg)](https://github.com/Mubashwer/git-mob/actions/workflows/build.yml)

You can attribute a git commit to more than one author by adding one or more `Co-authored-by` trailers to the commit's message. Co-authored commits are visible on GitHub. For more information, see [here](https://docs.github.com/en/pull-requests/committing-changes-to-your-project/creating-and-editing-commits/creating-a-commit-with-multiple-authors).

This CLI app will help you add them automatically and also help you store and manage co-authors for pair/mob programming sessions.

## Features

- Cross-platform
- Co-authors management via CLI
- Multi-select menu to choose co-author(s) for mobbing
- Automatic appending of `Co-authored-by` trailers to commit
- No need of any git aliases

## Installation

### Prerequisites
- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

### With Binary Package:

- Download and extract the appropriate binary depending on your OS from the [latest GitHub release](https://github.com/Mubashwer/git-mob/releases/latest)
- Ensure the binary is in your `PATH` variable


### With [Cargo](https://crates.io/crates/git-mob-tool):

- Install [Rust and Cargo](https://www.rust-lang.org/tools/install)
- If you are using Windows, you will also need to [install certain C++ build tools](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup#install-visual-studio-recommended-or-the-microsoft-c-build-tools)

```sh
$ cargo install git-mob-tool
```

## Configuration

- Store your team members' details with keys (usually initials)

  ```sh
  $ git mob coauthor -a lm "Leo Messi" leo.messi@example.com
  $ git mob coauthor -a em "Emi Martinez" emi.martinez@example.com
  $ git mob coauthor -a sa "Sergio Aguero" sergio.aguero@example.com
  ...
  ```

- Set a global [`githooks`](https://git-scm.com/docs/githooks) directory

  ```sh
  $ mkdir ~/git
  $ git config --global core.hooksPath "~/git"
  ```

- Copy [`prepare-commit-msg`](./prepare-commit-msg) into the directory. Ensure it is set as executable.

  This [`githook`](https://git-scm.com/docs/githooks#_prepare_commit_msg) will be used to append the `Co-authored-by` trailers to the commit's message.

## Usage

- To mob with some team member(s):

  ```
  $ git mob --with
  ? Select active co-authors(s):
  > [ ] Leo Messi <leo.messi@example.com>
    [ ] Emi Martinez <emi.martinez@example.com>
    [ ] Sergio Aguero <sergio.aguero@example.com>
  [↑↓ to move, space to select one, → to all, ← to none, type to filter ]
  ```

  Alternatively, if you remember the co-author keys, you may bypass the multi-select menu by running:

  ```sh
  $ git mob --with lm em
  ```

  This will start a global mob session. Any git commit made afterwards will have `Co-authored-by` trailers added to the commit message as shown below:

  ```
  This is an example commit message

  Co-authored-by: Leo Messi <leo.messi@example.com>
  Co-authored-by: Emi Martinez <emi.martinez@example.com>
  ```

- To clear the mob session:

  ```sh
  $ git mob --clear
  ```

- To view the co-authors in the current mob session:

  ```sh
  $ git mob --list
  ```

- To print help information:
  ```sh
  $ git mob --help
  $ git mob coauthor --help
  ```

## Troubleshooting

- Try installing the latest version of git
