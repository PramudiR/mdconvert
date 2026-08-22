# mdconvert — Markdown to HTML (Rust)

> A small, fast CLI to convert Markdown files to HTML with optional stylesheet support.

## Summary

| Item | Details |
|---|---|
| **Purpose** | A small, fast CLI to convert Markdown files to HTML with optional stylesheet support. |
| **Language** | Rust |
| **Where to look** | Source is in `main.rs` and configuration/dependencies in `Cargo.toml`. |

## Features

**Convert**  
Markdown -> HTML output.

**Styling**  
Includes example styles in the `assets` folder.

**Small & fast**  
Built with Rust for performance and portability.

## Quick Start

### Build

```sh
cargo build --release
```

### Run in development

```sh
cargo run -- input.md --output output.html
```

### Run the release binary

```sh
../target/release/mdconvert input.md --output output.html
```

## Usage

### Basic conversion

```sh
cargo run -- `README.md`
```

### Convert to a specific HTML file

```sh
cargo run -- input.md --output output.html
```

### Use the light theme

```sh
cargo run -- input.md --output output.html --theme light
```

### Use the dark theme

The dark theme is the default:

```sh
cargo run -- input.md --output output.html --theme dark
```

```sh
cargo run -- inpput.md --output output.html
```

### Use short options

```sh
cargo run -- input.md -o output.html -t light
```

## Configuration & Files

| Component | Location or details |
|---|---|
| **Binary entry** | See `main.rs` for CLI flags and behavior. |
| **Dependencies** | Defined in `Cargo.toml`. |
| **Styles** | Default CSS themes live in `assets` (e.g., `github-dark.css`, `minimal-light.css`). |

## Development

| Task | Command or instructions |
|---|---|
| **Format** | `cargo fmt` |
| **Build** | `cargo build` |
| **Customize** | Add style themes into the `assets` folder, edit `main.rs`, then rebuild. |

## Contributing

To contribute, open issues or PRs with feature requests, bug reports, or small improvements.

Keep changes focused and add tests where appropriate.

## License

mdconvert  Copyright (C) 2026 PramudiR

This program comes with ABSOLUTELY NO WARRANTY; for details refer to section w.

This is free software, and you are welcome to redistribute it under certain conditions; for details refer section c.

For more detail refer to [LICENSE](https://github.com/PramudiR/mdconvert/blob/master/LICENSE).
