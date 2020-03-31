extern crate clap;

use clap::{App, Arg};
use std::fs;
use std::fs::File;
use std::process;
use std::path::Path;
use std::io::prelude::*;

pub fn get_file_path() -> String {
    let matches = App::new("CSV/TSV to SQL converter")
        .version("0.1.0")
        .author("Rory Coffey <coffeyrt@gmail.com>")
        .about("Takes in a TSV/CSV file and outputs in a SQL import format")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("TSV or CSV file")
        )
        .get_matches();

    match matches.value_of("file") {
        Some(file) => file.to_string(),
        None => {
            eprintln!("Missing file argument: --file <file>");
            process::exit(1);
        }
    }

}

enum Filetype {
    CSV,
    TSV,
}

pub struct InputFile {
    text: String,
    header: String,
    first_row: String,
    body: String,
    filetype: Filetype
}

impl InputFile {
    pub fn load_file(file_path: &str) -> InputFile {
        // reads in the csv/tsv file and pulls important information into the InputFile struct
        let text = fs::read_to_string(file_path).unwrap_or_else(|error| {
            eprintln!("Unable to open file error: {:?}", error);
            process::exit(1)
        });

        // finds whether the file type is a CSV or a TSV
        let filetype = match file_path {
            file_path if file_path.to_uppercase().contains(".CSV") => Filetype::CSV,
            file_path if file_path.to_uppercase().contains(".TSV") => Filetype::TSV,
            _ => {
                eprintln!("Filetype not recognized, TSV or CSV file needed");
                process::exit(1);
            }
        };

        // splits out the header, first row, and the body of the data file for future parsing
        let mut header = String::new();
        let mut first_row = String::new();
        let mut body = String::new();
        for (x, line) in text.lines().enumerate() {
            if x == 0 {
                header = line.to_string()
            };
            if x == 1 {
                first_row = line.to_string();
                body.push_str(line)
            };
            if x > 1 {
                body.push_str(line)
            };
        }

        // returns
        InputFile {
            text,
            header,
            first_row,
            body,
            filetype,
        }
    }
    pub fn reform_header(&mut self) -> () {
        // for input into SQL the header needs to be reformatted with `` and comma separated
        let sep = match self.filetype {
            Filetype::CSV => ",",
            Filetype::TSV => "\t",
        };

        // replaces tabs or commas with `,`
        // also encapselates the whole header with ``
        let mut reformed_header = self.header.replace(sep, "`,`");
        reformed_header.push_str("`");
        let data_preceder = String::from("`");
        self.header = data_preceder + &reformed_header;
    }
}

pub fn write_sql_input(input_file: &InputFile) -> std::io::Result<()> {
    let path = Path::new("test.sql");
    let mut file = File::create(&path)?;
    file.write_all(input_file.header.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_file() {
        assert_eq!(
            InputFile::load_file("test.csv").text,
            fs::read_to_string("test.csv").unwrap()
        );
    }
    #[test]
    fn test_reform_header() {
        let mut test_input = InputFile::load_file("test.csv");
        test_input.reform_header();
        assert_eq!(test_input.header, "`one`,`two`,`three`".to_string());
    }
}
