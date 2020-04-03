extern crate clap;

use clap::{App, Arg};
use std::process;
use tsv_csv_to_sql;

fn main() {
    // retrieves command line arguments
    let (file_path, table_name, output_file) = arguments();
    // loads the file into an InputFile struct
    let mut input_file = tsv_csv_to_sql::InputFile::load_file(&file_path);
    // reforms the data
    input_file.reform_header();
    input_file.reform_body();
    // gets the data types from the first row
    input_file.infer_col_data_types();
    // writes in SQL input format to the output_file
    match tsv_csv_to_sql::write_sql_input(&input_file, &table_name, &output_file) {
        Err(msg) => {
            eprintln!("Error found while writing: {:?}", msg);
            process::exit(1)
        }
        Ok(_) => println!("SQL file written"),
    }
}

/// Gets the command line arguments and returns the file, table name, and output file name
pub fn arguments() -> (String, String, String) {
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
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output_file")
                .takes_value(true)
                .required(true)
                .help("The name wanted for the output file. Typically file.sql"),
        )
        .get_matches();
    let mut file_path = String::new();
    let mut table_name = String::new();
    let mut output_file = String::new();
    // assigns command line input -f to file_path
    if let Some(file) = args.value_of("file") {
        file_path = file.to_string()
    };
    // assigns command line input -t to table_name
    if let Some(table) = args.value_of("table_name") {
        table_name = table.to_string()
    };
    // assigns command line input -o to output_file
    if let Some(table) = args.value_of("output_file") {
        output_file = table.to_string()
    };
    return (file_path, table_name, output_file);
}

