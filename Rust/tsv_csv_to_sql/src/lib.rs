pub fn transform_tab(data: String) -> String {
    // Removes the tabs and replaces with ","
    let mut reformed_data = data.replace("\t", "`,`");
    reformed_data.push_str("`");
    let data_preceder = String::from("`");
    return data_preceder + &reformed_data
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transform_tab(){
        assert_eq!(transform_tab(String::from("one\ttwo\tthree")), String::from("`one`,`two`,`three`"));
    }
}
