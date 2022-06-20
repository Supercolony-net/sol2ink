use crate::structures::*;
use convert_case::{
    Case,
    Casing,
};
use std::{
    collections::HashSet,
    lazy::Lazy,
};
use substring::Substring;

#[derive(Debug, PartialEq)]
enum ArgsReader {
    ARGTYPE,
    ARGNAME,
}

#[derive(Debug, PartialEq)]
enum FunctionReader {
    FUNCTION,
    CONSTRUCTOR,
    NONE,
}

#[derive(Debug)]
pub enum ParserError {
    FileError(std::io::Error),
    NoContractDefinitionFound,
}

impl From<std::io::Error> for ParserError {
    fn from(error: std::io::Error) -> Self {
        ParserError::FileError(error)
    }
}

/// This function will build the code of a ink! interface (trait) without function bodies
///
/// `contract_definition` the definition of the interfacet
/// `lines` the solidity code of the interface
pub fn parse_interface(contract_definition: ContractDefinition, lines: Vec<String>) -> Interface {
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
        if line.is_empty()
            || line.chars().nth(0).unwrap() == '/'
            || line.chars().nth(0).unwrap() == '*'
        {
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
        let tokens: Vec<String> = line
            .replace(" )", ")")
            .split('(')
            .map(|s| s.to_owned())
            .collect();
        let (mut function, function_imports) = parse_interface_function_header(tokens);
        trait_def.append(function.as_mut());
        for import in function_imports.iter() {
            imports.insert(import.to_owned());
        }
        i += 1;
    }

    output.push(format!(
        "#[brush::wrapper] \npub type {0}Ref = dyn {0};",
        name
    ));
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

pub fn parse_contract(contract_definition: ContractDefinition, lines: Vec<String>) -> Contract {
    let name = contract_definition.contract_name;

    let mut fields = Vec::<ContractField>::new();
    let events = Vec::<Event>::new();
    let structs = Vec::<Struct>::new();
    let mut functions = Vec::<Function>::new();
    let mut statements = Vec::<Statement>::new();
    let mut imports = HashSet::<String>::new();
    let mut constructor = Function::default();

    let mut function_reader = FunctionReader::NONE;
    let mut function_header = FunctionHeader::default();
    let mut open_braces = 0;
    let mut close_braces = 0;
    let mut buffer = String::new();
    // read body of contract
    for i in contract_definition.next_line..lines.len() {
        let line = lines[i].trim().to_owned();

        if line.is_empty() {
            continue
        } else if line.chars().nth(0).unwrap() == '/' || line.chars().nth(0).unwrap() == '*' {
            // TODO parse comments
        } else if line.substring(0, 11) == "constructor" {
            // TODO parse constructor
            function_reader = FunctionReader::CONSTRUCTOR;
            if line.contains("{") {
                statements = Vec::<Statement>::new();
                function_header = parse_function_header(line.clone(), &mut imports);
            } else {
                buffer = line.to_owned();
            }
            let function_maybe = update_in_function(
                line,
                &mut open_braces,
                &mut close_braces,
                &mut function_reader,
                Some(function_header.clone()),
                Some(statements.clone()),
            );
            if function_maybe.is_some() {
                constructor = function_maybe.unwrap();
            }
        } else if line.substring(0, 8) == "function" {
            function_reader = FunctionReader::FUNCTION;
            if line.contains("{") {
                statements = Vec::<Statement>::new();
                function_header = parse_function_header(line.clone(), &mut imports);
            } else {
                buffer = line.to_owned();
            }
            let function_maybe = update_in_function(
                line,
                &mut open_braces,
                &mut close_braces,
                &mut function_reader,
                Some(function_header.clone()),
                Some(statements.clone()),
            );
            if function_maybe.is_some() {
                functions.push(function_maybe.unwrap());
            }
        } else if line.substring(0, 5) == "event" {
            // TODO parse event
        } else if line.substring(0, 6) == "struct" {
            // TODO parse struct
        } else if function_reader != FunctionReader::NONE {
            if open_braces == 0 && line.contains("{") {
                buffer.push_str(line.as_str());
                buffer = buffer.replace(",", ", ");
                function_header = parse_function_header(buffer.clone(), &mut imports);
                statements = Vec::<Statement>::new();
            } else if open_braces == 0 {
                buffer.push_str(line.as_str());
            } else if !line.contains("}") || open_braces != close_braces + 1 {
                statements.push(parse_statement(line.clone()));
            }
            let function_maybe = update_in_function(
                line,
                &mut open_braces,
                &mut close_braces,
                &mut function_reader,
                Some(function_header.clone()),
                Some(statements.clone()),
            );
            if function_maybe.is_some() {
                let function = function_maybe.unwrap();
                if function.cosntructor {
                    constructor = function;
                } else {
                    functions.push(function);
                }
            }
        } else if line == "}" {
            // end of contract
            continue
        } else {
            fields.push(parse_contract_field(line, &mut imports));
        }
    }

    Contract {
        name,
        fields,
        constructor,
        events,
        structs,
        functions,
        imports,
    }
}

/// This function updates the count of opne and close braces and ends reading of function if conditions are met
fn update_in_function(
    line: String,
    open_braces: &mut usize,
    close_braces: &mut usize,
    function_reader: &mut FunctionReader,
    header: Option<FunctionHeader>,
    statements: Option<Vec<Statement>>,
) -> Option<Function> {
    *open_braces += line.matches("{").count();
    *close_braces += line.matches("}").count();
    let mut output = None;
    if *open_braces == *close_braces && *open_braces > 0 {
        if header.is_some() {
            output = Some(Function {
                header: header.unwrap(),
                cosntructor: *function_reader == FunctionReader::CONSTRUCTOR,
                body: statements.unwrap(),
            });
        }
        *function_reader = FunctionReader::NONE;
        *open_braces = 0;
        *close_braces = 0;
    }
    output
}

/// This function parses the field of a contract represented by `line`
/// and adds imports to `imports`
///
/// returns the representation of contract field in `ContractField` struct
fn parse_contract_field(line: String, imports: &mut HashSet<String>) -> ContractField {
    // most mappings are written as `type => type`
    // we will make it `type=>type`
    let tokens = line
        .replace(" => ", "=>")
        .split(" ")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    let name_index = if tokens.len() > 2 { 2 } else { 1 };

    let field_type = convert_variable_type(tokens[0].to_owned(), imports);
    let name = tokens[name_index]
        .substring(0, tokens[name_index].len() - 1)
        .to_owned()
        .to_case(Case::Snake);
    ContractField { field_type, name }
}

/// This function parses the function header represented by `line`
/// and adds imports to `imports`
///
/// returns the representation of the function `Function` struct
fn parse_function_header(line: String, imports: &mut HashSet<String>) -> FunctionHeader {
    let split_by_left_brace = line
        .split("(")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    let name = split_by_left_brace[0]
        .substring(9, split_by_left_brace[0].len())
        .to_case(Case::Snake);
    let split_by_right_brace = split_by_left_brace[1]
        .split(")")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    let params_raw = split_by_right_brace[0].to_owned();
    let params = parse_function_parameters(params_raw, imports);
    let attributes_raw = split_by_right_brace[1].to_owned();
    let (external, view, payable) = parse_function_attributes(attributes_raw);

    let return_params = if split_by_left_brace.len() == 3 {
        parse_return_parameters(
            split_by_left_brace[2]
                .split(")")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()[0]
                .to_owned(),
            imports,
        )
    } else {
        Vec::<String>::new()
    };

    // TODO parse statements
    FunctionHeader {
        name,
        params,
        external,
        view,
        payable,
        return_params,
    }
}

/// This function parses the statement in `line` into rust statement
///
/// TODO: for now we only return the original statement and comment it
///
/// returns the statement as `Statement` struct
fn parse_statement(line: String) -> Statement {
    // TODO actual parsing
    Statement { content: line }
}

/// Parses parameters of a function
///
/// `parameters` the String which contains the paramters of the function
/// `imports` the Set of imports of the contract
///
/// returns the vec of function parameters of this function
fn parse_function_parameters(
    parameters: String,
    imports: &mut HashSet<String>,
) -> Vec<FunctionParam> {
    let mut out = Vec::<FunctionParam>::new();

    if !parameters.is_empty() {
        let tokens = parameters
            .split(" ")
            .map(|s| {
                let mut out = s.to_owned();
                out.remove_matches(",");
                out
            })
            .collect::<Vec<String>>();
        let mut mode = ArgsReader::ARGNAME;
        let mut param_type = convert_variable_type(tokens[0].to_owned(), imports);

        for j in 1..tokens.len() {
            if mode == ArgsReader::ARGTYPE {
                param_type = convert_variable_type(tokens[j].to_owned(), imports);
                mode = ArgsReader::ARGNAME;
            } else if mode == ArgsReader::ARGNAME {
                let name = tokens[j].to_case(Case::Snake);

                if name == "memory" || name == "calldata" {
                    continue
                }

                out.push(FunctionParam {
                    name,
                    param_type: param_type.to_owned(),
                });
                mode = ArgsReader::ARGTYPE;
            }
        }
    }

    out
}

/// Parses attributes of a function like payable, external, view
///
/// `attributes` the String which contains the attributes of the function
///
/// returns 0. external 1. view 2. payable
fn parse_function_attributes(attributes: String) -> (bool, bool, bool) {
    let mut external = false;
    let mut view = false;
    let mut payable = false;

    let tokens = attributes
        .split(" ")
        .map(|s| {
            let mut out = s.to_owned();
            out.remove_matches(",");
            out
        })
        .collect::<Vec<String>>();

    for i in 0..tokens.len() {
        let attribute = tokens[i].to_case(Case::Snake);
        if attribute == "external" || attribute == "public" {
            external = true;
        } else if attribute == "view" {
            view = true;
        } else if attribute == "payable" {
            payable = true;
        }
    }

    (external, view, payable)
}

/// Parses return parameters of a function
///
/// `parameters` the String which contains the return paramters of the function
/// `imports` the Set of imports of the contract
///
/// returns the vec of function return parameters of this function
fn parse_return_parameters(parameters: String, imports: &mut HashSet<String>) -> Vec<String> {
    let mut out = Vec::<String>::new();
    let mut updated_parameters = parameters.to_owned();
    updated_parameters.remove_matches(" memory");
    updated_parameters.remove_matches(" calldata");
    let tokens: Vec<String> = updated_parameters
        .split(' ')
        .map(|s| s.to_owned())
        .collect();

    for i in 0..tokens.len() {
        if i % 2 == 1 && tokens.len() >= (parameters.matches(",").count() + 1) * 2 {
            continue
        }
        let mut param = tokens[i].to_owned();
        param.remove_matches(",");
        out.push(convert_variable_type(param, imports));
    }

    out
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
    let mut arg_type = convert_variable_type(split_brace[1].to_owned(), &mut imports);

    for i in 2..tokens.len() {
        let mut token = tokens[i].to_owned();
        if args_reader == ArgsReader::ARGTYPE {
            arg_type = convert_variable_type(token, &mut imports);
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
    let params: Vec<String> = right.join(" ").split(',').map(|s| s.to_owned()).collect();

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

        arg_type = convert_variable_type(arg_type, &mut imports);

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

            arg_type = convert_variable_type(arg_type, &mut imports);

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
pub fn parse_contract_definition(lines: &Vec<String>) -> Result<ContractDefinition, ParserError> {
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
fn convert_variable_type(arg_type: String, imports: &mut HashSet<String>) -> String {
    // removes array braces from the type
    let (no_array_arg_type, is_vec) =
        if arg_type.substring(arg_type.len() - 2, arg_type.len()) == "[]" {
            (arg_type.substring(0, arg_type.len() - 2), true)
        } else {
            (arg_type.as_str(), false)
        };
    if arg_type.substring(0, 7) == "mapping" {
        imports.insert(String::from("use ink_storage::Mapping;\n"));
        let type_args = arg_type
            .substring(8, arg_type.len() - 1)
            .split("=>")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let to = convert_variable_type(
            {
                let mut arg_type_no_braces = type_args.last().unwrap().to_owned();
                arg_type_no_braces.remove_matches(")");
                arg_type_no_braces
            },
            imports,
        );
        let mut from_vec: Vec<String> =
            vec![convert_variable_type(type_args[0].to_owned(), imports)];
        for i in 1..type_args.len() - 1 {
            from_vec.push(convert_variable_type(
                type_args[i].substring(8, type_args[i].len()).to_owned(),
                imports,
            ));
        }
        let from = if from_vec.len() > 1 {
            format!("({})", from_vec.join(", "))
        } else {
            from_vec[0].to_owned()
        };
        return format!("Mapping<{}, {}>", from, to)
    }
    let output_type = match no_array_arg_type {
        "uint8" => String::from("u8"),
        "uint256" | "uint" => String::from("u128"),
        "bytes" => {
            imports.insert(String::from("use ink::prelude::vec::Vec;\n"));
            String::from("Vec<u8>")
        }
        "address" => {
            imports.insert(String::from("use brush::traits::AccountId;\n"));
            String::from("AccountId")
        }
        "string" => {
            imports.insert(String::from("use ink::prelude::string::String;\n"));
            String::from("String")
        }
        "bytes32" => String::from("[u8; 32]"),
        _ => arg_type,
    };
    return if is_vec {
        imports.insert(String::from("use ink::prelude::vec::Vec;\n"));
        format!("Vec<{}>", output_type)
    } else {
        output_type
    }
}
