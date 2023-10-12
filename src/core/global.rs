use std::env;

use once_cell::sync::Lazy;
use regex::Regex;

pub static DEBUG: Lazy<bool> = Lazy::new(|| {
    match env::var("DEBUG") {
        Ok(_) => true,
        Err(_) => false
    }
});

pub static DATA_DIR: &str = ".hina";
pub static RM_STACK: &str = "rm.stack";
pub static RECYCLE: &str = "RecycleBin";
pub static SPLITTER: &str = "##0x9876$$6789x0##";
pub static RAND_STR_LEN: u8 = 16;

pub static MAX_RECURSIVE_DEPTH: i8 = 64;

pub static MEM_EXTRACT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<name>\S+):\s+(?P<amount>\d+) kB").unwrap());