#[derive(PartialEq)]
pub enum FileType {
    Any,
    File,
    Folder,
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
