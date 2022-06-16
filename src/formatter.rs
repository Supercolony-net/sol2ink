/// This function will append `appended` vec of Strings to `output` vec of strings
/// and add a tab to the front
pub fn append_and_tab(output: &mut Vec<String>, appended: Vec<String>) {
    output.append(
        appended
            .iter()
            .map(|string| "\t".to_string() + string)
            .collect::<Vec<String>>()
            .as_mut(),
    );
}
