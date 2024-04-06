# Manual Setup

- Set a global [`githooks`](https://git-scm.com/docs/githooks) directory

  ```console
  $ mkdir -p ~/.git/hooks
  $ git config --global core.hooksPath "~/.git/hooks"
  ```

- Download the [`prepare-commit-msg`](../src/commands/prepare-commit-msg) file into the directory
- Ensure it is set as executable (Linux and macOS)

   ```console
  $ chmod +x ./prepare-commit-msg
  ```

  This githook will append the `Co-authored-by` trailers to the commit message.

  _If you want this githook to add a Jira Issue ID as a prefix to the commit message when the git branch name begins with a string resembling one, uncomment [line 12 to call the function `add_jira_issue_id_prefix`](../src/commands/prepare-commit-msg#LL12)._

## If a repository overrides `core.hooksPath` git configuration variable (e.g when using husky), then you will need to do additional steps for each such repository

- Retrieve the local (repository-specific) hooks directory

  ```console
  $ git config --local core.hooksPath
  ```

- Download the [`prepare-commit-msg.local`](../src/commands/prepare-commit-msg.local) as `prepare-commit-msg` file into the directory
- Ensure it is set as executable (Linux and macOS)

   ```console
  $ chmod +x ./prepare-commit-msg
  ```

  This githook will invoke the global `prepare-commit-msg` githook that you originally set up.
