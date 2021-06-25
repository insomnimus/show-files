use rs_ls::cmd::Cmd;
use std::process;

fn main() {
	process::exit(Cmd::from_args().run());
}
