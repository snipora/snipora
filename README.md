# snipora

A lightweight, local-first snippet manager for fast capture, simple organization, and quick search.

[Website](https://snipora.github.io) · [Changelog](CHANGELOG.md) · [Contributing](CONTRIBUTING.md) · [License](LICENSE)

## Tech Stack

- **Frontend**: Vue 3, TypeScript, Tailwind CSS, shadcn-vue
- **Backend**: Rust, Tauri 2, sqlx (SQLite)
- **Tooling**: Vite, just, commitlint, husky, vue-tsc

## Getting Started

Requirements:
- [Rust](https://rustup.rs) (stable)
- [Node.js](https://nodejs.org) (LTS)
- [just](https://github.com/casey/just) (command runner)
- System libraries for [Tauri](https://v2.tauri.app/start/prerequisites/)

```sh
npm install
just dev
```

Available `just` recipes are documented in the [justfile](justfile) and can be browsed interactively with `just --choose`.

## Project Structure

```
.
├── src/                          # Vue 3 frontend
│   ├── api/                      #   Tauri command wrappers and DTOs
│   ├── components/               #   Shared UI components
│   ├── composables/              #   Vue composables
│   ├── lib/                      #   Utility functions
│   ├── locales/                  #   Frontend i18n
│   ├── main/                     #   Main window
│   └── popup/                    #   Popup window
├── src-tauri/                    # Rust backend (Tauri)
│   ├── src/                      #   Rust source
│   ├── migrations/               #   Database migrations
│   ├── locales/                  #   Backend i18n
│   └── capabilities/             #   Tauri capability files
├── scripts/                      # Build and utility scripts
├── public/                       # Static frontend assets
└── assets/                       # App icon source
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development workflow, commit conventions, and pull request guidelines.
