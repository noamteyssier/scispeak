mod cli;
mod constants;
mod utils;
mod write;

use crate::{
    cli::Cli,
    utils::get_contiguous,
    write::{build_writer, write_records},
};
use anyhow::Result;
use clap::Parser;
use constants::{CellBarcodes, LigationBarcodes};
use fxread::initialize_reader;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let liga_bc_ref = LigationBarcodes::new();
    let cell_bc_ref = if cli.exact {
        CellBarcodes::new_exact()
    } else {
        CellBarcodes::new_oneoff()
    };

    let r1_reader = initialize_reader(&cli.r1)?;
    let r2_reader = initialize_reader(&cli.r2)?;

    let r1_output_filename = format!("{}_R1.fastq.gz", &cli.prefix,);

    let r2_output_filename = format!("{}_R2.fastq.gz", &cli.prefix,);

    let mut r1_writer = build_writer(&r1_output_filename, cli.threads, cli.level)?;

    let mut r2_writer = build_writer(&r2_output_filename, cli.threads, cli.level)?;

    let mut num_records = 0;
    let mut matched_records_liga = 0;
    let mut matched_records_cell = 0;

    for (r1, r2) in r1_reader.zip(r2_reader) {
        if let Some(liga) = liga_bc_ref.contains(&r1) {
            let cell_bc = get_contiguous(&r1, liga.len() + cli.bc_offset, cli.bc_length);

            if let Some(cell) = cell_bc_ref.contains(cell_bc) {
                let umi = get_contiguous(&r1, liga.len() + cli.umi_offset, cli.umi_length);

                write_records(&mut r1_writer, &mut r2_writer, cell, umi, &r1, &r2)?;
                matched_records_cell += 1;
            }
            matched_records_liga += 1;
        }
        num_records += 1;
    }

    eprintln!("Total records: {}", num_records);
    eprintln!("Ligation matched: {}", matched_records_liga);
    eprintln!("Cell matched: {}", matched_records_cell);

    Ok(())
}
