use regex::Regex;

/// Splits a string by a string and returns the output
///
/// `string` string to be split
/// `split` string by which the original string will be split
pub fn split(string: &str, split: &str, map_maybe: Option<fn(&str) -> String>) -> Vec<String> {
    let map = map_maybe.unwrap_or(move |str| str.to_owned());
    string.split(split).map(map).collect::<Vec<String>>()
}

pub fn remove_commas() -> fn(&str) -> String {
    move |s: &str| {
        let mut out = s.to_owned();
        out.remove_matches(",");
        out
    }
}

pub fn trim(line: &str) -> String {
    let regex = Regex::new(r"\s+").unwrap();
    regex.replace_all(line, " ").trim().to_string()
}
