use std::mem;

pub struct RowBuf {
	offsets: Vec<usize>,
	index: usize,
	buf: String,
}

impl RowBuf {
	pub fn new(width: usize, items: &[String], min_spaces: usize) -> Self {
		let mut n_col: usize = 1;
		let mut prev_cols = vec![0_usize];
		while n_col <= items.len() {
			let mut cols: Vec<usize> = (0..n_col).map(|_| 0_usize).collect();
			for chunk in items.chunks(n_col) {
				for (i, s) in chunk.iter().enumerate() {
					if s.len() > cols[i] {
						cols[i] = s.len();
					}
				}
			}

			// check if total width fits the term size
			let total_width: usize = cols.iter().intersperse(&min_spaces).sum();
			if total_width == width
				|| (total_width <= width && n_col * 2 >= items.len() && total_width * 2 >= width)
			{
				// perfect
				return Self {
					offsets: calc_offsets(&cols, min_spaces),
					index: 0,
					buf: String::with_capacity(width),
				};
			} else if total_width < width {
				prev_cols = cols;
				n_col += 1;
			} else {
				// return the previously calculated value that fit
				break;
			}
		}

		Self {
			offsets: calc_offsets(&prev_cols, min_spaces),
			index: 0,
			buf: String::with_capacity(width),
		}
	}

	pub fn push(&mut self, s: &str) -> Option<String> {
		if self.index >= self.offsets.len() {
			self.index = 1;
			let val = mem::take(&mut self.buf);
			self.buf.push_str(s);
			Some(val)
		} else if self.index == 0 {
			self.index += 1;
			self.buf.push_str(s);
			None
		} else {
			while self.buf.len() < self.offsets[self.index] {
				self.buf.push(' ');
			}
			self.buf.push_str(s);
			self.index += 1;
			None
		}
	}

	pub fn flush(&mut self) -> String {
		mem::take(&mut self.buf)
	}

	pub fn is_empty(&self) -> bool {
		self.buf.is_empty()
	}
}

fn calc_offsets(cols: &[usize], spaces: usize) -> Vec<usize> {
	if cols.is_empty() {
		return vec![];
	}

	let mut offsets = vec![];
	for i in 0..cols.len() {
		offsets.push(cols.iter().take(i).map(|n| n + spaces).sum::<usize>());
	}

	assert_eq!(cols.len(), offsets.len());
	offsets
}

#[test]
fn test_calc_offsets() {
	let cols: Vec<usize> = vec![1, 1, 1];
	let offsets = calc_offsets(&cols, 2);
	assert_eq!(offsets, vec![0, 3, 6]);
}

#[test]
fn test_rowbuf() {
	let mut items = vec!["a".to_string(), "b".to_string(), "c".to_string()];
	let mut buf = RowBuf::new(100, &items, 2);
	assert_eq!(&buf.offsets, &vec![0, 3, 6]);
	for _ in 0..items.len() {
		assert_eq!(buf.push(items.remove(0)), None);
	}
	assert_eq!(buf.push("1".to_string()), Some("a  b  c".to_string()),);

	assert!(buf.push("2".to_string()).is_none());
	assert!(buf.push("3".to_string()).is_none());
	assert_eq!(
		buf.push("pls work".to_string()),
		Some("1  2  3".to_string()),
	);
}

#[test]
fn test_n_columns() {
	fn stringer(n: usize) -> String {
		(0..n).map(|_| 'a').collect()
	}

	let items: Vec<String> = (25..30).map(stringer).collect();

	let buf = RowBuf::new(64, &items, 4);

	assert_eq!(0_usize, buf.offsets[0]);
	assert_eq!(2, buf.offsets.len());
}
