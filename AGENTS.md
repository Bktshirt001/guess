# AGENTS.md

## Project snapshot
- This repository is a small Rust CLI crate named `guess`.
- The package metadata lives in [Cargo.toml](Cargo.toml) and the main program entrypoint is [src/main.rs](src/main.rs).
- The current implementation is intentionally minimal: it reads a line from stdin and prints it back.

## Working conventions
- Use Rust 2024 edition idioms and keep changes small and focused.
- Prefer the simplest solution that matches the existing CLI style.
- Preserve the current Chinese user-facing strings unless the task explicitly asks for a wording change.

## Verification commands
- Run `cargo run` for a quick manual smoke check.
- Run `cargo test` after logic changes or when adding tests.
- If a dependency is added or changed, update [Cargo.toml](Cargo.toml) directly and keep the change minimal.

## Editing guidance
- Keep feature work in [src/main.rs](src/main.rs) unless the change clearly benefits from a new module or helper.
- Favor readable, direct Rust over unnecessary abstraction.
- When adding tests, prefer small unit or integration tests that exercise real behavior rather than mock-heavy setup.
