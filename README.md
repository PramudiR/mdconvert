# ⚡ mdconvert

> **Runs in under 5ms, zero-dependency Markdown to styled HTML CLI converter written in Rust.**

[![Release](https://img.shields.io/github/v/release/PramudiR/mdconvert?style=flat-square&color=88c0d0)](https://github.com/PramudiR/mdconvert/releases)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg?style=flat-square)](https://www.gnu.org/licenses/gpl-3.0)
[![Crates.io](https://img.shields.io/crates/v/mdconvert?style=flat-square&color=orange)](https://crates.io/crates/mdconvert)

Convert your Markdown files into single-file, self-contained HTML documents in **under 5 milliseconds**. Features compile-time embedded CSS themes, high-contrast `@media print` rules for browser PDF export, and zero external runtime dependencies.

---

## ✨ Features

- ⚡ **Blazing Fast:** Cold-boots and renders in `<5ms` powered by Rust and `pulldown-cmark`.
- 📦 **Zero-Dependency Portable HTML:** Embeds stylesheets directly into output `.html` files for 100% offline portability.
- 🎨 **Built-In Themes:** Embedded styles (`github-dark`, `minimal-light`, `nord`, `solarized-light`, `gruvbox`) compiled straight into the single binary.
- 🖨️ **PDF Print-Ready:** Includes custom `@media print` rules out of the box. Open the HTML in any browser and press `Cmd+P` / `Ctrl+P` for crisp, paper-optimized PDFs.

---

## 🚀 Installation

### Option 1: Via Cargo (Recommended for Rust users)

```bash
cargo install mdconvert
```

### Option 2: Download Pre-Built Binary (Recommended)

Download the latest pre-compiled binary for macOS, Linux, or Windows from the **[Releases Page](https://github.com/PramudiR/mdconvert/releases)**.

### Option 3: Build from Source

If you have the Rust toolchain installed:

```bash
cargo install --git [https://github.com/PramudiR/mdconvert](https://github.com/PramudiR/mdconvert)
```

Or build locally:

```bash
git clone [https://github.com/PramudiR/mdconvert.git](https://github.com/PramudiR/mdconvert.git)
cd mdconvert
cargo build --release
# Binary will be at ./target/release/mdconvert
```

## 💻 Usage

### Basic conversion (defaults to github-dark theme -> input.html)

```bash
mdconvert README.md
```

### Convert with custom output path and theme

```bash
mdconvert input.md --output docs.html --theme light
```

### Short options

```bash
mdconvert input.md -o output.html -t nord
```

## Available Themes (`-t, --theme`)

|Theme Name|Style Description|
|---|---|
|`dark` _(Default)_|Classic GitHub dark mode|
|`light`|Clean, high-contrast minimal light typography|
|`nord`|Arctic, pastel-blue dark theme|
|`solarized-light`|Low-contrast cream & teal palette for long reading sessions|
|`gruvbox`|Warm retro-dark theme popular among Vim/terminal users|

## 📄 Converting HTML to PDF

`mdconvert` prioritizes tiny binary size and high execution speed over heavy browser bundling. To export to PDF:

1. Convert your markdown: `mdconvert README.md -t light`
2. Open `README.html` in your web browser (Chrome, Safari, Firefox).
3. Press `Cmd + P` (macOS) or `Ctrl + P` (Windows/Linux) and select **Save as PDF**.

## 🤝 Contributing

Contributions, feature requests, and new CSS themes are welcome! Feel free to check the [issues page](https://github.com/PramudiR/mdconvert/issues).

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'feat: Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📜 License

Distributed under the **GNU General Public License v3.0** (GPL-3.0).

```text
mdconvert - Sub-millisecond, zero-dependency Markdown to styled HTML CLI
Copyright (C) 2026 PramudiR

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.
```
