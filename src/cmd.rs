use std::{
	fs,
	io::ErrorKind,
};

use atty::Stream;
use glob::{
	self,
	MatchOptions,
};

use crate::{
	app,
	displayer::{
		Displayer,
		SpaceOpt,
	},
	filepath::FilePath,
	filter::{
		FileType,
		Filter,
		HiddenType,
	},
	sorter::{
		SortBy,
		Sorter,
	},
};

pub struct Cmd<'a> {
	args: Vec<&'a str>,
	filter: Filter,
	sorter: Sorter,
	displayer: Displayer,
}

impl<'a> Cmd<'a> {
	fn run(self) -> i32 {
		let opt = MatchOptions {
			case_sensitive: false,
			require_literal_separator: true,
			require_literal_leading_dot: matches!(self.filter.hidden, HiddenType::NotHidden),
		};

		let mut n_err = 0;

		for a in &self.args {
			let mut arg_is_dir = false;
			// first check if it's a file or a dir
			// if it's not but is a glob pattern, execute it
			let maybe_files = match fs::metadata(&a) {
				Ok(md) => {
					if md.is_file() {
						println!("{}", &a);
						None
					} else {
						arg_is_dir = true;
						match self.read_dir(a) {
							Err(_) => {
								n_err += 1;
								None
							}
							Ok(files) => Some(files),
						}
					}
				}
				Err(e) => {
					match e.kind() {
						ErrorKind::NotFound if !super::is_glob(a) => {
							eprintln!("{}: the system cannot find the file specified", a);
							n_err += 1;
							None
						}
						ErrorKind::PermissionDenied => {
							eprintln!("{}: permission denied", &a);
							n_err += 1;
							None
						}
						// the arg is a glob pattern here
						_ => {
							let vals = glob::glob_with(a, opt)
								.unwrap_or_else(|e| {
									eprintln!("{a}: error: {e}");
									std::process::exit(n_err + 1);
								})
									.flatten()
                                        // filter and map at the same time, we don't want to call md twice
                                        .filter_map(|p| {
                                            // only request .metadata if it's required or wanted
                                            if self.should_md() {
																							p.metadata().ok().and_then(|md| {
																								if self.filter.file_type.is_match(&md) {
																									Some(
																									self.sorter
																									.sort_by
                                                                .new_filepath(p, &md),
                                                        )
                                                    } else {
                                                        None
                                                    }
                                                })
                                            } else {
                                                Some(FilePath::new(p))
                                            }
                                        })
                                        .collect::<Vec<FilePath>>();
							Some(vals)
						}
					}
				}
			};

			if let Some(mut files) = maybe_files {
				self.sorter.sort(&mut files);
				let files: Vec<_> = files
					.into_iter()
					.filter_map(|fp| fp.path.into_os_string().into_string().ok())
					.map(|s| {
						if arg_is_dir {
							super::trim_folder(a, &s)
						} else {
							s
						}
					})
					.collect();

				if self.args.len() > 1 {
					println!("# {}:", &a);
				}
				self.displayer.print(files);
			}
		}
		// end of loop

		n_err
	}

	fn read_dir(&self, name: &str) -> Result<Vec<FilePath>, i32> {
		fs::read_dir(name)
			.map_err(|e| {
				if e.kind() == ErrorKind::PermissionDenied {
					eprintln!("{}: permission denied", name);
					1
				} else {
					eprintln!("{}: error: {:?}", name, &e);
					1
				}
			})
			.map(|files| {
				// filter entries
				files
                    .filter_map(Result::ok)
                    .filter(|p| {
                        if self.filter.hidden == HiddenType::Any {
                            true
                        } else {
                            p.file_name()
                                .into_string()
                                .map(|s| self.filter.hidden.is_match(&s))
                                .unwrap_or(false)
                        }
                    })
                    // filter and map at the same time because we don't want to call md twice
                    .filter_map(|p| {
                        if self.should_md() {
                            // on windows, DirEntry::metadata does not traverse simlinks but we want that behaviour
                            if cfg!(windows) {
                                p.path().metadata()
                            } else {
                                p.metadata()
                            }
                            .ok()
                            .and_then(|md| {
                                if self.filter.file_type.is_match(&md) {
                                    Some(self.sorter.sort_by.new_filepath(p.path(), &md))
                                } else {
                                    None
                                }
                            })
                        } else {
                            Some(FilePath::new(p.path()))
                        }
                    })
                    .collect::<Vec<_>>()
			})
	}

	fn _run_pwd(&self) -> i32 {
		match self.read_dir("./") {
			Err(code) => code,
			Ok(mut files) => {
				self.sorter.sort(&mut files);

				// trim the "./" prefix
				let files: Vec<String> = files
					.into_iter()
					.filter_map(|fp| fp.path.into_os_string().into_string().ok())
					.map(|s| s.trim_start_matches("./").to_string())
					.collect();

				if !files.is_empty() {
					self.displayer.print(files);
				}
				0
			}
		}
	}

	fn should_md(&self) -> bool {
		self.filter.file_type != FileType::Any
			|| !matches!(self.sorter.sort_by, SortBy::None | SortBy::Name)
	}
}

pub fn run() -> i32 {
	let m = app::new().get_matches();

	let sorter = if let Some(v) = m.value_of("ascending") {
		Sorter::new(
			false,
			match v {
				"none" => SortBy::None,
				"size" => SortBy::Size,
				"name" => SortBy::Name,
				"created" => SortBy::DateCreated,
				"modified" => SortBy::LastModified,
				"accessed" => SortBy::LastAccessed,
				_ => unreachable!(),
			},
		)
	} else if let Some(v) = m.value_of("descending") {
		Sorter::new(
			true,
			match v {
				"none" => SortBy::None,
				"size" => SortBy::Size,
				"name" => SortBy::Name,
				"created" => SortBy::DateCreated,
				"modified" => SortBy::LastModified,
				"accessed" => SortBy::LastAccessed,
				_ => unreachable!(),
			},
		)
	} else if cfg!(windows) {
		// windows returns files in alphabetical order
		Sorter::new(false, SortBy::None)
	} else {
		// linux will have none of the automatic sort thing, it hands files randomly
		Sorter::new(false, SortBy::Name)
	};

	let file_type = if m.is_present("file") {
		FileType::File
	} else if m.is_present("dir") {
		FileType::Folder
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

	let space_opt = SpaceOpt::Bare;
	let one_per_line = m.is_present("1aline") || !atty::is(Stream::Stdout);

	let args = m.values_of("pattern").unwrap().collect::<Vec<_>>();

	let filter = Filter { file_type, hidden };
	let displayer = Displayer {
		one_per_line,
		space_opt,
	};

	let cmd = Cmd {
		args,
		filter,
		sorter,
		displayer,
	};

	cmd.run()
}
