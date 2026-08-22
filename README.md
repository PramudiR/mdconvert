
# mdconvert — Markdown to HTML (Rust)

## Summary

- **Purpose**: A small, fast CLI to convert Markdown files to HTML with optional stylesheet support.
- **Language**: Rust
- **Where to look**: Source is in main.rs and configuration/dependencies in Cargo.toml.

## Features

- **Convert**: Markdown -> HTML output.
- **Styling**: Includes example styles in the assets folder.
- **Small & fast**: Built with Rust for performance and portability.

## Quick Start

- **Build**: `cargo build --release`
- **Run (dev)**: `cargo run -- <input.md> --output <output.html>`
- **Run (release)**: `../target/release/mdconvert <input.md> --output <output.html>`

## Usage (examples)

- **Basic**: `cargo run -- README.md`
- **To a specific html file**: `cargo run -- input.md --output output.html`
- **With stylesheet light theme**: `cargo run -- input.md --output output.html --theme light`
- **with stylesheet dark theme (default)**: `cargo run -- input.md --output output.html --theme dark` or `cargo run -- inpput.md --output output.html`
- **with short options**: `cargo run -- input.md -o output.html -t light`

## Configuration & Files

- **Binary entry**: See main.rs for CLI flags and behavior.
- **Dependencies**: Defined in Cargo.toml.
- **Styles**: Default CSS themes live in assets (e.g., github-dark.css, minimal-light.css).

## Development

- **Format**: `cargo fmt`
- **Build**: `cargo build`
- **Customize**: Add style themes into assets folder, Edit main.rs, then rebuild.

## Contributing

- **How to help**: Open issues or PRs with feature requests, bug reports, or small improvements.
- **Style**: Keep changes focused and add tests where appropriate.

## License

mdconvert  Copyright (C) 2026 PramudiR
This program comes with ABSOLUTELY NO WARRANTY; for details refer to section w.
This is free software, and you are welcome to redistribute it
under certain conditions; for details refer section c.

- For more detail refer to [LICENSE](https://github.com/PramudiR/mdconvert/blob/master/LICENSE)
