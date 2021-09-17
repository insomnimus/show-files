#![deny(clippy::all)]

use std::process;

use show_files::cmd::Cmd;

fn main() {
	process::exit(Cmd::from_args().run());
}
