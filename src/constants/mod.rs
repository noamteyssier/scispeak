mod cell;
mod ligation;

pub use cell::CellBarcodes;
use hashbrown::HashMap;
pub use ligation::LigationBarcodes;

pub type Barcode = Vec<u8>;
pub type BarcodeMap = HashMap<Barcode, Barcode>;
