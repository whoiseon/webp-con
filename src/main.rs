use anyhow::Result;
use clap::{Parser, Subcommand, value_parser};
use std::path::PathBuf;
use webp_con::converter::{ConvertResult, convert_dir_to_webp, convert_file_to_webp};
use webp_con::summary::ConvertSummary;

#[derive(Subcommand, Debug)]
enum Commands {
    Convert {
        input: PathBuf,

        #[arg(short, long)]
        output: Option<PathBuf>,

        #[arg(long)]
        overwrite: bool,

        #[arg(short, long, default_value_t = 80, value_parser = value_parser!(u8).range(1..=100))]
        quality: u8,
    },
}

#[derive(Parser, Debug)]
#[command(name = "webp-con")]
#[command(about = "Convert images to WebP")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    match args.command {
        Commands::Convert {
            input,
            output,
            overwrite,
            quality,
        } => {
            let mut summary = ConvertSummary::default();

            if input.is_file() {
                let output = output.unwrap_or_else(|| input.with_extension("webp"));
                match convert_file_to_webp(&input, &output, overwrite, quality)? {
                    ConvertResult::Converted => summary.add_converted(),
                    ConvertResult::Skipped => summary.add_skipped(),
                }
            } else if input.is_dir() {
                let output_dir = output.unwrap_or_else(|| PathBuf::from("output"));

                if output_dir.extension().is_some() {
                    anyhow::bail!("{} is not a directory", output_dir.display());
                }

                convert_dir_to_webp(&input, &output_dir, overwrite, &mut summary, quality)?;
            } else {
                anyhow::bail!("input path does not exist: {:?}", input);
            }

            summary.print_summary();
        }
    }

    Ok(())
}
