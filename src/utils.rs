use std::io::Write;

use anyhow::Result;
use fxread::Record;

use crate::{
    cli::Cli,
    constants::{CellBarcodes, LigationBarcodes},
    write::write_records,
};

/// Recover a continuous stretch of bytes from a record.
pub fn get_contiguous(record: &Record, offset: usize, length: usize) -> &[u8] {
    let seq = record.seq();
    let end = offset + length;
    if end > seq.len() {
        panic!("Requested sequence is out of bounds");
    }
    &seq[offset..end]
}

/// Perform matching of barcodes and write to output files.
pub fn run_matching<R, W>(
    r1_reader: &mut R,
    r2_reader: &mut R,
    r1_writer: &mut W,
    r2_writer: &mut W,
    liga_bc_ref: &LigationBarcodes,
    cell_bc_ref: &CellBarcodes,
    cli: &Cli,
) -> Result<()>
where
    R: Iterator<Item = Record>,
    W: Write,
{
    let mut num_records = 0;
    let mut matched_records_liga = 0;
    let mut matched_records_cell = 0;

    for (r1, r2) in r1_reader.zip(r2_reader) {
        if let Some(liga) = liga_bc_ref.contains(&r1) {
            let cell_bc = get_contiguous(&r1, liga.len() + cli.bc_offset, cli.bc_length);

            if let Some(cell) = cell_bc_ref.contains(cell_bc) {
                let umi = get_contiguous(&r1, liga.len() + cli.umi_offset, cli.umi_length);

                write_records(r1_writer, r2_writer, cell, umi, &r1, &r2)?;
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
