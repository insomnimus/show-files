use std::fs::Metadata;

pub enum FileType{
	Any,
	File,
	Folder,
}

impl FileType{
	pub fn is_match(&self, md: &Metadata) -> bool{
		match self{
			Self::Any=> true,
			Self::Folder=> md.is_dir(),
			Self::File=> md.is_file(),
		}
	}
}

pub enum HiddenType{
	Any,
	Hidden,
	NotHidden,
}

impl HiddenType{
	pub fn is_match(&self, name: &str) -> bool{
		match self{
			Self::Any=> true,
			Self::Hidden=> name.starts_with('.'),
			Self::NotHidden=> !name.starts_with('.'),
		}
	}
}

pub struct Filter{
	pub file_type: FileType,
	pub hidden: HiddenType,
}

impl Filter{
	pub fn new(file_type: FileType, hidden: HiddenType) -> Self {
		Self{file_type, hidden}
	}
	
	pub fn is_match(&self, name: impl AsRef<str>, md: &Metadata) -> bool{
		self.file_type.match(md) && self.hidden.match(name.as_ref())
	}
}
