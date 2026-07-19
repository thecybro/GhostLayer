use anyhow::{bail, Result};
use clap::Parser;

use image::{
    GenericImageView,
    imageops::FilterType,
};

use std::fs;
use std::path::PathBuf;

/// Generate icons from a master image.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input image (preferably 1024x1024 PNG)
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory
    #[arg(short, long)]
    output: PathBuf,

    /// File prefix
    #[arg(short, long, default_value = "icon")]
    name: String,

    /// Comma-separated list of icon sizes.
    ///
    /// Example:
    /// --sizes 16,32,48,128
    #[arg(
        short,
        long,
        value_delimiter = ',',
        default_value = "16,32,48,128"
    )]
    sizes: Vec<u32>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.input.exists() {
        bail!("Input image does not exist.");
    }

    fs::create_dir_all(&args.output)?;

    let img = image::open(&args.input)?;

    let (w, h) = img.dimensions();

    if w < 128 || h < 128 {
        bail!(
            "Input image is too small ({}x{}). Use at least 128×128 (1024×1024 recommended).",
            w,
            h
        );
    }

    for &size in &args.sizes {
        let resized = img.resize_exact(size, size, FilterType::Lanczos3);

        let filename = format!("{}{}.png", args.name, size);

        let path = args.output.join(filename);

        resized.save(&path)?;

        println!("✓ {}", path.display());
    }

    println!("\nGenerated {} icon(s).", args.sizes.len());

    Ok(())
}