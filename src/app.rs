use clap::{
	arg,
	crate_version,
	App,
	Arg,
};

const SORT_BY: &[&str] = &["none", "name", "size", "created", "modified", "accessed"];

pub fn new() -> App<'static> {
	App::new("sf")
		.version(crate_version!())
		.about("list files and directories")
		.args(&[
			arg!(-'1' --"1aline" "Show each entry in a new line."),
			arg!(-f --file "Only display regular files."),
			arg!(-d --dir "Only display directories.").conflicts_with("file"),
			arg!(-a --all "Do not ignore hidden files."),
			arg!(-A --hidden "Only show hidden files.").conflicts_with("all"),
			arg!(-s --ascending [BY] "Sort files ascending.")
				.possible_values(SORT_BY)
				.ignore_case(true),
			arg!(-S --descending [BY] "Sort files descending.")
				.conflicts_with("ascending")
				.possible_values(SORT_BY)
				.ignore_case(true),
			Arg::new("pattern")
				.help("Filename or glob pattern.")
				.multiple_values(true)
				.default_values(&["."])
				.hide_default_value(true),
		])
}
