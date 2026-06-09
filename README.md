# Snipora

A lightweight, local-first snippet manager for fast capture, simple organization, and quick search.

[![Release](https://img.shields.io/github/v/release/snipora/snipora)](https://github.com/snipora/snipora/releases/latest)
&nbsp;
[![Website](https://img.shields.io/badge/website-blue)](https://snipora.github.io)
&nbsp;
[![Changelog](https://img.shields.io/badge/changelog-orange)](CHANGELOG.md)
&nbsp;
[![Contributing](https://img.shields.io/badge/contributing-green)](CONTRIBUTING.md)
&nbsp;
[![License](https://img.shields.io/github/license/snipora/snipora)](LICENSE)

> **For end users:** Visit [snipora.github.io](https://snipora.github.io) to download and learn about Snipora. This repository is intended for developers and contributors.

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

## Star History

<a href="https://www.star-history.com/?repos=snipora%2Fsnipora&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=snipora/snipora&type=date&theme=dark&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=snipora/snipora&type=date&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=snipora/snipora&type=date&legend=top-left" />
 </picture>
</a>
