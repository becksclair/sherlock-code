# Sherlock Code

Generate code reviews from a Github pull request url.
Very much a work in progress. But usable as it is.

TODO:

- [ ] Add monaco editor for code review
- [ ] Add cli option to run without GUI
- [ ] Add sqlite for storage
- [ ] Adding configuration options and UI

## Dependencies

The only dependency at the moment is the `Github` cli tool, and to be logged
in to access private repository pull requests.

## Configuration

Currently there's no configuration, so the program will use your OpenAI key from
the environment variable `OPENAI_KEY`.

## Recommended IDE Setup

[NeoVim](https://neovim.io/) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
