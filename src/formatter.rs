/// Appends `appended` vec of Strings to `output` vec of strings and adds a tab to the front
pub fn append_and_tab(output: &mut Vec<String>, appended: Vec<String>) {
    output.append(
        appended
            .iter()
            .map(|string| "\t".to_string() + string)
            .collect::<Vec<String>>()
            .as_mut(),
    );
}

/// Splits a string by a string and returns the output
///
/// `string` string to be split
/// `split` string by which the original string will be split
pub fn split(string: String, split: &str, map_maybe: Option<fn(&str) -> String>) -> Vec<String> {
    let map = map_maybe.unwrap_or(move |str| str.to_owned());
    string
        .split(split)
        .map(|str| map(str))
        .collect::<Vec<String>>()
}
