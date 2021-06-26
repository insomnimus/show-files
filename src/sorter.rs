use crate::filepath::FilePath;
use std::{
    cmp::{Ord, Ordering},
    fs::Metadata,
    path::PathBuf,
};

pub enum SortBy {
    None,
    Name,
    DateCreated,
    LastModified,
    LastAccessed,
    Size,
}

impl SortBy {
    pub fn sort_ascending(&self, files: &mut Vec<FilePath>) {
        files.sort_by(|a, b| match self {
            Self::Size => a.size.cmp(&b.size),
            Self::None => Ordering::Equal,
            Self::Name => cmp_opt(&a.path.file_stem(), &b.path.file_stem()),
            Self::DateCreated => cmp_opt(&a.date_created, &b.date_created),
            Self::LastModified => cmp_opt(&a.last_modified, &b.last_modified),
            Self::LastAccessed => cmp_opt(&a.last_accessed, &b.last_accessed),
        })
    }

    pub fn sort_descending(&self, files: &mut Vec<FilePath>) {
        files.sort_by(|b, a| match self {
            Self::Size => a.size.cmp(&b.size),
            Self::None => Ordering::Greater,
            Self::Name => cmp_opt(&a.path.file_stem(), &b.path.file_stem()),
            Self::DateCreated => cmp_opt(&a.date_created, &b.date_created),
            Self::LastModified => cmp_opt(&a.last_modified, &b.last_modified),
            Self::LastAccessed => cmp_opt(&a.last_accessed, &b.last_accessed),
        })
    }

    pub fn new_filepath(&self, p: PathBuf, md: &Metadata) -> FilePath {
        match self {
            Self::None | Self::Name => FilePath::new(p),
            Self::Size => FilePath::with_size(p, md.len()),
            Self::DateCreated => FilePath::with_date_created(p, md.created().ok()),
            Self::LastModified => FilePath::with_last_modified(p, md.modified().ok()),
            Self::LastAccessed => FilePath::with_last_accessed(p, md.accessed().ok()),
        }
    }
}

pub struct Sorter {
    pub descending: bool,
    pub sort_by: SortBy,
}

impl Sorter {
    pub fn new(descending: bool, sort_by: SortBy) -> Self {
        Self {
            descending,
            sort_by,
        }
    }
    pub fn sort(&self, files: &mut Vec<FilePath>) {
        if self.descending {
            self.sort_by.sort_descending(files);
        } else {
            self.sort_by.sort_ascending(files);
        }
    }
}

fn cmp_opt<T: Ord>(a: &Option<T>, b: &Option<T>) -> Ordering {
    match (a, b) {
        (Some(t1), Some(t2)) => t1.cmp(t2),
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}
