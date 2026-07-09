use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};
use webp::Encoder;
use walkdir::WalkDir;
use crate::summary::ConvertSummary;
use crate::utils::{is_image_file, format_bytes};
use log::{info, warn, error};

pub enum ConvertResult {
    Converted,
    Skipped,
}

pub fn convert_file_to_webp(input: &Path, output: &Path, overwrite: bool, quality: u8) -> anyhow::Result<ConvertResult> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    if !overwrite && output.exists() {
        warn!("skipped: {} already exists", output.display());
        return Ok(ConvertResult::Skipped);
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

    info!("converted: {}({}) -> {}({}) -> saved: {:.1}%", input.display(), format_bytes(before_size), output.display(), format_bytes(after_size), saved_percent);

    Ok(ConvertResult::Converted)
}

pub fn convert_dir_to_webp(input: &Path, output_dir: &Path, overwrite: bool, summary: &mut ConvertSummary, quality: u8) -> anyhow::Result<()> {
    let files: Vec<_> = WalkDir::new(input)
        .into_iter()
        .filter_map(Result::ok)
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file() && is_image_file(path))
        .collect();

    let pb = ProgressBar::new(files.len() as u64);

    let style = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}"
    )?
    .progress_chars("#>-");

    pb.set_style(style);
    pb.set_message("converting images");

    for path in files {
        if path.is_file() && is_image_file(&path) {
            let relative_path = path.strip_prefix(input)?;

            let mut output_path = output_dir.to_path_buf();
            output_path.push(relative_path);
            output_path.set_extension("webp");

            match convert_file_to_webp(&path, &output_path, overwrite, quality) {
                Ok(ConvertResult::Converted) => {
                    summary.add_converted();
                },
                Ok(ConvertResult::Skipped) => {
                    summary.add_skipped();
                },
                Err(err) => {
                    summary.add_failed();
                    error!("failed: {} ({})", path.display(), err);
                }
            }

            pb.inc(1);
        }

        pb.finish_and_clear();
    }

    Ok(())
}