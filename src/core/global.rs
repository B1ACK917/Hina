use std::env;

use once_cell::sync::Lazy;

pub static DEBUG: Lazy<bool> = Lazy::new(|| {
    match env::var("DEBUG") {
        Ok(_) => true,
        Err(_) => false
    }
});

pub static DATA_DIR: &str = ".nijika";
pub static RM_STACK: &str = "rm.stack";
pub static RECYCLE: &str = "RecycleBin";
pub static SPLITTER: &str = "##0x9876$$6789x0##";
pub static RAND_STR_LEN: u8 = 16;
