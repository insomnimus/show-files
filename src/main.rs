#![deny(clippy::all)]

use show_files::cmd::Cmd;
use std::process;

fn main() {
    process::exit(Cmd::from_args().run());
}
