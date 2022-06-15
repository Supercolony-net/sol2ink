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

struct Contract {
    name: String,
    fields: Vec<ContractField>,
    constructor: Constructor,
    events: Vec<Event>,
    structs: Vec<Struct>,
    functions: Vec<Function>,
    imports: HashSet<String>,
}

struct ContractField {
    field_type: String,
    name: String,
}

struct Event {
    name: String,
    fields: Vec<EventField>,
}

struct EventField {
    indexed: bool,
    field_type: String,
    name: String,
}

struct Struct {
    name: String,
    fields: Vec<StructField>,
}

struct StructField {
    name: String,
    field_type: String,
}

struct Function {
    name: String,
    params: Vec<FunctionParam>,
    external: bool,
    view: bool,
    payable: bool,
    return_params: Vec<String>,
    body: Vec<Statement>,
}

struct Constructor {
    params: Vec<FunctionParam>,
    body: Vec<Statement>,
}

struct FunctionParam {
    name: String,
    param_type: String,
}

struct Statement {
    content: String,
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
        ContractType::CONTRACT => {
            let contract = parse_contract(contract_definition, lines);
            let ink_contract = assemble_contract(contract);
            let file_name = path.replace(".sol", ".rs");
            file_utils::write_file(&ink_contract, Some(file_name))?;
            Ok(())
        }
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

fn parse_contract(contract_definition: ContractDefinition, lines: Vec<String>) -> Contract {
    let name = contract_definition.contract_name;

    let fields = Vec::<ContractField>::new();
    let events = Vec::<Event>::new();
    let structs = Vec::<Struct>::new();
    let functions = Vec::<Function>::new();
    let imports = HashSet::<String>::new();
    let constructor = Constructor {
        params: Vec::<FunctionParam>::new(),
        body: Vec::<Statement>::new(),
    };

    let mut in_function = false;
    let mut open_braces = 0;
    let mut close_braces = 0;
    // read body of contract
    for i in contract_definition.next_line..lines.len() {
        let line = lines[i].trim().to_owned();

        if line.is_empty() {
            continue
        } else if line.chars().nth(0).unwrap() == '/' || line.chars().nth(0).unwrap() == '*' {
            // TODO parse comments
        } else if line.substring(0, 11) == "constructor" {
            // TODO parse constructor
            in_function = true;
            update_in_function(line, &mut open_braces, &mut close_braces, &mut in_function);
        } else if line.substring(0, 8) == "function" {
            // TODO parse function
            in_function = true;
            update_in_function(line, &mut open_braces, &mut close_braces, &mut in_function);
        } else if in_function {
            // TODO parse statements
            update_in_function(line, &mut open_braces, &mut close_braces, &mut in_function);
        } else if line == "}" {
            // end of contract
            continue
        } else {
            println!("{}", line);
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
fn update_in_function(line: String, open_braces: &mut usize, close_braces: &mut usize, in_function: &mut bool) {
    *open_braces += line.matches("{").count();
    *close_braces += line.matches("}").count();
    if *open_braces == *close_braces && *open_braces > 0 {
        *in_function = false;
        *open_braces = 0;
        *close_braces = 0;
    }
}

/// This function will assemble ink! contract from the parsed contract struct and save it to a file
fn assemble_contract(contract: Contract) -> Vec<String> {
    let mut output_vec = Vec::<String>::new();
    output_vec.push(String::from("#![cfg_attr(not(feature = \"std\"), no_std)]\n"));
    output_vec.push(String::from("#![feature(min_specialization)]\n\n"));
    output_vec.push(String::from("#[brush::contract]\n"));
    output_vec.push(format!("pub mod {} {{\n", contract.name.to_case(Case::Snake)));

    // imports
    append_and_tab(&mut output_vec, Vec::from_iter(contract.imports));
    output_vec.push(String::from("\n"));
    // events
    append_and_tab(&mut output_vec, assemble_events(contract.events));
    // fields
    append_and_tab(
        &mut output_vec,
        assemble_storage(contract.name.clone(), contract.fields),
    );
    // structs
    append_and_tab(&mut output_vec, assemble_structs(contract.structs));
    output_vec.push(format!("\timpl {} {{\n", contract.name));
    // constructor
    append_and_tab(&mut output_vec, assemble_constructor(contract.constructor));
    // functions
    append_and_tab(&mut output_vec, assemble_functions(contract.functions));
    output_vec.push(String::from("\t}\n"));

    output_vec.push(String::from("}\n"));
    output_vec
}

/// This function will assemble ink! events from the parsed contract
fn assemble_events(events: Vec<Event>) -> Vec<String> {
    let mut output_vec = Vec::<String>::new();

    for event in events.iter() {
        output_vec.push(String::from("#[ink(event)]\n"));
        output_vec.push(format!("pub struct {} {{\n", event.name));
        for event_field in event.fields.iter() {
            if event_field.indexed {
                output_vec.push(String::from("\t#[ink(topic)]\n"));
            }
            output_vec.push(format!("\t{}: {},\n", event_field.name, event_field.field_type));
        }
        output_vec.push(String::from("}}\n"));
    }

    output_vec
}

/// This function will assemble ink! storage from the parsed contract
fn assemble_storage(contract_name: String, fields: Vec<ContractField>) -> Vec<String> {
    let mut output_vec = Vec::<String>::new();

    output_vec.push(String::from("#[ink(storage)]\n"));
    output_vec.push(String::from("#[derive(Default, SpreadAllocate)]\n"));

    output_vec.push(format!("pub struct {}{{\n", contract_name));
    for field in fields.iter() {
        output_vec.push(format!("\t{}: {},\n", field.name, field.field_type));
    }

    output_vec.push(String::from("}\n"));
    output_vec.push(String::from("\n"));

    output_vec
}

/// This function will assemble rust structs from the parsed contract
fn assemble_structs(structs: Vec<Struct>) -> Vec<String> {
    let mut output_vec = Vec::<String>::new();

    for structure in structs.iter() {
        output_vec.push(String::from("#[derive(Default, Encode, Decode)]\n"));
        output_vec.push(String::from(
            "#[cfg_attr(feature = \"std\", derive(scale_info::TypeInfo))]\n",
        ));
        output_vec.push(format!("pub struct {} {{\n", structure.name));
        for struct_field in structure.fields.iter() {
            output_vec.push(format!("\tpub {}: {},\n", struct_field.name, struct_field.field_type));
        }
        output_vec.push(String::from("}\n"));
    }

    output_vec
}

/// This function will assemble the constructor of the ink! contract from the parsed contract
fn assemble_constructor(constructor: Constructor) -> Vec<String> {
    let mut output_vec = Vec::<String>::new();
    // we do this so we dont put tab between each string that we insert (function params)
    let mut header = String::new();

    output_vec.push(String::from("\t#[ink(constructor)]\n"));
    header.push_str("\tpub fn new(");

    for param in constructor.params.iter() {
        header.push_str(format!("{}: {},", param.name, param.param_type).as_str());
    }
    header.push_str(") -> Self {\n");
    output_vec.push(header);

    output_vec.push(String::from(
        "\t\tink_lang::codegen::initialize_contract(|instance: &mut Self| {\n",
    ));
    for statement in constructor.body.iter() {
        // TODO remove comments
        output_vec.push(format!("\t\t\t//{}\n", statement.content));
    }
    output_vec.push(String::from("\t\t})\n"));
    output_vec.push(String::from("\t}\n"));

    output_vec
}

/// This function will assemble the constructor of the ink! contract from the parsed contract
fn assemble_functions(functions: Vec<Function>) -> Vec<String> {
    let mut output_vec = Vec::<String>::new();
    // we do this so we dont put tab between each string that we insert (function params)
    let mut header: String;

    for function in functions.iter() {
        header = String::new();
        output_vec.push(format!(
            "\t#[ink(message{})]\n",
            if function.payable {
                String::from(", payable")
            } else {
                String::from("")
            }
        ));
        header.push_str(
            format!(
                "\t{}fn {}(",
                if function.external {
                    String::from("pub ")
                } else {
                    String::from("")
                },
                function.name.to_case(Case::Snake)
            )
            .as_str(),
        );
        // arguments
        header.push_str(
            format!(
                "&{}self",
                if function.view {
                    String::from("")
                } else {
                    String::from(" mut ")
                }
            )
            .as_str(),
        );
        for param in function.params.iter() {
            header.push_str(format!(",{}: {}", param.name, param.param_type).as_str());
        }
        header.push_str(")");
        // return params
        if !function.return_params.is_empty() {
            header.push_str(" -> ");
            if function.return_params.len() > 1 {
                header.push_str("(");
            }
            for i in 0..function.return_params.len() {
                header.push_str(
                    format!(
                        "{}{}",
                        if i > 0 { String::from(", ") } else { String::from("") },
                        function.return_params[i]
                    )
                    .as_str(),
                );
            }
            if function.return_params.len() > 1 {
                header.push_str(")");
            }
        }
        header.push_str("{\n");
        output_vec.push(header.to_owned());
        // body
        for statement in function.body.iter() {
            // TODO remove comments
            output_vec.push(format!("\t\t//{}\n", statement.content));
        }
        output_vec.push(String::from("\t}\n"));
        output_vec.push(String::from("\n"));
    }

    output_vec
}

/// This function will append `appended` vec of Strings to `output` vec of strings
/// and add a tab to the front
fn append_and_tab(output: &mut Vec<String>, appended: Vec<String>) {
    output.append(
        appended
            .iter()
            .map(|string| "\t".to_string() + string)
            .collect::<Vec<String>>()
            .as_mut(),
    );
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
            imports.insert(String::from("use ink::prelude::vec::Vec;\n"));
            String::from("Vec<u8>")
        }
        "address" => {
            imports.insert(String::from("use brush::traits::AccountId;\n"));
            String::from("AccountId")
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
