use crate::FilePath;
use std::cmp::{Ordering, PartialOrd};

pub enum SortBy{
	None,
	Name,
	DateCreated,
	LastModified,
	LastAccessed,
	Size,
}

impl SortBy{
	pub fn sort_ascending(&self, files: &mut Vec<FilePath>) {
		files.sort_by(|a, b| {
			match self{
				self::None=> Ordering::Equal,
				Self::Name=> {
					cmp_opt(a.path.file_stem().as_ref(), b.path.file_stem().as_ref())
				}
				Self::DateCreated=> cmp_opt(&a.date_created, &b.date_created),
				Self::LastModified=> cmp_opt(&a.last_modified, &b.last_modified),
				Self::LastAccessed=> cmp_opt(&a.last_accessed, &b.last_accessed),
			},
		})
	}
	
	pub fn sort_descending(&self, files: &mut Vec<FilePath>) {
		files.sort_by(|b, a| {
			match self{
				self::None=> Ordering::Greater,
				Self::Name=> {
					cmp_opt(a.path.file_stem().as_ref(), b.path.file_stem().as_ref())
				}
				Self::DateCreated=> cmp_opt(&a.date_created, &b.date_created),
				Self::LastModified=> cmp_opt(&a.last_modified, &b.last_modified),
				Self::LastAccessed=> cmp_opt(&a.last_accessed, &b.last_accessed),
			},
		})
	}
}

pub struct Sorter{
	descending: bool,
	sort_by: SortBy,
}

impl Sorter{
	pub fn new(descending: bool, sort_by: SortBy) -> Self{
		Self{descending, sort_by}
	}
	
	pub fn sort(&self, files: &mut Vec<FilePath>) {
		if self.descending{
			self.sort_by.sort_descending(files);
		}else{
			self.sort_by.sort_ascending(files);
		}
	}
}

fn cmp_opt<T: PartialOrd>(a: &Option<T>, b: &Option<T>) -> Ordering{
	match (a, b) {
		(Some(t1), Some(t2))=> t1.cmp(t2),
		(None, Some(_))=> Ordering::Less,
		(Some(_), None)=> Ordering::Greater,
		(None, None)=> Ordering::Equal,
	}
}
