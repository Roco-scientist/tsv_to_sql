use tsv_csv_to_sql;
use std::process;

fn main() {
    let file_path = tsv_csv_to_sql::get_file_path();
    let mut input_file = tsv_csv_to_sql::InputFile::load_file(&file_path);
    input_file.reform_header();
    match tsv_csv_to_sql::write_sql_input(&input_file) {
        Err(msg) => {
            eprintln!("Error found while writing: {:?}", msg);
            process::exit(1)
        },
        Ok(_) => println!("SQL file written"),
    }
}
