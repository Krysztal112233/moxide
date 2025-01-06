use std::{path::PathBuf, str::FromStr, sync::OnceLock};

use parking_lot::RwLock;
use uuid::Uuid;

pub(crate) fn tmp_output_dir() -> PathBuf {
    let mut tmp = std::env::temp_dir();
    tmp.push(format!("moxide-{}", Uuid::new_v4()));
    tmp
}

static BASE_DIR: OnceLock<RwLock<&str>> = OnceLock::new();

pub(crate) fn base_dir() -> PathBuf {
    PathBuf::from_str(&BASE_DIR.get_or_init(|| RwLock::new(".")).read()).unwrap()
}

pub(crate) fn src_dir() -> PathBuf {
    let mut path = base_dir();
    path.push("src");
    path
}

pub(crate) fn output_dir() -> PathBuf {
    let mut path = base_dir();
    path.push("output");
    path
}
