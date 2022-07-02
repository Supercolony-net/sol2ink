/// Splits a string by a string and returns the output
///
/// `string` string to be split
/// `split` string by which the original string will be split
pub fn split(string: &String, split: &str, map_maybe: Option<fn(&str) -> String>) -> Vec<String> {
    let map = map_maybe.unwrap_or(move |str| str.to_owned());
    string
        .split(split)
        .map(|str| map(str))
        .collect::<Vec<String>>()
}
