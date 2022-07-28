use proc_macro2::TokenStream;
use std::{
    fs::File,
    io::{
        prelude::*,
        BufReader,
    },
};

use rust_format::{
    Config,
    Formatter,
    PostProcess,
    RustFmt,
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
pub fn write_file(lines: TokenStream, file_name: Option<String>) -> std::io::Result<()> {
    let path = file_name.unwrap_or_else(|| String::from("output.rs"));
    let mut file = File::create(path)?;

    let config = Config::new_str().post_proc(PostProcess::ReplaceMarkersAndDocBlocks);
    file.write_all(
        RustFmt::from_config(config)
            .format_tokens(lines)
            .unwrap()
            .as_bytes(),
    )?;
    Ok(())
}
