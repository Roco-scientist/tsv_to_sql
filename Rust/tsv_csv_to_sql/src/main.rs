use tsv_csv_to_sql;
use std::process;

fn main() {
    let (file_path, table_name) = tsv_csv_to_sql::arguments();
    let mut input_file = tsv_csv_to_sql::InputFile::load_file(&file_path);
    input_file.reform_header();
    input_file.infer_col_data_types();
    match tsv_csv_to_sql::write_sql_input(&input_file, &table_name) {
        Err(msg) => {
            eprintln!("Error found while writing: {:?}", msg);
            process::exit(1)
        },
        Ok(_) => println!("SQL file written"),
    }
}
