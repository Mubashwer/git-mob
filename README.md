# `git mob`

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
- [Rust and Cargo](https://www.rust-lang.org/tools/install)

```sh
$ cargo install git-mob
```

## Configuration

- Store your team members' details with keys (usually initials)

  ```sh
  $ git mob coauthor -a lm "Leo Messi" leo.messi@example.com
  $ git mob coauthor -a em "Emi Martinez" emi.martinez@example.com
  $ git mob coauthor -a sa "Sergio Aguero" sergio.aguero@example.com
  ...
  ```

- Set a global hooks directory

  ```sh
  $ mkdir ~/git
  $ git config --global core.hooksPath ~/git
  ```

- Copy [`prepare-commit-msg`](./prepare-commit-msg) into the directory

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

  ```sh
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

### Troubleshooting

- Try installing the latest version of git
