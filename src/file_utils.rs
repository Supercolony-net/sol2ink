use std::{
    fs::File,
    io::{
        prelude::*,
        BufReader,
    },
};

/// This function reads the file to be transpiled and returns it as string
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
pub fn write_file(lines: &Vec<String>) -> std::io::Result<()> {
    let mut file = File::create("output.rs")?;
    for line in lines.iter() {
        file.write_all(line.as_bytes())?;
    }
    Ok(())
}
