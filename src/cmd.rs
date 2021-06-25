use crate::{
	sorter::{Sorter, SortBy},
	filter::{Filter, FileType, HiddenType},
	displayer::{Displayer, SpaceOpt},
	app,
};

pub struct Cmd{
	args: Vec<String>,
	filter: Filter,
	sorter: Sorter,
	displayer: Displayer,
}

impl Cmd{
	pub fn from_args() -> Self{
		let m= app::new().get_matches();
		
		let sorter = if let Some(v) = m.value_of("sort-ascending") {
			Sorter::new(false, match v{
				"none"=> SortBy::None,
				"name"=> SortBy::Name,
				"date-created"=> SortBy::DateCreated,
				"last-modified"=> SortBy::LastModified,
				"last-accessed"=> SortBy::LastAccessed,
				_=> unreachable!(),
			})
		}else if let Some(v) = m.value_of("sort-descending") {
			Sorter::new(true, match v{
					"none"=> SortBy::None,
				"name"=> SortBy::Name,
				"date-created"=> SortBy::DateCreated,
				"last-modified"=> SortBy::LastModified,
				"last-accessed"=> SortBy::LastAccessed,
				_=> unreachable!(),
			})
		}else{
			Sorter::new(false, SortBy::None)
		};
		
		let file_type= if m.is_present("files") {
			FileType::Files
		}else if m.is_present("directories") {
			FileType::Folders
		}else{
			FileType::Any
		};
		
		let hidden = if m.is_present("all") {
			HiddenType::Any
		}else if m.is_present("hidden") {
			HiddenType::Hidden
		}else{
			HiddenType::NotHidden
		};
		
		let space_opt= if m.is_present("quote") {
			SpaceOpt::Quote
		}else if m.is_present("escape") {
			SpaceOpt::Escape
		}else{
			SpaceOpt::Bare
		};
		
		let one_per_line= m.is_present("one-per-line");
		
		let args= m.values_of("pattern").map(String::from).unwrap_or_default();
		
		let filter = Filter{file_type, hidden};
		let displayer = Displayer{one_per_line, space_opt};
		
		Self{
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
		let opt= MatchOptions{
			case_sensitive: false,
			require_literal_separator: true,
			require_literal_leading_dot: matches!(self.filter.hidden, HiddenType::NotHidden),
		};
		
		let mut exit_code= 0usize;
		
		// helper closure to set the exit code
		// exit code 0: success
						// exit code 1: system error
						// exit code 2: user error
						// exit code 3: system error + user error
		let err_code= |n: usize| {
			if n == 1 {
				exit_code= match exit_code{
					0=> 1,
					2=> 3,
					_=> exit_code,
				};
			}else if n == 2{
				exit_code = match exit_code{
					0=> 2,
					1=> 3,
					_=> exit_code,
				};
			}
		};
		
		let mut files: Vec<FilePath>= vec![];
		
		for a in &self.args{
			// first check if it's a file or a dir
			// if it's not but is a glob pattern, execute it
			match  fs::metadata(&a) {
				Ok(md) => err_code(self.print_top_md(&a, md) != 0),
				Err(e) => {
					match e.kind() {
						ErrorKind::NotFound if !is_glob(&a) => {
							eprintln!("{}: the system cannot find the file specified", &a);
							err_code(2);
						}
						ErrorKind::PermissionDenied=> {
							eprintln!("{}: permission denied", &a);
							err_code(1);
						}
						ErrorKind::NotFound=> {
							files.extend(glob::glob_with(&a, opt)
							.unwrap_or_else(|e| {
								eprintln!("{}: error: {:?}", &a, &e);
								err_code(2);
								vec![]
								})
							.filter_map(Result::ok)
							.filter(|x| self.filter.file_type.is_match_path(&x))
							.map(|p| {
								match self.sorter.sort_by{
									SortBy::None=> FilePath::new(p),
									SortBy::DateCreated=> {
										if let Ok(md) = p.metadata() {
											FilePath::with_date_created(p, md.created().ok())
										}else{
											FilePath::new(p)
										}
									}
									SortBy::LastModified=> {
										if let Ok(md) = p.metadata() {
											FilePath::with_last_modified(p, md.modified().ok())
										}else{
											FilePath::new(p)
										}
									}
									SortBy::LastAccessed=> {
										if let Ok(md) = p.metadata() {
											FilePath::with_last_accessed(p, md.accessed().ok())
										}else{
											FilePath::new(p)
										}
									}
									SortBy::Size=> {
										let size = p.metadata().map(|md| md.len()).unwrap_or_default();
										FilePath::with_size(p, size)
									}
								}
							})
							);
						}
						_=> {
							eprintln!("{}: error: {:?}", &a, &e);
							err_code(1);
						}
					};
				}
		}
	}
	// end of loop
	
	if files.is_empty() {
		return exit_code;
	}
	
	self.sorter.sort(&mut files);
	
	let files: Vec<String> = files
	.into_iter()
	.map(|p| p.into_os_string().into_string())
	.filter_map(Result::ok)
	.collect();
	
	self.displayer.print(files);
	
	exit_code
	}
	
	fn print_top_md(name: &str, md: Metadata) -> usize{
		if md.is_file() {
			let s = name.trim_start_matches('./');
			#[cfg(windows)]
			let s = s.trim_start_matches(".\\");
			
			println!("{}", &s);
			0
		}else{
			// trim the folder name from collected paths
			
		}
	}
}