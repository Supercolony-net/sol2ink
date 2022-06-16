#![feature(once_cell)]
#![feature(string_remove_matches)]

pub mod assembler;
pub mod file_utils;
pub mod formatter;
pub mod parser;
pub mod structures;

use std::env;

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

    let lines: Vec<String> = content.split('\n').map(|s| s.trim().to_owned()).collect();

    // we skip all lines until the contract definition
    let contract_definition: structures::ContractDefinition =
        parser::parse_contract_definition(&lines)?;

    match contract_definition.contract_type {
        structures::ContractType::INTERFACE => {
            // TODO make it pretty
            let (mut interface, imports) = {
                let int = parser::parse_interface(contract_definition, lines);
                (int.functions, int.imports)
            };
            let mut output_vec = Vec::from_iter(imports);
            output_vec.append(interface.as_mut());
            output_vec = output_vec
                .iter()
                .map(|line| {
                    if line.contains(";") {
                        line.to_owned() + "\n"
                    } else {
                        line.to_owned()
                    }
                })
                .collect();
            let file_name = path.replace(".sol", ".rs");
            file_utils::write_file(&output_vec, Some(file_name))?;
            println!("File saved!");
            Ok(())
        }
        structures::ContractType::CONTRACT => {
            let contract = parser::parse_contract(contract_definition, lines);
            let ink_contract = assembler::assemble_contract(contract);
            let file_name = path.replace(".sol", ".rs");
            file_utils::write_file(&ink_contract, Some(file_name))?;
            Ok(())
        }
    }
}
