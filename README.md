# Finance App

Simple personal finance tracker. Standalone application developed using svelte & tauri.
Database is implemented with SQLite.

## Setup

Check guide on Tauri website [GUIDE](https://tauri.app/v1/guides/getting-started/prerequisites)

### Run setup script

`./scripts/init`

### Or alternatively run commands manually

Install the pre-commit hook `cp scripts/pre-commit .git/hooks/`.
Install the required cargo packages: `npm run cargo install cargo-tarpaulin cargo-watch`
Initialize the project with `npm run tauri dev`

### Demonstration

Simple demonstration of most features in the application.

![Demonstration video](./docs/demo.gif)
