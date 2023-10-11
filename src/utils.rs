use fxread::Record;

/// Recover a continuous stretch of bytes from a record.
pub fn get_contiguous<'a>(record: &'a Record, offset: usize, length: usize) -> &[u8] {
    let seq = record.seq();
    let end = offset + length;
    if end > seq.len() {
        panic!("Requested sequence is out of bounds");
    }
    &seq[offset..end]
}
