use std::mem;
use terminal_size::terminal_size;

pub enum SpaceOpt {
    Bare,
    Quoted,
    Escaped,
}

impl SpaceOpt {
    pub fn format(&self, s: &str) -> String {
        match self {
            Self::Quoted if s.contains(' ') => {
                format!("'{}'", s.replace('\'', "\\'"))
            }
            Self::Escaped if s.contains(' ') => s.replace(' ', "\\ "),
            Self::Bare | Self::Escaped | Self::Quoted => s.to_string(),
        }
    }
}

pub struct Displayer {
    pub one_per_line: bool,
    pub space_opt: SpaceOpt,
}

impl Displayer {
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

    fn print_cell(&self, mut files: Vec<String>) {
        let term_size = terminal_size().map(|x| x.0 .0).unwrap_or(128) - 1;
        for f in files.iter_mut() {
            *f = self.space_opt.format(f);
        }

        let rows = Rows::new(term_size as usize, files, 4);
        for row in rows {
            println!("{}", row);
        }
    }
}

struct RowBuf {
    n_col: usize,
    col_len: usize,
    col_index: usize,
    buff: String,
}

impl RowBuf {
    fn new(max_size: usize, items: &[String], min_spaces: usize) -> Self {
        let mut n_col = items.len();
        while n_col >= 1 {
            let max = items
                .chunks(n_col)
                .map(|i| i.iter().map(|s| s.len()).sum::<usize>() + min_spaces * (n_col - 1))
                .max()
                .unwrap_or(100);
            if max <= max_size {
                return Self {
                    n_col,
                    col_len: max_size / n_col,
                    col_index: 0,
                    buff: String::new(),
                };
            } else {
                n_col /= 2;
            }
        }

        Self {
            n_col: 1,
            col_len: items.iter().map(|s| s.len()).max().unwrap_or(100),
            col_index: 0,
            buff: String::new(),
        }
    }

    fn push(&mut self, s: &str) -> Option<String> {
        if self.col_index >= self.n_col {
            self.col_index = 0;
            Some(mem::replace(
                &mut self.buff,
                format!("{item:width$}", item = s, width = self.col_len),
            ))
        } else {
            self.col_index += 1;
            self.buff.push_str(s);
            let spaces = if self.col_len < s.len() || self.n_col == self.col_index {
                0
            } else {
                self.col_len - s.len()
            };
            for _ in 0..spaces {
                self.buff.push(' ');
            }
            None
        }
    }

    fn flush(&mut self) -> String {
        mem::take(&mut self.buff)
    }
}

pub struct Rows {
    buff: RowBuf,
    items: Vec<String>,
}

impl Rows {
    pub fn new(max_size: usize, items: Vec<String>, max_spaces: usize) -> Self {
        Self {
            buff: RowBuf::new(max_size, &items, max_spaces),
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
            if let Some(s) = self.buff.push(&self.items.remove(0)) {
                return Some(s);
            }
        }
        if self.buff.buff.is_empty() {
            None
        } else {
            Some(self.buff.flush())
        }
    }
}
