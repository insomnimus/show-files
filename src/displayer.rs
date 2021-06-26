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
        let term_size = terminal_size().map(|x| x.0 .0 * 8 / 10).unwrap_or(128);
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
    capacity: usize,
    item_size: usize,
    cur_items: usize,
    buff: String,
}

impl RowBuf {
    fn new(term_size: usize, items: &[String], min_spaces: usize) -> Self {
        if items.is_empty() {
            return Self {
                capacity: 1,
                item_size: term_size,
                cur_items: 0,
                buff: String::new(),
            };
        }

        let item_size = items.iter().map(String::len).max().unwrap();
        if term_size <= item_size {
            return Self {
                capacity: 1,
                cur_items: 0,
                item_size,
                buff: String::with_capacity(item_size),
            };
        }

        let capacity = term_size / (item_size + min_spaces);
        Self {
            capacity,
            item_size: item_size + min_spaces,
            cur_items: 0,
            buff: String::with_capacity(capacity * item_size),
        }
    }

    fn push(&mut self, s: String) -> Option<String> {
        if self.cur_items >= self.capacity {
            self.cur_items = 1;
            Some(mem::replace(&mut self.buff, s))
        } else {
            let n_spaces = (self.cur_items * self.item_size) - self.buff.len();
            for _ in 0..n_spaces {
                self.buff.push(' ');
            }
            self.buff.push_str(&s);
            self.cur_items += 1;
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
    pub fn new(max_size: usize, items: Vec<String>, min_spaces: usize) -> Self {
        Self {
            buff: RowBuf::new(max_size, &items, min_spaces),
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
            if let Some(s) = self.buff.push(self.items.remove(0)) {
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
