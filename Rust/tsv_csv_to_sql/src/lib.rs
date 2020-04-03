use std::fs::File;
use std::io::prelude::*;
use std::{fs, process};

/// A private enum created for labeling input files as either TSV or CSV
///
enum Filetype {
    CSV,
    TSV,
}

/// A struct for structering the CSV or TSV data in order to be converted to SQL
///
/// #Examples
///
/// ```
/// use tsv_csv_to_sql::InputFile;
///
/// let file_path = "test.csv";
/// let input_data = InputFile::load_file(file_path);
/// ```
pub struct InputFile {
    text: String,      //The input text body.  Used for testing
    header: String,    //Column header or the first row of the CSV/TSV file
    first_row: String, //First row after the header.  This is used to infer data types
    body: Vec<String>, //All rows other than the header from the CSV/TSV file
    filetype: Filetype,
}

/// Methods attached to InputFile for creating and editing for output to SQL input format
///
/// #Examples
///
/// ```
/// use tsv_csv_to_sql;
///
/// let file_path = "test.csv";
/// let mut input_data = tsv_csv_to_sql::InputFile::load_file(file_path);
/// input_data.reform_header();
/// input_data.reform_body();
/// ```
impl InputFile {
    /// loads the file and formats it for the InputFile struct
    ///
    /// #Examples
    ///
    /// ```
    /// use tsv_csv_to_sql;
    ///
    /// let file_path = "test.csv";
    /// let input_data = tsv_csv_to_sql::InputFile::load_file(file_path);
    /// ```
    pub fn load_file(file_path: &str) -> InputFile {
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
        let mut contents = text.lines();
        let header = contents.next().unwrap().to_string();
        let body = contents.map(|line| line.to_string()).collect::<Vec<String>>();
        let first_row = body[0].clone();

        // returns
        InputFile {
            text,
            header,
            first_row,
            body,
            filetype,
        }
    }

    /// Reforms the column headers to be comma separated and `` encapselated for SQL input requirements
    ///
    /// #Examples
    ///
    /// ```
    /// use tsv_csv_to_sql;
    ///
    /// let file_path = "test.csv";
    /// let mut input_data = tsv_csv_to_sql::InputFile::load_file(file_path);
    /// input_data.reform_header();
    /// ```
    pub fn reform_header(&mut self) -> () {
        let sep = self.separator();

        // replaces tabs or commas with `,`
        // also encapselates the whole header with ``
        let mut reformed_header = self.header.replace(&sep, "`,`");
        reformed_header.push_str("`");
        let data_preceder = String::from("`");
        self.header = data_preceder + &reformed_header;
    }

    /// Reforms the body of the dataframe to be a SQL format input
    ///
    /// String are encapselated by '', while numerics are not
    /// Also comma separtes the cells
    /// Returns so that each row is a String within a Vec
    ///
    /// #Examples
    ///
    /// ```
    /// use tsv_csv_to_sql;
    ///
    /// let file_path = "test.csv";
    /// let mut input_data = tsv_csv_to_sql::InputFile::load_file(file_path);
    /// input_data.reform_body();
    /// ```
    pub fn reform_body(&mut self) -> () {
        let data_types = self.infer_col_data_types();
        let sep = self.separator();
        let mut new_body: Vec<String> = Vec::new();

        for row in self.body.iter() {
            let mut new_row = String::new();
            for (cell, data_type) in row.split(&sep).zip(data_types.iter()) {
                if data_type == "VARCHAR(20)" {
                    new_row.push_str(&format!("'{}',", cell))
                } else {
                    new_row.push_str(&format!("{},", cell))
                }
            }
            new_row.pop();
            new_body.push(new_row.clone())
        }

        self.body = new_body
    }

