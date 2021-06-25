use std::fs::Metadata;

#[derive(PartialEq)]
pub enum FileType {
	Any,
	File,
	Folder,
}

impl FileType {
	pub fn is_match(&self, md: &Metadata) -> bool {
		match self {
			Self::Any => true,
			Self::Folder => md.is_dir(),
			Self::File => md.is_file(),
		}
	}
}

#[derive(PartialEq)]
pub enum HiddenType {
	Any,
	Hidden,
	NotHidden,
}

impl HiddenType {
	pub fn is_match(&self, name: &str) -> bool {
		match self {
			Self::Any => true,
			Self::Hidden => name.starts_with('.'),
			Self::NotHidden => !name.starts_with('.'),
		}
	}
}

pub struct Filter {
	pub file_type: FileType,
	pub hidden: HiddenType,
}
