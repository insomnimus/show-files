use super::table::RowBuf;
use terminal_size::terminal_size;

/// SpaceOpt defines how paths with spaces should be displayed.
pub enum SpaceOpt {
    /// Do not format, just print.
    Bare,
    /// Single quote if path has spaces but doesn't have `'`; double quote if path has spaces and `'`.
    Quoted,
    /// Do not quote, escape spaces and the escape characters (`\\` on linux and `\`` on windows).
    Escaped,
}

impl SpaceOpt {
    /// Formats a string, according to the variant of `self`.
    pub fn format(&self, s: &str) -> String {
        match self {
            // windows escape character is "`".
            #[cfg(windows)]
            Self::Quoted if s.contains(' ') && s.contains('\'') => {
                format!(r#""{}""#, s.replace('`', "``"))
            }
            // Same with above but on linux the escape char is "\".
            #[cfg(not(windows))]
            Self::Quoted if s.contains(' ') && s.contains('\'') => {
                format!(r#""{}""#, s.replace('\\', "\\\\"))
            }
            // No need to escape here, `s` is guaranteed to not contain `'`.
            Self::Quoted if s.contains(' ') => {
                format!("'{}'", s)
            }
            #[cfg(windows)]
            Self::Escaped if s.contains(' ') => s.replace('`', "``").replace(' ', "` "),
            #[cfg(not(windows))]
            Self::Escaped if s.contains(' ') => s.replace('\\', "\\\\").replace(' ', "\\ "),
            Self::Bare | Self::Escaped | Self::Quoted => s.to_string(),
        }
    }
}

/// Displayer stores configuration that controls how files are printed to the screen.
pub struct Displayer {
    /// If set to `true`, every file be printed in a separate line.
    pub one_per_line: bool,
    /// `space_opt` dictates how paths containing spaces should be formatted, for example quoted or escaped.
    pub space_opt: SpaceOpt,
}

impl Displayer {
    /// `print` will pretty print the given paths (as Strings).
    pub fn print(&self, files: Vec<String>) {
        if self.one_per_line {
            self.print_one_per_line(&files);
        } else {
            self.print_cell(files);
        }
    }

    fn print_one_per_line(&self, files: &[String]) {
        for f in files {
            println!("{}", self.space_opt.format(f));
        }
    }

    /// `print_cell` will print each item as a table cell.
    fn print_cell(&self, mut files: Vec<String>) {
        let term_size = terminal_size().map(|x| x.0 .0).unwrap_or(128);
        for f in files.iter_mut() {
            *f = self.space_opt.format(f);
        }

        let rows = Rows::new(term_size as usize, files, 4);
        for row in rows {
            println!("{}", row);
        }
    }
}

pub struct Rows {
    buf: RowBuf,
    items: Vec<String>,
}

impl Rows {
    pub fn new(max_size: usize, items: Vec<String>, min_spaces: usize) -> Self {
        Self {
            buf: RowBuf::new(max_size, &items, min_spaces),
            items,
        }
    }
}

impl Iterator for Rows {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.items.is_empty() {
            return None;
        }
        while !self.items.is_empty() {
            if let Some(s) = self.buf.push(self.items.remove(0)) {
                return Some(s);
            }
        }
        if self.buf.is_empty() {
            None
        } else {
            Some(self.buf.flush())
        }
    }
}

#[test]
fn test_row_len() {
    fn veccer(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect::<Vec<String>>()
    }

    let mut items = veccer(&[
        "fasdfsdfffffffffff",
        "afsfsfsfsfsfsfsfswwwwwbasxefgasdfq",
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "1234123412341234123412342134",
        "fafa..:::::::::!!@fafafsdfiiiii",
    ]);

    items.push((0..50).map(|_| 'a').collect());

    let rows = Rows::new(100, items, 4);

    for row in rows {
        if row.len() > 100 {
            panic!("row.len is greater than 100:\n{}", &row);
        }
    }
}
