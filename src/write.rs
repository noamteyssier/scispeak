use anyhow::Result;
use fxread::Record;
use gzp::{deflate::Bgzf, Compression, ZBuilder};
use std::{
    ffi::OsStr,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    str::from_utf8,
};

pub fn build_writer(
    filename: &str,
    compression_threads: usize,
    compression_level: u32,
) -> Result<Box<dyn Write>> {
    let file = File::create(filename)?;
    let buffer = BufWriter::new(file);
    let ext = Path::new(filename).extension();
    if ext == Some(OsStr::new("gz")) || ext == Some(OsStr::new("bgz")) {
        let writer = ZBuilder::<Bgzf, _>::new()
            .num_threads(compression_threads)
            .compression_level(Compression::new(compression_level))
            .from_writer(buffer);
        Ok(Box::new(writer))
    } else {
        Ok(Box::new(buffer))
    }
}

pub fn write_records<W: Write>(
    writer_r1: &mut W,
    writer_r2: &mut W,
    cell_bc: &[u8],
    umi: &[u8],
    r1: &Record,
    r2: &Record,
) -> Result<()> {
    write!(
        writer_r1,
        "@{}\n{}{}\n+\n{}\n",
        r1.id_str(),
        from_utf8(cell_bc)?,
        from_utf8(umi)?,
        &r1.qual_str().unwrap()[..cell_bc.len() + umi.len()],
    )?;
    write!(
        writer_r2,
        "@{}\n{}\n+\n{}\n",
        r2.id_str(),
        r2.seq_str(),
        r2.qual_str().unwrap(),
    )?;
    Ok(())
}
