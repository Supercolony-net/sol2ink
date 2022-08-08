// MIT License

// Copyright (c) 2022 Supercolony

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
