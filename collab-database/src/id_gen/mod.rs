mod gen;

use lazy_static::lazy_static;
use std::sync::Mutex;

use gen::RowIDGen;
lazy_static! {
  pub static ref ROW_ID_GEN: Mutex<RowIDGen> = Mutex::new(RowIDGen::new(0));
}
