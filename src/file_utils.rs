use std::{
    fs::File,
    io::{
        prelude::*,
        BufReader,
    },
};

/// Reads the file to be transpiled and returns it as string
/// 
/// `path` the path to the file
pub fn read_file(path: &String) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

/// writes the output to file
///
/// `lines` the transpiled file in the form of vec of strings
/// each item in the vec represents a separate line in the output file
pub fn write_file(lines: &Vec<String>, file_name: Option<String>) -> std::io::Result<()> {
    let path = file_name.unwrap_or(String::from("output.rs"));
    let mut file = File::create(path)?;
    for line in lines.iter() {
        file.write_all(line.as_bytes())?;
    }
    Ok(())
}
