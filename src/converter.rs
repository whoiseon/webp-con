use std::path::Path;
use webp::Encoder;
use walkdir::WalkDir;
use crate::summary::ConvertSummary;
use crate::utils::{is_image_file, format_bytes};

pub fn convert_file_to_webp(input: &Path, output: &Path, overwrite: bool, summary: &mut ConvertSummary, quality: u8) -> anyhow::Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    if !overwrite && output.exists() {
        println!("skipped: {} already exists", output.display());
        summary.add_skipped();
        return Ok(());
    }

    let before_size = std::fs::metadata(input)?.len();

    let img = image::open(input)?;
    let rgba = img.to_rgba8();

    let encoder = Encoder::from_rgba(
        rgba.as_raw(),
        rgba.width(),
        rgba.height(),
    );
    let webp = encoder.encode(quality as f32);
    std::fs::write(output, &*webp)?;

    let after_size = std::fs::metadata(output)?.len();

    let ratio = after_size as f64 / before_size as f64;
    let saved_percent = (1.0 - ratio) * 100.0;

    summary.add_converted();
    println!("converted: {}({}) -> {}({}) -> saved: {:.1}%", input.display(), format_bytes(before_size), output.display(), format_bytes(after_size), saved_percent);

    Ok(())
}

pub fn convert_dir_to_webp(input: &Path, output_dir: &Path, overwrite: bool, summary: &mut ConvertSummary, quality: u8) -> anyhow::Result<()> {
    for entry in WalkDir::new(input) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && is_image_file(&path) {
            let relative_path = path.strip_prefix(input)?;

            let mut output_path = output_dir.to_path_buf();
            output_path.push(relative_path);
            output_path.set_extension("webp");

            match convert_file_to_webp(path, &output_path, overwrite, summary, quality) {
                Ok(()) => {},
                Err(err) => {
                    summary.add_failed();
                    eprintln!("failed: {} ({})", path.display(), err);
                }
            }
        }
    }

    Ok(())
}