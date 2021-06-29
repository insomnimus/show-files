use std::{path::PathBuf, time::SystemTime};

pub struct FilePath {
    pub path: PathBuf,
    pub size: u64,
    pub date_created: Option<SystemTime>,
    pub last_modified: Option<SystemTime>,
    pub last_accessed: Option<SystemTime>,
}

impl Default for FilePath {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            size: 0_u64,
            date_created: None,
            last_modified: None,
            last_accessed: None,
        }
    }
}

impl FilePath {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            ..Self::default()
        }
    }

    pub fn with_size(path: PathBuf, size: u64) -> Self {
        Self {
            path,
            size,
            ..Self::default()
        }
    }

    pub fn with_date_created(path: PathBuf, date_created: Option<SystemTime>) -> Self {
        Self {
            path,
            date_created,
            ..Self::default()
        }
    }

    pub fn with_last_modified(path: PathBuf, last_modified: Option<SystemTime>) -> Self {
        Self {
            path,
            last_modified,
            ..Self::default()
        }
    }

    pub fn with_last_accessed(path: PathBuf, last_accessed: Option<SystemTime>) -> Self {
        Self {
            path,
            last_accessed,
            ..Self::default()
        }
    }
}
