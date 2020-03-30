extern crate clap;

use std::process;
use clap::{App, Arg};

fn main() {
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Rory Coffey <coffeyrt@gmail.com>")
        .about("Takes in a TSV/CSV file and outputs in a SQL import format")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("TSV or CSV file"),
        )
        .get_matches();
    let input_file = match matches.value_of("file") {
        Some(file) => file,
        None => {
            eprintln!("Missing file argument: --file <file>");
            process::exit(1);
        }
    };
}
