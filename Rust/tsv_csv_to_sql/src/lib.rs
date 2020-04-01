extern crate clap;

use clap::{App, Arg};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

pub fn arguments() -> (String, String) {
    let args = App::new("CSV/TSV to SQL converter")
        .version("0.1.0")
        .author("Rory Coffey <coffeyrt@gmail.com>")
        .about("Takes in a TSV/CSV file and outputs in a SQL import format")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .help("TSV or CSV file"),
        )
        .arg(
            Arg::with_name("table_name")
                .short("t")
                .long("table_name")
                .takes_value(true)
                .required(true)
                .help("The name to give the table within SQL"),
        )
        .get_matches();
    let mut file_path = String::new();
    let mut table_name = String::new();
    if let Some(file) = args.value_of("file") {
        file_path = file.to_string()
    };
    if let Some(table) = args.value_of("table_name") {
        table_name = table.to_string()
    };
    return (file_path, table_name);
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
    filetype: Filetype,
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
        let sep = self.separator();

        // replaces tabs or commas with `,`
        // also encapselates the whole header with ``
        let mut reformed_header = self.header.replace(&sep, "`,`");
        reformed_header.push_str("`");
        let data_preceder = String::from("`");
        self.header = data_preceder + &reformed_header;
    }

    pub fn reform_body(&mut self) -> () {
        //
    }

    pub fn infer_col_data_types(&self) -> Vec<String> {
        let sep = self.separator();
        let first_row_values: Vec<&str> = self.first_row.split(&sep).collect();
        let mut data_types = Vec::new();

        for cell in first_row_values {
            if let Ok(value) = cell.parse::<f32>() {
                if value.round() == value {
                    data_types.push("INT".to_string())
                } else {
                    data_types.push("FLOAT".to_string())
                }
            } else {
                data_types.push("VARCHAR(20)".to_string())
            }
        }
        data_types
    }

    fn separator(&self) -> String {
        match self.filetype {
            Filetype::CSV => ",".to_string(),
            Filetype::TSV => "\t".to_string(),
        }
    }
}

pub fn write_sql_input(input_file: &InputFile, table_name: &str) -> std::io::Result<()> {
    let path = Path::new("test.sql");
    let mut file = File::create(&path)?;
    let data_types = input_file.infer_col_data_types();

    file.write_all(format!("DROP TABLE IF EXISTS {};\n", &table_name).as_bytes())?;
    file.write_all(format!("CREATE TABLE {}(\n", &table_name).as_bytes())?;
    file.write_all("id INT NOT NULL AUTO_INCREMENT, \n".as_bytes())?;

    for (header, data_type) in input_file
        .header
        .split(",")
        .zip(data_types.iter())
    {
        file.write_all(format!("{} {},\n", header, data_type).as_bytes())?
    }

    file.write_all("PRIMARY KEY ( id ));\n\n".as_bytes())?;
    file.write_all(format!("INSERT INTO {}\n(", &table_name).as_bytes())?;
    file.write_all(input_file.header.as_bytes())?;
    file.write_all(")\nVALUES\n".as_bytes())?;
    file.write_all(input_file.body.as_bytes())?;
    file.write_all(";".as_bytes())?;

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
    #[test]
    fn test_return_data_types() {
        let mut test_input = InputFile::load_file("test.csv");
        assert_eq!(
            test_input.infer_col_data_types(),
            vec![
                "VARCHAR(20)".to_string(),
                "INT".to_string(),
                "FLOAT".to_string()
            ]
        )
    }
}
