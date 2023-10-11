# scispeak

a rust parser to convert sci-seq-v3 reads into kallisto compatible formats

a CLI tool to whitelist filter sci-seq-v3 reads and convert them to a 10X-style format.

## Overview

This tool is used to filter sciseq reads against their respective barcode whitelists
and then output fastq file formats in the style of 10X reads.

This parses the sci-seq-v3 format, identifies the cell barcodes and UMIs and
writes out a new file to resemble the 10X sequence construct to be used with
other tools that have not yet adopted the sci-seq format.

### sci-rna-seq3 Sequencing Construct

The sci-rna-seq3 sequencing construct is organized in the following way:

```text
            ┌─'illumina_p5:29'
            ├─'i5:10'
            ├─'truseq_read_1_adapter:33'
            │                            ┌─'hairpin_barcode:10'
            │                            ├─'hairpin_adapter:6'
            ├─read_1─────────────────────┤
            │                            ├─'umi:8'
──RNA───────┤                            └─'cell_bc:10'
            ├─'poly_T:98'
            ├─'read_2:98'
            │                            ┌─'ME:19'
            ├─i7_primer──────────────────┤
            │                            └─'s7:15'
            ├─'i7:10'
            └─'illumina_p7:24'
```

Visualization from [seqspec](https://github.com/IGVF/seqspec).

And so the resulting R1 and R2 files boil down to:

``` txt
# R1
[linker][adapter][umi][barcode]

# R2
[cDNA]
```

## Usage

This is a single command CLI tool. It requires just the R1 and R2 filepaths

``` bash
scispeak \
    -i data/SRR7827205_sample_R1.fastq.gz \
    -I data/SRR7827205_sample_R2.fastq.gz;
```

However, it can be accelerated using multiple compression threads:

``` bash
scispeak \
    -i data/SRR7827205_sample_R1.fastq.gz \
    -I data/SRR7827205_sample_R2.fastq.gz \
    -t 8;
```

And can store a log file as well to keep matching statistics:

``` bash
scispeak \
    -i data/SRR7827205_sample_R1.fastq.gz \
    -I data/SRR7827205_sample_R2.fastq.gz \
    -t 8 \
    -l;
```

### Outputs

This program will output 3 files per run:

1. `<args.prefix>_R1.fastq.gz`: A fastq with the `[barcode][UMI]` construct for all reads passing the whitelist.
2. `<args.prefix>_R2.fastq.gz`: An unaltered fastq of the R2 for all reads passing the whitelist.
4. `<args.prefix>_log.json`: A log file containing the filtering statistics of the run.
