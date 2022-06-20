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
    let name = contract_definition.contract_name;

    let mut events = Vec::<Event>::new();
    let mut enums = Vec::<Enum>::new();
    let mut structs = Vec::<Struct>::new();
    let mut function_headers = Vec::<FunctionHeader>::new();
    let mut imports = HashSet::<String>::new();

    let mut function_reader = FunctionReader::NONE;
    let mut struct_name: Option<String> = None;
    let mut struct_fields = Vec::<StructField>::new();
    let mut buffer = String::new();
    // read body of contract
    for i in contract_definition.next_line..lines.len() {
        let line = lines[i].trim().to_owned();

        if line.is_empty() {
            continue
        } else if line.chars().nth(0).unwrap() == '/' || line.chars().nth(0).unwrap() == '*' {
            // TODO parse comments
        } else if line.substring(0, 8) == "function" {
            function_reader = FunctionReader::FUNCTION;
            if line.contains(";") {
                function_headers.push(parse_function_header(line.clone(), &mut imports));
            } else {
                buffer = line.to_owned();
                function_reader = FunctionReader::FUNCTION;
            }
        } else if line.substring(0, 5) == "event" {
            events.push(parse_event(line, &mut imports));
        } else if line.substring(0, 4) == "enum" {
            enums.push(parse_enum(line));
        } else if line.substring(0, 6) == "struct" {
            struct_name = Some(parse_struct_name(line));
        } else if struct_name.is_some() {
            if line == "}" {
                structs.push(Struct {
                    name: struct_name.unwrap(),
                    fields: struct_fields,
                });
                struct_name = None;
                struct_fields = Vec::<StructField>::new();
            } else {
                struct_fields.push(parse_struct_field(line, &mut imports));
            }
        } else if function_reader != FunctionReader::NONE {
            if line.contains(";") {
                buffer.push_str(line.as_str());
                buffer = buffer.replace(",", ", ");
                function_headers.push(parse_function_header(buffer.clone(), &mut imports));
                function_reader = FunctionReader::NONE;
            } else {
                buffer.push_str(line.as_str());
            }
        } else if line == "}" {
            // end of contract
            continue
        }
    }

    Interface {
        name,
        events,
        enums,
        structs,
        function_headers,
        imports,
    }
}

pub fn parse_contract(contract_definition: ContractDefinition, lines: Vec<String>) -> Contract {
    let name = contract_definition.contract_name;

    let mut fields = Vec::<ContractField>::new();
    let mut events = Vec::<Event>::new();
    let mut enums = Vec::<Enum>::new();
    let mut structs = Vec::<Struct>::new();
    let mut functions = Vec::<Function>::new();
    let mut statements = Vec::<Statement>::new();
    let mut imports = HashSet::<String>::new();
    let mut constructor = Function::default();

    let mut function_reader = FunctionReader::NONE;
    let mut struct_name: Option<String> = None;
    let mut struct_fields = Vec::<StructField>::new();
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
            events.push(parse_event(line, &mut imports));
        } else if line.substring(0, 4) == "enum" {
            enums.push(parse_enum(line));
        } else if line.substring(0, 6) == "struct" {
            struct_name = Some(parse_struct_name(line));
        } else if struct_name.is_some() {
            if line == "}" {
                structs.push(Struct {
                    name: struct_name.unwrap(),
                    fields: struct_fields,
                });
                struct_name = None;
                struct_fields = Vec::<StructField>::new();
            } else {
                struct_fields.push(parse_struct_field(line, &mut imports));
            }
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
        enums,
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

/// This function parses event
///
/// `line` the Solidity event definition
///
/// returns the event definition in ink! along with imports needed by this event
fn parse_event(line: String, imports: &mut HashSet<String>) -> Event {
    let tokens: Vec<String> = line.split(' ').map(|s| s.to_owned()).collect();

    let mut args_reader = ArgsReader::ARGNAME;
    let mut indexed = false;
    // we assume Approval(address, didnt get split by white space
    let split_brace: Vec<String> = tokens[1].split('(').map(|s| s.to_owned()).collect();
    let name = split_brace[0].to_owned();
    let mut field_type = convert_variable_type(split_brace[1].to_owned(), imports);
    let mut fields = Vec::<EventField>::new();

    for i in 2..tokens.len() {
        let mut token = tokens[i].to_owned();
        if args_reader == ArgsReader::ARGTYPE {
            field_type = convert_variable_type(token, imports);
            args_reader = ArgsReader::ARGNAME;
        } else if token == "indexed" {
            indexed = true;
            continue
        } else {
            token.remove_matches(&[',', ')', ';'][..]);
            fields.push(EventField {
                indexed,
                field_type: field_type.to_owned(),
                name: name.to_owned(),
            });
            indexed = false;
            args_reader = ArgsReader::ARGTYPE;
        }
    }

    Event { name, fields }
}

/// This function parses struct name
///
/// `line` the Solidity struct definition
///
/// returns the struct name
fn parse_struct_name(line: String) -> String {
    let tokens = line
        .split(" ")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    tokens[1].to_owned()
}

/// This function parses struct fields
///
/// `line` the Solidity definition of the struct fields
///
/// returns the field in form of `StructField` struct
fn parse_struct_field(line: String, imports: &mut HashSet<String>) -> StructField {
    let tokens = line
        .split(" ")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    let field_type = convert_variable_type(tokens[0].to_owned(), imports);
    let mut name = tokens[1].to_case(Case::Snake);
    name.remove_matches(";");
    StructField { name, field_type }
}

/// This function parses enum
///
/// `line` the Solidity definition of enum
///
/// returns the enum in form of `Enum` struct
fn parse_enum(line: String) -> Enum {
    let tokens: Vec<String> = line.split(' ').map(|s| s.to_owned()).collect();
    let name = tokens[1].to_owned();
    let mut values = Vec::<String>::new();

    for i in 2..tokens.len() {
        let mut token = tokens[i].to_owned();
        if token == "{" || token == "}" {
            continue
        } else {
            token.remove_matches(",");
            values.push(token);
        }
    }
    Enum { name, values }
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
