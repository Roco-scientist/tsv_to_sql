use std::fs;
use std::process;

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
        let text = fs::read_to_string(file_path).unwrap_or_else(|error| {
            eprintln!("Unable to open file error: {:?}", error);
            process::exit(1)
        });

        let filetype = match file_path {
            file_path if file_path.to_uppercase().contains(".CSV") => Filetype::CSV,
            file_path if file_path.to_uppercase().contains(".TSV") => Filetype::TSV,
            _ => {
                eprintln!("Filetype not recognized, TSV or CSV file needed");
                process::exit(1);
            }
        };

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
        InputFile {
            text,
            header,
            first_row,
            body,
            filetype,
        }
    }
    pub fn reform_header(&mut self) -> () {
        let sep = match self.filetype {
            Filetype::CSV => ",",
            Filetype::TSV => "\t",
        };
        let mut reformed_header = self.header.replace(sep, "`,`");
        reformed_header.push_str("`");
        let data_preceder = String::from("`");
        self.header = data_preceder + &reformed_header;
    }
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
