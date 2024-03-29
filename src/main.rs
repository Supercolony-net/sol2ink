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

#![feature(once_cell)]
#![feature(string_remove_matches)]
#![feature(exclusive_range_pattern)]

pub mod assembler;
pub mod file_utils;
pub mod formatter;
pub mod parser;
pub mod structures;
pub mod toml_builder;

use std::{
    collections::{
        HashMap,
        HashSet,
    },
    env,
};

use crate::parser::ParserError;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please pass name of the file as argument");
        return
    }

    std::process::exit(match run(&args[1]) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn run(path: &String) -> Result<(), parser::ParserError> {
    // read the file
    let content = file_utils::read_file(path)?;
    let mut chars = content.chars();
    let mut imports = HashSet::new();
    let mut storage = HashMap::new();
    let mut functions = HashMap::new();
    let mut events = HashMap::new();
    let mut modifiers = HashMap::new();
    let mut structs = HashMap::new();

    let mut parser = parser::Parser::new(
        &mut chars,
        &mut imports,
        &mut storage,
        &mut functions,
        &mut events,
        &mut modifiers,
        &mut structs,
    );
    let output = parser.parse_file()?;
    match output {
        (None, None) | (Some(_), Some(_)) => Err(ParserError::FileCorrupted),
        (Some(contract), None) => {
            let ink_contract = assembler::assemble_contract(contract);
            let file_name = path.replace(".sol", "");
            file_utils::write_file(ink_contract, Some(file_name))?;
            println!("File saved!");
            Ok(())
        }
        (None, Some(interface)) => {
            let ink_trait = assembler::assemble_interface(interface);
            let file_name = path.replace(".sol", "");
            file_utils::write_file(ink_trait, Some(file_name))?;
            println!("File saved!");
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::run;

    #[test]
    fn erc20() {
        assert_eq!(
            run(&"examples/contracts/ERC20/ERC20.sol".to_string()),
            Ok(())
        );
    }

    #[test]
    fn erc721() {
        assert_eq!(
            run(&"examples/contracts/ERC721/ERC721.sol".to_string()),
            Ok(())
        );
    }

    #[test]
    fn erc1155() {
        assert_eq!(
            run(&"examples/contracts/ERC1155/ERC1155.sol".to_string()),
            Ok(())
        );
    }

    #[test]
    fn access_control() {
        assert_eq!(
            run(&"examples/contracts/AccessControl/AccessControl.sol".to_string()),
            Ok(())
        );
    }

    #[test]
    fn solang_example() {
        assert_eq!(
            run(&"examples/contracts/SolangExample/example.sol".to_string()),
            Ok(())
        );
    }

    #[test]
    fn flipper() {
        assert_eq!(
            run(&"examples/contracts/Flipper/flipper.sol".to_string()),
            Ok(())
        );
    }

    #[test]
    fn primitives() {
        assert_eq!(
            run(&"examples/contracts/Primitives/Primitives.sol".to_string()),
            Ok(())
        );
    }

    #[test]
    fn ierc20() {
        assert_eq!(
            run(&"examples/interfaces/IERC20/IERC20.sol".to_string()),
            Ok(())
        );
    }

    #[test]
    fn ierc721() {
        assert_eq!(
            run(&"examples/interfaces/IERC721/IERC721.sol".to_string()),
            Ok(())
        );
    }

    #[test]
    fn ierc1155() {
        assert_eq!(
            run(&"examples/interfaces/IERC1155/IERC1155.sol".to_string()),
            Ok(())
        );
    }

    #[test]
    fn iaccess_control() {
        assert_eq!(
            run(&"examples/interfaces/IAccessControl/IAccessControl.sol".to_string()),
            Ok(())
        );
    }
}
