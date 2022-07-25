#![feature(once_cell)]
#![feature(string_remove_matches)]
#![feature(exclusive_range_pattern)]

pub mod assembler;
pub mod file_utils;
pub mod formatter;
pub mod parser;
pub mod structures;

use std::env;

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

    let output = parser::parse_file(content)?;
    match output {
        (None, None) | (Some(_), Some(_)) => return Err(ParserError::FileCorrupted),
        (Some(contract), None) => {
            let ink_contract = assembler::assemble_contract(contract);
            let file_name = path.replace(".sol", ".rs");
            file_utils::write_file(ink_contract, Some(file_name))?;
            println!("File saved!");
            return Ok(())
        }
        (None, Some(interface)) => {
            let ink_trait = assembler::assemble_interface(interface);
            let file_name = path.replace(".sol", ".rs");
            file_utils::write_file(ink_trait, Some(file_name))?;
            println!("File saved!");
            return Ok(())
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
