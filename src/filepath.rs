use std::{path::PathBuf, time::SystemTime};

pub struct FilePath {
	pub path: PathBuf,
	pub date_created: Option<SystemTime>,
	pub last_modified: Option<SystemTime>,
	pub last_accessed: Option<SystemTime>,
	pub size: u64,
}

impl FilePath {
	pub fn new(path: PathBuf) -> Self {
		Self {
			path,
			..[Default::default()]
		}
	}

	pub fn with_size(path: PathBuf, size: u64) -> Self {
		Self {
			path,
			size,
			..[Default::default()]
		}
	}

	pub fn with_date_created(path: PathBuf, date_created: Option<SystemTime>) -> Self {
		Self {
			path,
			date_created,
			..[Default::default()]
		}
	}

	pub fn with_last_modified(path: PathBuf, last_modified: Option<SystemTime>) -> Self {
		Self {
			path,
			last_modified,
			..[Default::default()]
		}
	}

	pub fn with_last_accessed(path: PathBuf, last_accessed: Option<SystemTime>) -> Self {
		Self {
			path,
			last_accessed,
			..[Default::default()]
		}
	}
}
