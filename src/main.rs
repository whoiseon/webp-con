use anyhow::Result;
use clap::{Parser, value_parser};
use std::path::{PathBuf};
use webp_con::converter::{convert_file_to_webp, convert_dir_to_webp};
use webp_con::summary::ConvertSummary;

#[derive(Parser, Debug)]
struct Args {
    input: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(long)]
    overwrite: bool,

    #[arg(short, long, default_value_t = 80, value_parser = value_parser!(u8).range(1..=100))]
    quality: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut summary = ConvertSummary::default();

    if args.input.is_file() {
        let output = args.output.unwrap_or_else(|| args.input.with_extension("webp"));
        convert_file_to_webp(&args.input, &output, args.overwrite, &mut summary, args.quality)?;
        summary.print_done()
    } else if args.input.is_dir() {
        let output_dir = args.output.unwrap_or_else(|| PathBuf::from("output"));

        if output_dir.extension().is_some() {
            anyhow::bail!("{} is not a directory", output_dir.display());
        }

        convert_dir_to_webp(&args.input, &output_dir, args.overwrite, &mut summary, args.quality)?;
    } else {
        anyhow::bail!("input path does not exist: {:?}", args.input);
    }

    summary.print_summary();

    Ok(())
}