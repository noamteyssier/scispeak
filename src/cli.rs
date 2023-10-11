use clap::Parser;

#[derive(Parser)]
#[clap(version)]
pub struct Cli {
    /// Input file for read 1
    #[clap(short = 'i', long)]
    pub r1: String,

    /// Input file for read 2
    #[clap(short = 'I', long)]
    pub r2: String,

    /// Output prefix (will be written <prefix>_R[12].fastq.gz)
    #[clap(short = 'o', long, default_value = "scispeak")]
    pub prefix: String,

    /// Number of bases to skip after ligation barcode for UMI
    #[clap(short = 'u', long, default_value = "6")]
    pub umi_offset: usize,

    /// Length of UMI
    #[clap(short = 'U', long, default_value = "8")]
    pub umi_length: usize,

    /// Number of bases to skip after ligation barcode for cell barcode
    #[clap(short = 'b', long, default_value = "14")]
    pub bc_offset: usize,

    /// Length of cell barcode
    #[clap(short = 'B', long, default_value = "10")]
    pub bc_length: usize,

    /// Perform exact matching of cell barcodes (no one-off errors)
    #[clap(short = 'x', long)]
    pub exact: bool,

    /// Number of threads to use for compression
    #[clap(short = 't', long, default_value = "1")]
    pub threads: usize,

    /// Compression level (0-9)
    #[clap(short = 'l', long, default_value = "6")]
    pub level: u32,
}
