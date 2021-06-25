mod app;
pub mod cmd;
mod displayer;
mod filepath;
mod filter;
mod sorter;

fn is_glob(s: &str) -> bool {
	s.contains(&['*', '?', '['])
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
