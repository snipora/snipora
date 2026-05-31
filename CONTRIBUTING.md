# Contributing to Snipora

Thanks for your interest in contributing.

## Getting Started

1. Fork the repository
2. Follow the [development setup](README.md#getting-started) in the README
3. Create a branch: `git checkout -b feat/description`

### Branch naming

Using a `type/description` format (matching [conventional commits](https://www.conventionalcommits.org/)) is recommended but not enforced. The commit history is what matters, and those are validated by commitlint. Feel free to use any branch naming scheme that works for you.

## Development

Run the app in development mode:

```sh
just dev
```

Available `just` recipes are documented in the [justfile](justfile) and can be browsed interactively with:

```sh
just --choose
```

## Commit Conventions

This project uses [conventional commits](https://www.conventionalcommits.org/) enforced via commitlint.

Examples:

```
feat: add setting autoCheckForUpdates
fix: resolved toast height problem on hover
refactor: re-structured settings components directory
```

Allowed types: `feat`, `fix`, `refactor`, `style`, `test`, `docs`, `ci`, `perf`, `chore`.

## Pull Request Process

- Keep PRs focused on a single concern
- Checks (`check-types`, `check-i18n`) are enforced via git hooks; make sure they pass before pushing
- Write a clear PR description linking to related issues

## Reporting Issues

- **Bug reports**: include steps to reproduce, expected vs actual behavior, and environment details (OS, app version)
- **Feature requests**: describe the problem you're trying to solve and any alternatives considered
