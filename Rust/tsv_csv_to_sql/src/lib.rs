enum Filetype {
    CSV,
    TSV,
}

pub fn transform_tab(data: String) -> String {
    // Removes the tabs and replaces with ","
    let mut reformed_data = data.replace("\t", "`,`");
    reformed_data.push_str("`");
    let data_preceder = String::from("`");
    return data_preceder + &reformed_data;
}

pub fn transform_csv(data: String) -> String {
    // Removes the commas and replaces with ","
    let mut reformed_data = data.replace(",", "`,`");
    reformed_data.push_str("`");
    let data_preceder = String::from("`");
    return data_preceder + &reformed_data;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transform_tab() {
        let header_ouput = String::from("`one`,`two`,`three`");
        assert_eq!(transform_tab(String::from("one\ttwo\tthree")), header_ouput);
    }
    #[test]
    fn test_transform_csv() {
        let header_ouput = String::from("`one`,`two`,`three`");
        assert_eq!(transform_csv(String::from("one,two,three")), header_ouput);
    }
}
