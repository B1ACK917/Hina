use indexmap::IndexMap;
use once_cell::sync::Lazy;
use regex::Regex;

pub static DATA_DIR: &str = ".hina";
pub static RM_STACK: &str = "rm.stack";
pub static RECYCLE: &str = "RecycleBin";
pub static RAND_STR_LEN: usize = 16;
pub static MAX_RECURSIVE_DEPTH: usize = 64;
pub static MEM_EXTRACT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<name>\S+):\s+(?P<amount>\d+) kB").unwrap());

pub static HELP_DICT: Lazy<IndexMap<&str, IndexMap<&str, &str>>> = Lazy::new(|| {
    IndexMap::from([
        ("Use Hina recycle bin to remove/restore files", IndexMap::from([
            ("rm", "Remove target to recycle bin."),
            ("rb", "Operations on Hina recycle bin, list bin/restore/etc.")
        ])),
        ("Hina operations on filesystem", IndexMap::from([
            ("mkndir", "Make nested directories for each single file."),
            ("rn", "Batch renaming function, can also rename symbol links by set -s."),
            ("lc", "Link convertor, can convert symlink to hardlink and can also revert it.")
        ])),
        ("Powerful process utils", IndexMap::from([
            ("ps", "Advanced process checker, can see swap/pss/rss utilization and track process ancestor."),
        ])),
    ])
});