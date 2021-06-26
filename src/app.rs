use clap::{crate_version, App, AppSettings, Arg};

const SORT_VALUES: &[&str] = &[
    "none",
    "name",
    "size",
    "date-created",
    "last-modified",
    "last-accessed",
];

pub fn new() -> App<'static> {
    let app = App::new("sf")
        .version(crate_version!())
        .about("list files and directories")
        .setting(AppSettings::UnifiedHelpMessage);

    let one_per_line = Arg::new("one-per-line")
        .short('1')
        .long("one-per-line")
        .about("display files line by line");

    let files = Arg::new("files")
        .short('f')
        .long("files")
        .about("only show plain files");

    let directories = Arg::new("directories")
        .short('d')
        .long("directories")
        .about("only show directories")
        .conflicts_with("files");

    let all = Arg::new("all")
        .short('a')
        .long("all")
        .about("do not ignore hidden files and directories");

    let hidden = Arg::new("hidden")
        .short('A')
        .long("hidden")
        .about("only show hidden files and directories")
        .conflicts_with("all");

    let sort_ascending = Arg::new("sort-ascending")
        .short('s')
        .long("sort-ascending")
        .about("sort output ascending")
        .takes_value(true)
        .possible_values(SORT_VALUES);

    let sort_descending = Arg::new("sort-descending")
        .short('S')
        .long("sort-descending")
        .about("sort output descending")
        .takes_value(true)
        .possible_values(SORT_VALUES)
        .conflicts_with("sort-ascending");

    let quote = Arg::new("quote")
        .short('q')
        .long("quote")
        .about("single quote paths with spaces while printing");

    let escape = Arg::new("escape")
        .short('Q')
        .long("escape")
        .about("escape spaces while printing paths with spaces")
        .conflicts_with("quote");

    let args = Arg::new("pattern")
        .multiple(true)
        .about("list of glob patterns to match");

    app.arg(one_per_line)
        .arg(files)
        .arg(directories)
        .arg(all)
        .arg(hidden)
        .arg(sort_ascending)
        .arg(sort_descending)
        .arg(quote)
        .arg(escape)
        .arg(args)
}
