use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use pulldown_cmark::{html, Options, Parser as MdParser};
use std::fs;
use std::path::PathBuf;

// Compile time embedded CSS assets
const THEME_GITHUB_DARK: &str = include_str!("../assets/github-dark.css");
const THEME_MINIMAL_LIGHT: &str = include_str!("../assets/minimal-light.css");

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Theme {
    Dark,
    Light,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Sub-millisecond Markdown to HTML converter", long_about = None)]
struct Args {
    /// Path to the input Markdown file
    #[arg(required = true)]
    input: PathBuf,

    /// Output HTML path (default to same name with .html extension)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// CSS Theme style
    #[arg(short, long, value_enum, default_value_t = Theme::Dark)]
    theme: Theme,
}

fn main() -> Result<()> {
    // 1. Parse command line flags
    let args = Args::parse();

    // 2. Read input file
    let markdown_content = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read file: {:?}", args.input))?;

    // 3. Convert Markdown AST to HTML body
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = MdParser::new_ext(&markdown_content, options);
    let mut html_body = String::new();
    html::push_html(&mut html_body, parser);

    // 4. Select theme style sheet
    let css_theme = match args.theme {
        Theme::Dark => THEME_GITHUB_DARK,
        Theme::Light => THEME_MINIMAL_LIGHT,
    };

    // 5. Wrap body in a complete self-contained HTML document
    let full_html = format!(
        r#"<!DOCTYPE html>
        <html lang="en"
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>{}</title>
            <style>{}</style>
        </head>
        <body>
        {}
        </body>
        </html>"#,
        args.input.file_stem().unwrap_or_default().to_string_lossy(),
        css_theme,
        html_body
    );

    // 6. Determine output filename and write to disk
    let output_path = args.output.unwrap_or_else(|| args.input.with_extension("html"));
    fs::write(&output_path, full_html)
        .with_context(|| format!("Failed to write output to: {:?}", output_path))?;

    println!("Successfully converted {:?} to HTML at {:?}", args.input, output_path);
    Ok(())
    
}
