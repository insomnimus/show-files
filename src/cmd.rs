use glob::{self, MatchOptions};
use std::{fs, io::ErrorKind};

use crate::{
    app,
    displayer::{Displayer, SpaceOpt},
    filter::{FileType, Filter, HiddenType},
    sorter::{SortBy, Sorter},
};

pub struct Cmd {
    args: Vec<String>,
    filter: Filter,
    sorter: Sorter,
    displayer: Displayer,
}

impl Cmd {
    pub fn from_args() -> Self {
        let m = app::new().get_matches();

        let sorter = if let Some(v) = m.value_of("sort-ascending") {
            Sorter::new(
                false,
                match v {
                    "none" => SortBy::None,
                    "name" => SortBy::Name,
                    "date-created" => SortBy::DateCreated,
                    "last-modified" => SortBy::LastModified,
                    "last-accessed" => SortBy::LastAccessed,
                    _ => unreachable!(),
                },
            )
        } else if let Some(v) = m.value_of("sort-descending") {
            Sorter::new(
                true,
                match v {
                    "none" => SortBy::None,
                    "name" => SortBy::Name,
                    "date-created" => SortBy::DateCreated,
                    "last-modified" => SortBy::LastModified,
                    "last-accessed" => SortBy::LastAccessed,
                    _ => unreachable!(),
                },
            )
        } else {
            Sorter::new(false, SortBy::None)
        };

        let file_type = if m.is_present("files") {
            FileType::Files
        } else if m.is_present("directories") {
            FileType::Folders
        } else {
            FileType::Any
        };

        let hidden = if m.is_present("all") {
            HiddenType::Any
        } else if m.is_present("hidden") {
            HiddenType::Hidden
        } else {
            HiddenType::NotHidden
        };

        let space_opt = if m.is_present("quote") {
            SpaceOpt::Quote
        } else if m.is_present("escape") {
            SpaceOpt::Escape
        } else {
            SpaceOpt::Bare
        };

        let one_per_line = m.is_present("one-per-line");

        let args = m.values_of("pattern").map(String::from).unwrap_or_default();

        let filter = Filter { file_type, hidden };
        let displayer = Displayer {
            one_per_line,
            space_opt,
        };

        Self {
            args,
            filter,
            sorter,
            displayer,
        }
    }

    pub fn run(&self) -> usize {
        // handle the most common case first so it's more efficient
        if self.args.is_empty() {
            return self.run_pwd();
        }

        let opt = MatchOptions {
            case_sensitive: false,
            require_literal_separator: true,
            require_literal_leading_dot: matches!(self.filter.hidden, HiddenType::NotHidden),
        };

        let mut exit_code = 0usize;

        // helper closure to set the exit code
        // exit code 0: success
        // exit code 1: system error
        // exit code 2: user error
        // exit code 3: system error + user error
        let err_code = |n: usize| {
            if n == 1 {
                exit_code = match exit_code {
                    0 => 1,
                    2 => 3,
                    _ => exit_code,
                };
            } else if n == 2 {
                exit_code = match exit_code {
                    0 => 2,
                    1 => 3,
                    _ => exit_code,
                };
            }
        };

        for a in &self.args {
            let mut arg_is_dir = false;
            // first check if it's a file or a dir
            // if it's not but is a glob pattern, execute it
            let maybe_files = match fs::metadata(&a) {
                Ok(md) => {
                    if md.is_file() {
                        println!("\n{}", &a);
                        None
                    } else {
                        arg_is_dir = true;
                        match self.read_dir(&a) {
                            Err(code) => {
                                err_code(code);
                                None
                            }
                            Ok(files) => Some(files),
                        }
                    }
                }
                Err(e) => {
                    match e.kind() {
                        ErrorKind::NotFound if !is_glob(&a) => {
                            eprintln!("{}: the system cannot find the file specified", &a);
                            err_code(2);
                            None
                        }
                        ErrorKind::PermissionDenied => {
                            eprintln!("{}: permission denied", &a);
                            err_code(1);
                            None
                        }
                        // the arg is a glob pattern here
                        ErrorKind::NotFound => {
                            Some(
                                glob::glob_with(&a, opt)
                                    .unwrap_or_else(|e| {
                                        eprintln!("{}: error: {:?}", &a, &e);
                                        err_code(2);
                                        vec![]
                                    })
                                    .filter_map(Result::ok)
                                    .filter_map(|p| {
                                        // only request .metadata if it's required or wanted
                                        if !self.should_metadata() {
                                            Some(FilePath::new(p))
                                        } else {
                                            // metadata is needed
                                            p.metadata()
                                                .map(|md| self.sorter.sort_by.new_file_path(p, &md))
                                                .ok()
                                        }
                                    })
                                    .collect::<Vec<FilePath>>(),
                            )
                        }
                        _ => {
                            eprintln!("{}: error: {:?}", &a, &e);
                            err_code(1);
                            None
                        }
                    }
                }
            };

            if let Some(mut files) = maybe_files {
                self.sorter.sort(&mut files);
                let files: Vec<String> = files
                    .into_iter()
                    .filter_map(|fp| fp.path.to_os_string().to_string().ok())
                    .map(|s| if arg_is_dir { trim_folder(&a, s) } else { s })
                    .collect();

                if self.args.len() > 1 {
                    println!("# {}:", &a);
                }
                self.displayer.print(files);
            }
        }
        // end of loop

        exit_code
    }