    /// Infors the datatypes from the first row of the database
    ///
    /// This may lead to errors in the future based on the integer checking.  A float of 3.0 will be
    /// called an integer, which will impact lower rows. Kept public because this method is also
    /// used during file writing.
    ///
    /// #Examples
    ///
    /// ```
    /// use tsv_csv_to_sql;
    ///
    /// let file_path = "test.csv";
    /// let input_data = tsv_csv_to_sql::InputFile::load_file(file_path);
    /// input_data.infer_col_data_types();
    /// ```
    pub fn infer_col_data_types(&self) -> Vec<String> {
        let sep = self.separator();
        let first_row_values: Vec<&str> = self.first_row.split(&sep).collect();
        let mut data_types = Vec::new();

        //Find what data type the first row contains
        for cell in first_row_values {
            // check if it can be a number
            if let Ok(value) = cell.parse::<f32>() {
                // check if the number is an integer.  round(2.0) == 2
                if value.round() == value {
                    data_types.push("INT".to_string())
                } else {
                    // if not integer then a float
                    data_types.push("FLOAT".to_string())
                }
            // if not a number, then a character string
            } else {
                data_types.push("VARCHAR(20)".to_string())
            }
        }
        data_types
    }

    /// An internal method to find the separator used based on the Filetype
    ///
    /// #Panics
    ///
    /// Exits if the file type detected, TSV or CSV, does not contain any separators which these
    /// types indicate, ie \t or ","
    fn separator(&self) -> String {
        let sep = match self.filetype {
            Filetype::CSV => ",".to_string(),
            Filetype::TSV => "\t".to_string(),
        };
        // check if the text contains the separator indicated by the file type
        if self.text.contains(&sep) {
            return sep;
        } else {
            eprintln!("File separator does not match file naming\nOr only one column");
            process::exit(1)
        }
    }
}

/// Writes an sql file which can be used to input into an SQL database using InputFile after being
/// reformed
///
/// #Examples
///
/// ```
/// use tsv_csv_to_sql;
///
/// let file_path = "test.csv";
/// let output_file = "test.sql";
/// let table_name = "test";
/// let mut input_data = tsv_csv_to_sql::InputFile::load_file(file_path);
/// input_data.reform_header();
/// input_data.reform_body();
/// tsv_csv_to_sql::write_sql_input(&input_data, &table_name, &output_file);
/// ```
///
/// to import into SQL:
/// ```c
/// mysql -u <username> -p < <sql_file>
/// ```
pub fn write_sql_input(
    input_file: &InputFile,
    table_name: &str,
    output_file: &str,
) -> std::io::Result<()> {
    let mut file = File::create(output_file)?;
    let data_types = input_file.infer_col_data_types();

    file.write_all(format!("DROP TABLE IF EXISTS {};\n", &table_name).as_bytes())?;
    file.write_all(format!("CREATE TABLE {}(\n", &table_name).as_bytes())?;
    file.write_all("id INT NOT NULL AUTO_INCREMENT, \n".as_bytes())?;

    for (header, data_type) in input_file.header.split(",").zip(data_types.iter()) {
        file.write_all(format!("{} {},\n", header, data_type).as_bytes())?
    }

    file.write_all("PRIMARY KEY ( id ));\n\n".as_bytes())?;
    file.write_all(format!("INSERT INTO {}\n(", &table_name).as_bytes())?;
    file.write_all(input_file.header.as_bytes())?;
    file.write_all(")\nVALUES\n".as_bytes())?;

    for (x, row) in input_file.body.iter().enumerate() {
        if x + 1 == input_file.body.len() {
            file.write_all(format!("({})", row).as_bytes())?
        } else {
            file.write_all(format!("({}),\n", row).as_bytes())?
        }
    }
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
        let test_input = InputFile::load_file("test.csv");
        assert_eq!(
            test_input.infer_col_data_types(),
            vec![
                "VARCHAR(20)".to_string(),
                "INT".to_string(),
                "FLOAT".to_string()
            ]
        )
    }
    #[test]
    fn test_reform_body() {
        let mut test_input = InputFile::load_file("test.csv");
        test_input.reform_body();
        assert_eq!(
            test_input.body,
            vec!["'One',2,3.2".to_string(), "'One',2,3.2".to_string()]
        )
    }
}
