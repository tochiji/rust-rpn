use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::{App, Arg, ArgMatches};

fn main() {
    let opts = get_matches();

    if let Some(path) = opts.value_of("formula_file") {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.is_present("verbose"));
    } else {
        println!("No file is specified");
    }
}

fn run(reader: BufReader<File>, verbose: bool) {
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }

    println!("Is verbosity specified?: {}", verbose);
}

fn get_matches() -> ArgMatches {
    App::new("逆ポーランド記法コマンドラインツール")
        .version("1.0.0")
        .author("tochiji")
        .about("逆ポーランド記法で計算ができます")
        .arg(
            Arg::new("formula_file")
                .about("Formulas written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .about("Sets the level of verbosity")
                .short('v')
                .long("verbose")
                .required(false),
        )
        .get_matches()
}
