mod cli;
mod constants;
mod utils;
mod write;

use crate::{cli::Cli, write::build_writer};
use anyhow::Result;
use clap::Parser;
use constants::{CellBarcodes, LigationBarcodes};
use fxread::initialize_reader;
use utils::run_matching;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize barcode references.
    let liga_bc_ref = LigationBarcodes::new();
    let cell_bc_ref = if cli.exact {
        CellBarcodes::new_exact()
    } else {
        CellBarcodes::new_oneoff()
    };

    // Initialize output filenames.
    let r1_output_filename = format!("{}_R1.fastq.gz", &cli.prefix,);
    let r2_output_filename = format!("{}_R2.fastq.gz", &cli.prefix,);
    let log_output_filename = format!("{}_log.json", &cli.prefix,);

    // Initialize readers and writers.
    let mut r1_reader = initialize_reader(&cli.r1)?;
    let mut r2_reader = initialize_reader(&cli.r2)?;
    let mut r1_writer = build_writer(&r1_output_filename, cli.threads, cli.level)?;
    let mut r2_writer = build_writer(&r2_output_filename, cli.threads, cli.level)?;

    // Match barcodes and write to output files.
    let match_stats = run_matching(
        &mut r1_reader,
        &mut r2_reader,
        &mut r1_writer,
        &mut r2_writer,
        &liga_bc_ref,
        &cell_bc_ref,
        &cli,
    )?;

    // Write log file.
    if cli.log {
        let mut log_writer = build_writer(&log_output_filename, cli.threads, cli.level)?;
        serde_json::to_writer_pretty(&mut log_writer, &match_stats)?;
    }
    serde_json::to_writer_pretty(&mut std::io::stdout(), &match_stats)?;

    Ok(())
}
