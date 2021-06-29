#![deny(clippy::all)]
#![feature(iter_intersperse)]

mod app;
pub mod cmd;
mod displayer;
mod filepath;
mod filter;
mod sorter;

fn is_glob(s: &str) -> bool {
    s.chars().any(|c| c == '*' || c == '?' || c == '[')
}

#[cfg(not(windows))]
fn trim_folder(folder: &str, s: &str) -> String {
    let p = s.trim_start_matches(folder);
    if s.ends_with('/') {
        p.to_string()
    } else {
        p.trim_start_matches('/').to_string()
    }
}

#[cfg(windows)]
/// `trim_folder` trims the folder name from a path.
/// Since this is targeted for windows, the trimming is case insensitive.
fn trim_folder(folder: &str, s: &str) -> String {
    if folder.len() > s.len() {
        s.to_string()
    } else {
        let chars: Vec<_> = folder.chars().collect();
        s.chars()
            .enumerate()
            .skip_while(|(i, c)| {
                chars
                    .get(*i)
                    .map_or(false, |x| x == c || x.to_uppercase().eq(c.to_uppercase()))
            })
            .map(|(_, c)| c)
            .skip_while(|c| *c == '\\')
            .collect::<String>()
    }
}
mod table;
