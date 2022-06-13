use crate::file_utils;
use convert_case::{
    Case,
    Casing,
};
use std::{
    collections::HashSet,
    lazy::Lazy,
};
use substring::Substring;

#[derive(Debug, PartialEq, Default)]
enum ContractType {
    #[default]
    INTERFACE,
    CONTRACT,
}

#[derive(Debug, PartialEq)]
enum ArgsReader {
    ARGTYPE,
    ARGNAME,
}

/// `contract_name` the name of the contract
/// `next_line` n-th line where we found contract definition
/// `contract_type` type of parsed contract (contract/interface)
#[derive(Debug, Default)]
struct ContractDefinition {
    pub contract_name: String,
    pub next_line: usize,
    pub contract_type: ContractType,
}

#[derive(Debug)]
pub enum ParserError {
    ContractsParsingNotImplemented,
    FileError(std::io::Error),
    NoContractDefinitionFound,
}

impl From<std::io::Error> for ParserError {
    fn from(error: std::io::Error) -> Self {
        ParserError::FileError(error)
    }
}

struct Interface {
    imports: HashSet<String>,
    functions: Vec<String>,
}

/// Function which will run the parser
pub fn run(path: &String) -> Result<(), ParserError> {
    // read the file
    let content = file_utils::read_file(path)?;

    let lines: Vec<String> = content.split('\n').map(|s| s.trim().to_owned()).collect();

    // we skip all lines until the contract definition
    let contract_definition: ContractDefinition = get_contract_definition(&lines)?;

    match contract_definition.contract_type {
        ContractType::INTERFACE => {
            let (mut interface, imports) = {
                let int = parse_interface(contract_definition, lines);
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
        ContractType::CONTRACT => Err(ParserError::ContractsParsingNotImplemented),
    }
}

/// This function will build the code of a ink! interface (trait) without function bodies
///
/// `contract_definition` the definition of the interfacet
/// `lines` the solidity code of the interface
fn parse_interface(contract_definition: ContractDefinition, lines: Vec<String>) -> Interface {
    let name = contract_definition
        .contract_name
        .substring(1, contract_definition.contract_name.len());

    let mut output = Vec::<String>::new();
    let mut non_trait = Vec::<String>::new();
    let mut trait_def = Vec::<String>::new();
    let mut imports = HashSet::<String>::new();

    let mut i = contract_definition.next_line;
    let mut search_semicolon = false;
    // read body of contract
    while i < lines.len() {
        let mut line = lines[i].trim().to_owned();
        if !line.contains(";") && line.substring(0, 5) != "event" {
            search_semicolon = true;
        }
        if line.is_empty() || line.chars().nth(0).unwrap() == '/' || line.chars().nth(0).unwrap() == '*' {
            i += 1;
            continue
        } else if search_semicolon && line.contains(";") {
            search_semicolon = false;
            i += 1;
            continue
        } else if line.substring(0, 4) == "enum" {
            let mut enum_defintion = parse_enum(&line);
            non_trait.append(enum_defintion.as_mut());
            i += 1;
            continue
        } else if line.substring(0, 5) == "event" {
            let (mut event_definition, event_imports) = parse_event(&line);
            non_trait.append(event_definition.as_mut());
            for import in event_imports.iter() {
                imports.insert(import.to_owned());
            }
            i += 1;
            continue
        } else if line == "}" {
            break
        }
        while line.chars().nth(line.len() - 1).unwrap() != ';' {
            i += 1;
            let next = lines[i].trim();
            // some nice formatting
            let new = line.to_owned()
                + if line.contains(')') || line.contains(',') {
                    " "
                } else {
                    ""
                }
                + next;
            line = new;
        }
        let tokens: Vec<String> = line.replace(" )", ")").split('(').map(|s| s.to_owned()).collect();
        let (mut function, function_imports) = parse_interface_function_header(tokens);
        trait_def.append(function.as_mut());
        for import in function_imports.iter() {
            imports.insert(import.to_owned());
        }
        i += 1;
    }

    output.push(format!("#[brush::wrapper] \npub type {0}Ref = dyn {0};", name));
    // we add events and enums
    output.append(non_trait.as_mut());
    output.push(String::from("#[brush::trait_definition]\n"));
    output.push(format!("pub trait {} {{\n", name));
    // add the trait
    output.append(trait_def.as_mut());
    // end of body
    output.push(String::from("}"));

    Interface {
        imports,
        functions: output,
    }
}

/// This function parses enum from one-liner enum
///
/// `line` the line of enum from solidity
/// returns the enum definition in ink!
fn parse_enum(line: &String) -> Vec<String> {
    let tokens: Vec<String> = line.split(' ').map(|s| s.to_owned()).collect();
    let mut out = Vec::<String>::new();

    out.push(String::from("enum "));
    out.push(tokens[1].to_owned());
    for i in 2..tokens.len() {
        let token = tokens[i].to_owned();
        if token.chars().nth(0).unwrap() == '{' {
            out.push(token.substring(1, token.len() - 2).to_owned());
        } else {
            out.push(token.substring(0, token.len() - 1).to_owned());
        }
    }
    out
}

/// This function parses event
///
/// `line` the Solidity event definition
///
/// returns the event definition in ink! along with imports needed by this event
fn parse_event(line: &String) -> (Vec<String>, HashSet<String>) {
    let tokens: Vec<String> = line.split(' ').map(|s| s.to_owned()).collect();

    let mut out = Vec::<String>::new();
    let mut event_def = Vec::<String>::new();
    let mut imports = HashSet::<String>::new();

    let mut args_reader = ArgsReader::ARGNAME;
    let mut is_indexed = false;
    // we assume Approval(address, didnt get split by white space
    let split_brace: Vec<String> = tokens[1].split('(').map(|s| s.to_owned()).collect();
    let event_name = split_brace[0].to_owned();
    let mut arg_type = convert_argument_type(split_brace[1].to_owned(), &mut imports);

    for i in 2..tokens.len() {
        let mut token = tokens[i].to_owned();
        if args_reader == ArgsReader::ARGTYPE {
            arg_type = convert_argument_type(token, &mut imports);
            args_reader = ArgsReader::ARGNAME;
        } else if token == "indexed" {
            is_indexed = true;
            continue
        } else {
            token.remove_matches(&[',', ')', ';'][..]);
            event_def.push(format!(
                "{}{} : {},",
                if is_indexed { "#[ink(topic)]\n" } else { "" },
                token,
                arg_type
            ));
            is_indexed = false;
            args_reader = ArgsReader::ARGTYPE;
        }
    }

    out.push(format!("#[ink(event)] \n pub struct {} {{", event_name));
    out.append(event_def.as_mut());
    out.push(String::from("}"));

    (out, imports)
}

/// Parses interface function definition into ink! trait
///
/// function coming here is split by '(' therefore we get
/// 1. the function definition { function fn( }
/// 2. the function arguments { address account) public view returns ( }
/// 3. the function return parameters { uint, uint) }
/// since a function may not be a return function, the length of this vec can be 2
/// when the length of this vec is 3, we know it is a return function
///
/// `tokens` solidity function definition split by '('
///
/// returns the trait definition in ink! and a set of imports needed by this function
fn parse_interface_function_header(tokens: Vec<String>) -> (Vec<String>, HashSet<String>) {
    let mut function = Vec::<String>::new();
    let mut imports = HashSet::<String>::new();

    let is_return = tokens.len() == 3;
    let left: Vec<String> = tokens[0].split(' ').map(|s| s.to_owned()).collect();
    let right: Vec<String> = tokens[1].split(' ').map(|s| s.to_owned()).collect();
    let has_args = right[0] != ")";

    let fn_name = &left[1].to_case(Case::Snake);
    let mut is_mut = true;

    for i in 0..right.len() {
        if right[i].trim() == "view" {
            is_mut = false;
            break
        }
    }

    function.push(format!(
        "#[ink(message)] \nfn {} (&{}self{}",
        fn_name,
        if is_mut { "mut " } else { "" },
        if has_args { "," } else { "" }
    ));

    if has_args {
        let (mut args, args_imports) = parse_args(right.as_ref());
        function.append(args.as_mut());
        for import in args_imports.iter() {
            imports.insert(import.to_owned());
        }
    }

    function.push(String::from(")"));

    if is_return {
        function.push(String::from(" -> "));
        let input: Vec<String> = tokens[2].split(' ').map(|s| s.to_owned()).collect();
        let (mut params, params_imports) = parse_return_params(&input);
        function.append(params.as_mut());
        for import in params_imports.iter() {
            imports.insert(import.to_owned());
        }
    }
    function.push(String::from(";"));

    (function, imports)
}

/// Parses return paramaters of a function
///
/// `right` the right side of the function
///
/// returns the return parameters of the function and a set of imports needed
fn parse_return_params(right: &Vec<String>) -> (Vec<String>, HashSet<String>) {
    let mut args = Vec::<String>::new();
    let mut imports = HashSet::<String>::new();
    let mut arg_type: String;
    let params: Vec<String> = right
        .iter()
        .map(|x| x.to_owned() + " ")
        .collect::<String>()
        .split(',')
        .map(|s| s.to_owned())
        .collect();

    if params.len() > 1 {
        args.push(String::from("("));
    }
    for j in 0..params.len() {
        let end = j == params.len() - 1;

        arg_type = params[j]
            .trim()
            .split(' ')
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()[0]
            .to_owned();
        if arg_type.chars().nth(arg_type.len() - 1).unwrap() == ';' {
            arg_type = arg_type.substring(0, arg_type.len() - 2).to_owned();
        }

        arg_type = convert_argument_type(arg_type, &mut imports);

        if end {
            args.push(arg_type);
        } else {
            args.push(format!("{}, ", arg_type));
        }
    }
    if params.len() > 1 {
        args.push(String::from(")"));
    }

    (args, imports)
}

/// Parses arguments of a function
///
/// `right` the right side of the function
///
/// returns the return parameters of the function and a set of imports needed
fn parse_args(right: &Vec<String>) -> (Vec<String>, HashSet<String>) {
    let mut args = Vec::<String>::new();
    let mut imports = HashSet::<String>::new();

    let mut mode = ArgsReader::ARGNAME;
    let mut arg_type = right[0].to_owned();

    for j in 1..right.len() {
        if mode == ArgsReader::ARGTYPE {
            arg_type = right[j].to_owned();
            mode = ArgsReader::ARGNAME;
        } else if mode == ArgsReader::ARGNAME {
            let mut end = false;
            let mut arg_name = right[j].to_case(Case::Snake);

            if arg_name == "memory" || arg_name == "calldata" {
                continue
            } else if arg_name.chars().nth(arg_name.len() - 1).unwrap() == ')' {
                arg_name = arg_name.substring(0, arg_name.len() - 1).to_owned();
                end = true;
            } else if arg_name.chars().nth(arg_name.len() - 1).unwrap() == ',' {
                arg_name = arg_name.substring(0, arg_name.len() - 1).to_owned();
            }

            arg_type = convert_argument_type(arg_type, &mut imports);

            if end {
                args.push(format!("{} : {}", arg_name, arg_type));
                break
            } else {
                args.push(format!("{} : {},", arg_name, arg_type));
            }
            mode = ArgsReader::ARGTYPE;
        }
    }
    (args, imports)
}

/// this function looks for `contract Contract` or `interface Interface` definition in solidity file
///
/// `lines` the lines of original solidity code
///
/// returns `NoContractDefinitionFound` if no definition of a contract nor interface was found
/// return the definition of the contract
fn get_contract_definition(lines: &Vec<String>) -> Result<ContractDefinition, ParserError> {
    for i in 0..lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            continue
        }
        let tokens: Vec<String> = line.split(' ').map(|s| s.to_owned()).collect();
        let contract_name = Lazy::new(|| tokens[1].to_owned());
        if tokens[0] == "interface" {
            return Ok(ContractDefinition {
                contract_name: contract_name.to_owned(),
                next_line: i + 1,
                contract_type: ContractType::INTERFACE,
            })
        } else if tokens[0] == "contract" {
            return Ok(ContractDefinition {
                contract_name: contract_name.to_owned(),
                next_line: i + 1,
                contract_type: ContractType::CONTRACT,
            })
        }
    }
    Err(ParserError::NoContractDefinitionFound)
}

/// converts solidity argument type to ink! argument type (eg. address -> AccountId, uint -> u128)
///
/// `arg_type` solidity argument type
/// `imports` the set of imports
///
/// return the converted type
fn convert_argument_type(arg_type: String, imports: &mut HashSet<String>) -> String {
    // removes array braces from the type
    let (no_array_arg_type, is_vec) = if arg_type.substring(arg_type.len() - 2, arg_type.len()) == "[]" {
        (arg_type.substring(0, arg_type.len() - 2), true)
    } else {
        (arg_type.as_str(), false)
    };
    let output_type = match no_array_arg_type {
        "uint256" | "uint" => String::from("u128"),
        "bytes" => {
            imports.insert(String::from("use ink::prelude::vec::Vec;"));
            String::from("Vec<u8>")
        }
        "address" => {
            imports.insert(String::from("use brush::traits::AccountId;"));
            String::from("AccountId")
        }
        "bytes32" => String::from("[u8; 32]"),
        _ => arg_type,
    };
    return if is_vec {
        format!("Vec<{}>", output_type)
    } else {
        output_type
    }
}
