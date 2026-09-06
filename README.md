# Base | Text Editor <img width="25" height="25" alt="image" src="https://github.com/user-attachments/assets/50a45d36-41c0-4296-bb84-eeca03afa788"/>


## Introduction

**Base** is a lightweight, terminal-based text editor built from scratch. It's designed to be fast, minimal, and configurable through Lua scripting, letting you customize commands and behavior to fit your workflow.

> Note: This project is still early in development — documentation and features are actively being added.

## Languages & Technologies

- **Rust** — core editor logic and terminal rendering
- **Lua** — configuration and custom commands (`commands.lua`)

### Key dependencies
- [`crossterm`](https://crates.io/crates/crossterm) — cross-platform terminal manipulation
- [`mlua`](https://crates.io/crates/mlua) (Lua 5.4, vendored) — embedded Lua scripting engine

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition or later) with `cargo`
- A C compiler (e.g. `gcc` or `clang`) — required to build the vendored Lua 5.4 runtime
- A terminal emulator that supports standard ANSI escape sequences

Lua itself does **not** need to be installed separately — it's compiled and bundled automatically via the `mlua` vendored feature.

## How to Run

1. Clone the repository:
   ```bash
   git clone https://github.com/realv1sta/base.git
   cd base
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the editor:
   ```bash
   cargo run --release
   ```

   Or, once built, run the compiled binary directly:
   ```bash
   ./target/release/base
   ```

4. (Optional) Customize your commands and keybindings by editing `commands.lua`.

## Contributing

Contributions, issues, and feature requests are welcome. Feel free to check the [issues page](https://github.com/realv1sta/base/issues) if you'd like to help out.