    fn read_dir(&self, name: &str) -> Result<Vec<FilePath>, usize> {
        fs::read_dir(name)
            .map_err(|e| match e.kind() {
                ErrorKind::PermissionDenied => {
                    eprintln!("{}: permission denied", name);
                    1usize
                }
                _ => {
                    eprintln!("{}: error: {:?}", name, &e);
                    1usize
                }
            })
            .map(|files| {
                // filter entries
                files
                    .filter_map(Result::ok)
                    .filter(|p| {
                        if self.filter.hidden == HiddenOpt::Any {
                            true
                        } else {
                            p.file_name()
                                .to_os_string()
                                .to_string()
                                .map(|s| self.filter.hidden.is_match(&s))
                                .unwrap_or(false)
                        }
                    })
                    // filter and map at the same time because we don't want to call .metadata twice
                    .filter_map(|p| {
                        // only request .metadata if it's required or wanted
                        if !self.should_metadata() {
                            Some(FilePath::new(p))
                        } else {
                            // metadata is needed
                            p.metadata()
                                .map(|md| self.sorter.sort_by.new_file_path(p, &md))
                                .ok()
                        }
                    })
                    .collect::<Vec<_>>()
            })
    }

    fn run_pwd(&self) -> usize {
        match self.read_dir("./") {
            Err(code) => code,
            Ok(mut files) => {
                self.sorter.sort(&mut files);

                // trim the "./" prefix
                let files: Vec<String> = files
                    .into_iter()
                    .filter_map(|fp| fp.path.to_os_string().to_string().ok())
                    .map(|s| s.trim_start_matches("./"))
                    .collect();

                if !files.is_empty() {
                    self.displayer.print(files);
                }
                0
            }
        }
    }
}

#[cfg(not(windows))]
fn trim_folder(folder: &str, s: &str) -> String {
    s.trim_start_matches(folder)
}

#[cfg(windows)]
/// trim_folder trims the folder name from a path.
/// Since this is targeted for windows, the trimming is case insensitive.
fn trim_folder(folder: &str, s: &str) -> String {
    if folder.len() > s.len() {
        s.to_string()
    } else {
        let chars: Vec<_> = folder.chars().collect();
        s.iter()
            .enumerate()
            .skip_while(|(i, c)| {
                if let Some(x) = chars.get(*i) {
                    x == c || x.to_uppercase() == c.to_uppercase()
                } else {
                    false
                }
            })
            .map(|(_, c)| c)
            .collect::<String>()
    }
}
