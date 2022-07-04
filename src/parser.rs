use crate::{
    formatter::*,
    structures::*,
};
use convert_case::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{
        HashMap,
        HashSet,
        VecDeque,
    },
    str::Chars,
};
use substring::Substring;

#[derive(Debug, PartialEq)]
enum ArgsReader {
    ARGTYPE,
    ARGNAME,
}

const DEFAULT_ERROR: &str = "SMART CONTRACTZ MAKE PANIC BEEP BEEP BEEP";

lazy_static! {
    static ref TYPES: HashMap<String, (String, Option<String>)> = {
        let mut map = HashMap::new();
        map.insert(
            "mapping".to_string(),
            ("Mapping".to_string(), Some("ink_storage".to_string())),
        );
        map.insert("uint".to_string(), ("u128".to_string(), None));
        map.insert("uint256".to_string(), ("u128".to_string(), None));
        map.insert(
            "address".to_string(),
            ("AccountId".to_string(), Some("brush::traits".to_string())),
        );
        map
    };
    static ref OPERATIONS: HashMap<String, Operation> = {
        let mut map = HashMap::new();
        map.insert(String::from("!"), Operation::Not);
        map.insert(String::from(">="), Operation::GreaterThanEqual);
        map.insert(String::from("<="), Operation::LessThanEqual);
        map.insert(String::from(">"), Operation::GreaterThan);
        map.insert(String::from("<"), Operation::LessThan);
        map.insert(String::from("=="), Operation::Equal);
        map.insert(String::from("!="), Operation::NotEqual);
        map
    };
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParserError {
    FileError(String),
    FileCorrupted,
}

impl From<std::io::Error> for ParserError {
    fn from(error: std::io::Error) -> Self {
        ParserError::FileError(error.to_string())
    }
}

#[derive(Eq, PartialEq)]
enum Action {
    None,
    ContractName,
    ContractNamed,
    Contract,
    Slash,
}

const ASTERISK: char = '*';
const BRACE_CLOSE: char = ')';
const BRACE_OPEN: char = '(';
const COMMA: char = ',';
const CURLY_CLOSE: char = '}';
const CURLY_OPEN: char = '{';
const NEW_LINE: char = '\n';
const SEMICOLON: char = ';';
const SLASH: char = '/';
const SPACE: char = ' ';
const SQUARE_CLOSE: char = ']';
const SQUARE_OPEN: char = '[';

pub fn parse_file(string: String) -> Result<(Option<Contract>, Option<Interface>), ParserError> {
    let mut chars = string.chars();
    let mut comments = Vec::<String>::new();
    let mut action = Action::None;
    let mut buffer = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            SLASH if action == Action::None => action = Action::Slash,
            SLASH if action == Action::Slash => {
                let comment = parse_comment(&mut chars);
                if comment.len() > 0 {
                    comments.push(comment);
                }
                action = Action::None;
            }
            ASTERISK if action == Action::Slash => {
                let mut new_comments = parse_multiline_comment(&mut chars);
                comments.append(&mut new_comments);
                action = Action::None;
            }
            NEW_LINE | SPACE => {}
            _ => {
                buffer.push(ch);
                if buffer == "pragma" || buffer == "import" {
                    skip(&mut chars);
                    buffer = String::new();
                } else if buffer == "contract" {
                    let contract = parse_contract(&mut chars, comments)?;
                    return Ok((Some(contract), None))
                } else if buffer == "interface" {
                    let interface = parse_interface(&mut chars, comments)?;
                    return Ok((None, Some(interface)))
                }
            }
        }
    }

    Ok((None, None))
}

fn parse_comment(chars: &mut Chars) -> String {
    let mut buffer = String::new();
    let mut reading = false;

    while let Some(ch) = chars.next() {
        match ch {
            SLASH if !reading => {
                reading = true;
            }
            NEW_LINE => return buffer.trim().to_owned(),
            _ if !reading => {
                buffer.push(ch);
                reading = true;
            }
            _ => {
                buffer.push(ch);
            }
        }
    }

    buffer
}

fn parse_multiline_comment(chars: &mut Chars) -> Vec<String> {
    let mut comments = Vec::<String>::new();
    let mut buffer = String::new();
    let mut reading = false;
    let mut new_line = false;
    let mut asterisk = false;

    while let Some(ch) = chars.next() {
        if ch == SLASH && asterisk {
            break
        } else {
            asterisk = false;
        }
        match ch {
            ASTERISK if !reading => {
                reading = true;
            }
            ASTERISK if new_line => {
                new_line = false;
            }
            NEW_LINE => {
                if buffer.trim().len() > 0 {
                    comments.push(format!(" {}", buffer.trim()));
                    buffer.clear();
                }
                new_line = true;
            }
            _ if !reading => {
                buffer.push(ch);
                reading = true;
            }
            SPACE if new_line => {}
            _ if new_line => {
                buffer.push(ch);
                new_line = false;
            }
            _ => {
                buffer.push(ch);
            }
        }
        if ch == ASTERISK {
            asterisk = true;
        }
    }

    comments
}

fn skip(chars: &mut Chars) {
    while let Some(ch) = chars.next() {
        match ch {
            SEMICOLON => return,
            _ => {}
        }
    }
}

/// Parses the code of a Solidity contract
///
/// `contract_definition` the definition of the contract
/// `lines` the solidity code of the contract
///
/// returns the representation of the contract as `Contract` struct
fn parse_contract(
    chars: &mut Chars,
    contract_comments: Vec<String>,
) -> Result<Contract, ParserError> {
    let mut buffer = String::new();
    let mut action = Action::None;

    let mut name = String::new();
    let mut comments = Vec::<String>::new();
    let mut fields = Vec::<ContractField>::new();
    let mut events = Vec::<Event>::new();
    let mut enums = Vec::<Enum>::new();
    let mut structs = Vec::<Struct>::new();
    let mut functions = Vec::<Function>::new();
    let mut imports = HashSet::<String>::new();
    let mut constructor = Function::default();

    while let Some(ch) = chars.next() {
        match ch {
            NEW_LINE if action == Action::None || action == Action::Contract => {}
            SLASH if action == Action::Contract => action = Action::Slash,
            SLASH if action == Action::Slash => {
                let comment = parse_comment(chars);
                if comment.len() > 0 {
                    comments.push(comment);
                }
                action = Action::Contract;
            }
            ASTERISK if action == Action::Slash => {
                let mut new_comments = parse_multiline_comment(chars);
                comments.append(&mut new_comments);
                action = Action::Contract;
            }
            CURLY_OPEN if action == Action::None => {
                action = Action::Contract;
            }
            SPACE if action == Action::ContractName => {
                name = buffer.trim().to_string();
                buffer = String::new();
                action = Action::None;
            }
            _ if action == Action::None => {
                buffer.push(ch);
                action = Action::ContractName;
            }
            _ if action == Action::ContractName || action == Action::Contract => {
                buffer.push(ch);
                if buffer.trim() == "event" {
                    events.push(parse_event(&mut imports, chars, &comments));
                    comments.clear();
                    buffer.clear();
                } else if buffer.trim() == "enum" {
                    enums.push(parse_enum(chars, &comments));
                    comments.clear();
                    buffer.clear();
                } else if buffer.trim() == "struct" {
                    structs.push(parse_struct(&mut imports, chars, &comments));
                    comments.clear();
                    buffer.clear();
                } else if buffer.trim() == "constructor" {
                    constructor = parse_function(&mut imports, chars, &comments)?;
                    comments.clear();
                    buffer.clear();
                } else if buffer.trim() == "function" {
                    functions.push(parse_function(&mut imports, chars, &comments)?);
                    comments.clear();
                    buffer.clear();
                } else if ch == SEMICOLON {
                    fields.push(parse_contract_field(buffer.trim().to_owned(), &mut imports));
                    buffer.clear();
                }
            }
            _ => {}
        }
    }

    let mut storage = HashMap::<String, String>::new();
    for contract_field in fields.iter() {
        storage.insert(
            contract_field.name.clone(),
            contract_field.field_type.clone(),
        );
    }
    let mut functions_map = HashMap::<String, String>::new();
    for function in functions.iter() {
        functions_map.insert(
            function.header.name.clone(),
            format!(
                "{}{}",
                if function.header.external {
                    String::from("")
                } else {
                    String::from("_")
                },
                function.header.name.to_case(Case::Snake)
            ),
        );
    }

    // now we know the contracts members and we can parse statements
    for function in functions.iter_mut() {
        parse_statements(function, &storage, &mut imports, &functions_map);
    }
    parse_statements(&mut constructor, &storage, &mut imports, &functions_map);

    Ok(Contract {
        name,
        fields,
        constructor,
        events,
        enums,
        structs,
        functions,
        imports,
        comments: contract_comments,
    })
}

/// Parses the code of a Solidity interface
///
/// `contract_definition` the definition of the interface
/// `lines` the solidity code of the interface
///
/// returns the representation of the interface as `Interface` struct
pub fn parse_interface(
    chars: &mut Chars,
    contract_comments: Vec<String>,
) -> Result<Interface, ParserError> {
    let mut buffer = String::new();
    let mut action = Action::None;

    let mut name = String::new();
    let mut comments = Vec::<String>::new();
    let mut events = Vec::<Event>::new();
    let mut enums = Vec::<Enum>::new();
    let mut structs = Vec::<Struct>::new();
    let mut function_headers = Vec::<FunctionHeader>::new();
    let mut imports = HashSet::<String>::new();

    while let Some(ch) = chars.next() {
        match ch {
            NEW_LINE if action == Action::None || action == Action::Contract => {}
            SLASH if action == Action::Contract => action = Action::Slash,
            SLASH if action == Action::Slash => {
                let comment = parse_comment(chars);
                if comment.len() > 0 {
                    comments.push(comment);
                }
                action = Action::Contract;
            }
            ASTERISK if action == Action::Slash => {
                let mut new_comments = parse_multiline_comment(chars);
                comments.append(&mut new_comments);
                action = Action::Contract;
            }
            CURLY_OPEN => {
                action = Action::Contract;
            }
            SPACE if action == Action::ContractName => {
                name = buffer.trim().substring(1, buffer.len()).to_owned();
                buffer = String::new();
                action = Action::ContractNamed;
            }
            _ if action == Action::None => {
                buffer.push(ch);
                action = Action::ContractName;
            }
            _ if action == Action::ContractName || action == Action::Contract => {
                buffer.push(ch);
                if buffer.trim() == "event" {
                    events.push(parse_event(&mut imports, chars, &comments));
                    comments.clear();
                    buffer.clear();
                } else if buffer.trim() == "enum" {
                    enums.push(parse_enum(chars, &comments));
                    comments.clear();
                    buffer.clear();
                } else if buffer.trim() == "struct" {
                    structs.push(parse_struct(&mut imports, chars, &comments));
                    comments.clear();
                    buffer.clear();
                } else if buffer.trim() == "function" {
                    let function_header = parse_function_header(chars, &mut imports, &comments);
                    function_headers.push(function_header);
                    comments.clear();
                    buffer.clear();
                }
            }
            _ => {}
        }
    }

    Ok(Interface {
        name,
        events,
        enums,
        structs,
        function_headers,
        imports,
        comments: contract_comments,
    })
}

/// Parses a field of the contract
///
/// `line` the raw representation of the field
/// `imports` the HashSet of imports of the contract
///
/// returns the representation of contract field as `ContractField` struct
fn parse_contract_field(line: String, imports: &mut HashSet<String>) -> ContractField {
    // most mappings are written as `type => type`
    // we will make it `type=>type`
    let tokens = split(&line.replace(" => ", "=>"), " ", None);
    let name_index = if tokens.len() > 2 { 2 } else { 1 };

    let field_type = convert_variable_type(tokens[0].to_owned(), imports);
    let name = tokens[name_index]
        .substring(0, tokens[name_index].len() - 1)
        .to_owned();
    ContractField { field_type, name }
}

/// Parses the function header of a Solidity function
///
/// `line` the raw representation of the function header
/// `imports` the set of imports of the contract
///
/// returns the representation of the function header as `FunctionHeader` struct
fn parse_function_header(
    chars: &mut Chars,
    imports: &mut HashSet<String>,
    comments: &Vec<String>,
) -> FunctionHeader {
    let mut function_header_raw = read_until(chars, vec![SEMICOLON, CURLY_OPEN]);
    function_header_raw.remove_matches(" memory");
    function_header_raw.remove_matches(" storage");
    function_header_raw.remove_matches(" calldata");

    let split_by_left_brace = split(&function_header_raw, "(", None);
    let name = split_by_left_brace[0].to_owned();

    let split_by_right_brace = split(&split_by_left_brace[1].trim().to_owned(), ")", None);

    let params_raw = split_by_right_brace[0].trim().to_owned();
    let params = parse_function_parameters(params_raw, imports);
    let attributes_raw = split_by_right_brace[1].to_owned();
    let (external, view, payable) = parse_function_attributes(attributes_raw);

    let return_params = if split_by_left_brace.len() == 3 {
        parse_return_parameters(
            split(&split_by_left_brace[2], ")", None)[0].to_owned(),
            imports,
        )
    } else {
        Vec::<FunctionParam>::new()
    };

    FunctionHeader {
        name,
        params,
        external,
        view,
        payable,
        return_params,
        comments: comments.to_vec(),
    }
}

/// Parses the Solidity function
///
/// `line` the first line where we found function definition
/// `imports` the set of imports of the contract
/// `iterator` the iterator over lines of the contract file
///
/// returns the function definition as `Function` struct
fn parse_function(
    imports: &mut HashSet<String>,
    chars: &mut Chars,
    comments: &Vec<String>,
) -> Result<Function, ParserError> {
    let mut open_braces = 1;
    let mut close_braces = 0;
    let function_header = parse_function_header(chars, imports, comments);
    let mut statements = Vec::<Statement>::new();
    let mut buffer = String::new();

    while let Some(ch) = chars.next() {
        if ch == CURLY_OPEN {
            open_braces += 1;
        } else if ch == CURLY_CLOSE {
            close_braces += 1
        }

        if ch == NEW_LINE {
            buffer.push(SPACE);
        } else if ch == SEMICOLON || ch == CURLY_CLOSE || ch == CURLY_OPEN {
            buffer.push(ch);
            if open_braces == close_braces {
                break
            }
            statements.push(Statement {
                content: buffer.clone(),
                comment: true,
            });
            buffer.clear();
        } else {
            buffer.push(ch);
        }
    }

    Ok(Function {
        header: function_header,
        body: statements,
    })
}

fn parse_statements(
    function: &mut Function,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, String>,
) {
    let statements = function
        .body
        .iter()
        .map(|statement| {
            let mut adjusted = statement.content.clone();
            adjusted.remove_matches(";");
            parse_statement(
                &adjusted,
                function.header.name.is_empty(),
                &storage,
                imports,
                &functions,
            )
        })
        .collect::<Vec<Statement>>();
    function.body = statements;
}

/// Parses the statement of a Solidity function
///
/// `line` the statement
///
/// returns the statement as `Statement` struct
fn parse_statement(
    line: &String,
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, String>,
) -> Statement {
    let tokens = split(&trim(line), " ", None);

    if tokens[0] == "return" {
        return parse_return(line, storage)
    } else if is_type(tokens[0].to_owned()) {
        return parse_declaration(tokens, constructor, storage, imports, functions)
    } else if split(&tokens[0], "(", None)[0] == "require" {
        return parse_require(line, constructor, storage, imports)
    } else if tokens[0] == "unchecked" {
        // TODO
    } else if tokens[0] == "emit" {
        // TODO
    }
    // Assignment
    // Function call

    // TODO actual parsing
    Statement {
        content: trim(line),
        comment: true,
    }
}

#[inline(always)]
fn is_type(line: String) -> bool {
    TYPES.contains_key(&line)
}

fn parse_return(line: &String, storage: &HashMap<String, String>) -> Statement {
    let mut raw_content = line.to_owned();
    raw_content.remove_matches("return");
    raw_content = trim(&raw_content);
    raw_content = raw_content.replace(", ", ",");

    let tokens = split(&raw_content, " ", None);
    let output = if tokens.len() == 1 {
        parse_expression(raw_content, false, storage)
    } else {
        // TODO
        raw_content
    };

    Statement {
        content: format!("return Ok({})", output),
        comment: false,
    }
}

fn parse_require(
    line: &String,
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
) -> Statement {
    let regex = Regex::new(
        r#"(?x)
        ^\s*require\((?P<condition>.+)
        \s*,\s*"(?P<error>.+)"\)$
        "#,
    )
    .unwrap();

    imports.insert(String::from("use ink::prelude::string::String;"));

    let condition = capture_regex(&regex, line, "condition");
    let error = capture_regex(&regex, line, "error");

    let condition = parse_condition(&condition.unwrap(), constructor, storage, imports);
    let error_output = if constructor {
        format!("panic!(\"{}\")", error.unwrap_or(DEFAULT_ERROR.to_owned()))
    } else {
        format!(
            "return Err(Error::Custom(String::from(\"{}\")))",
            error.unwrap_or(DEFAULT_ERROR.to_owned())
        )
    };

    let content = if condition.right.is_some() {
        format!(
            "if {} {} {} {{\n{}\n}}",
            condition.left,
            negate(condition.operation).to_string(),
            condition.right.unwrap(),
            error_output
        )
    } else {
        format!(
            "if {}{} {{\n{}\n}}",
            negate(condition.operation).to_string(),
            condition.left,
            error_output
        )
    };
    Statement {
        content,
        comment: false,
    }
}

fn negate(operation: Operation) -> Operation {
    match operation {
        Operation::Not => Operation::True,
        Operation::True => Operation::Not,
        Operation::GreaterThanEqual => Operation::LessThan,
        Operation::GreaterThan => Operation::LessThanEqual,
        Operation::LessThanEqual => Operation::GreaterThan,
        Operation::LessThan => Operation::GreaterThanEqual,
        Operation::Equal => Operation::NotEqual,
        Operation::NotEqual => Operation::Equal,
    }
}

fn parse_condition(
    line: &String,
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
) -> Condition {
    let tokens = split(&trim(line), " ", None);
    let mut left_raw = tokens[0].to_owned();
    left_raw.remove_matches("!");
    let mut left = parse_expression(left_raw, constructor, storage);

    let (mut right, mut operation) = if tokens.len() > 1 {
        let right = parse_expression(tokens[2].to_owned(), constructor, storage);
        let operation = *OPERATIONS.get(&tokens[1]).unwrap();
        (Some(right), operation)
    } else {
        (
            None,
            if tokens[0].contains("!") {
                Operation::Not
            } else {
                Operation::True
            },
        )
    };

    if right.clone().unwrap_or(String::from("")) == "address(0)" {
        operation = match operation {
            Operation::Equal => Operation::True,
            Operation::NotEqual => Operation::Not,
            _ => operation,
        };
        right = None;
        left = format!("{}.is_zero()", left);
        imports.insert(String::from("use brush::traits::AcountIdExt;\n"));
    }

    Condition {
        left: left.to_owned(),
        operation,
        right,
    }
}

#[inline(always)]
fn capture_regex(regex: &Regex, line: &String, capture_name: &str) -> Option<String> {
    regex.captures(line).and_then(|cap| {
        cap.name(capture_name)
            .map(|value| value.as_str().to_string())
    })
}

fn parse_declaration(
    tokens: Vec<String>,
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, String>,
) -> Statement {
    let var_type = convert_variable_type(tokens[0].to_owned(), imports);
    let var_name = tokens[1].to_owned();
    if tokens.len() > 2 {
        let expression_raw = tokens.clone().drain(3..).collect::<Vec<String>>().join(" ");
        let expression = parse_expression_raw(expression_raw, constructor, storage, functions);
        return Statement {
            content: format!(
                "let {} : {} = {};",
                var_name.to_case(Case::Snake),
                var_type,
                expression.to_string()
            ),
            comment: false,
        }
    }

    Statement {
        content: format!("let {} : {};", var_name.to_case(Case::Snake), var_type),
        comment: false,
    }
}

fn parse_expression_raw(
    line: String,
    constructor: bool,
    storage: &HashMap<String, String>,
    functions: &HashMap<String, String>,
) -> Expression {
    let mut chars = line.chars();
    let mut buffer = String::new();
    let mut stack = VecDeque::<Expression>::new();

    while let Some(ch) = chars.next() {
        match ch {
            BRACE_OPEN => {
                stack.push_back(Expression::FunctionCall(FunctionCall {
                    name: functions.get(&buffer).unwrap().clone(),
                    args: Vec::new(),
                    constructor,
                }));
                buffer.clear();
            }
            BRACE_CLOSE => {
                let expression = stack.pop_back().unwrap();
                match expression {
                    Expression::FunctionCall(mut function_call) => {
                        function_call.args.push(parse_expression(
                            trim(&buffer),
                            constructor,
                            storage,
                        ));
                        buffer.clear();
                        if stack.is_empty() {
                            return Expression::FunctionCall(function_call)
                        }
                    }
                    _ => {}
                }
            }
            COMMA => {
                let expression = stack.pop_back().unwrap();
                match expression {
                    Expression::FunctionCall(mut function_call) => {
                        function_call.args.push(parse_expression(
                            trim(&buffer),
                            constructor,
                            storage,
                        ));
                        stack.push_back(Expression::FunctionCall(function_call));
                        buffer.clear();
                    }
                    _ => {}
                }
            }
            NEW_LINE => buffer.push(SPACE),
            _ => buffer.push(ch),
        }
    }

    if buffer.len() > 0 {
        return Expression::Custom(parse_expression(buffer, constructor, storage))
    }

    return Expression::Custom(String::from(""))
}

fn parse_expression(
    expression_raw: String,
    constructor: bool,
    storage: &HashMap<String, String>,
) -> String {
    if expression_raw == "msg.sender" {
        return String::from("self.env().caller()")
    } else if is_literal(&expression_raw) {
        return expression_raw
    }

    let (field, expression) = if expression_raw.contains(SQUARE_OPEN) {
        let mut mapping_name = String::new();
        let mut chars = expression_raw.chars();
        while let Some(ch) = chars.next() {
            match ch {
                SQUARE_OPEN => break,
                _ => mapping_name.push(ch),
            }
        }
        let mut buffer = String::new();
        let mut indices_raw = Vec::<String>::new();
        let mut square_open = 1;
        let mut square_close = 0;
        while let Some(ch) = chars.next() {
            match ch {
                SQUARE_OPEN => {
                    square_open += 1;
                }
                SQUARE_CLOSE => {
                    square_close += 1;
                    if square_close == square_open {
                        indices_raw.push(buffer.clone());
                        buffer.clear();
                    } else {
                        buffer.push(ch)
                    }
                }
                _ => buffer.push(ch),
            }
        }
        let indices = if indices_raw.len() > 1 {
            format!("({})", indices_raw.join(", "))
        } else {
            indices_raw[0].to_owned()
        };
        (
            mapping_name.clone(),
            format!("{}.get(&{})", mapping_name.to_case(Case::Snake), indices),
        )
    } else {
        (expression_raw.clone(), expression_raw.to_case(Case::Snake))
    };

    if storage.contains_key(&field) {
        return format!(
            "{}.{}",
            if constructor { "instance" } else { "self" },
            expression
        )
    }
    expression
}

fn is_literal(line: &String) -> bool {
    return line.parse::<i32>().is_ok()
        || line.contains("\"")
        || line.contains("\'")
        || line == "true"
        || line == "false"
}

/// Parses parameters of a function
///
/// `parameters` the raw representation of the paramters of the function
/// `imports` the Set of imports of the contract
///
/// returns the vec of function parameters of this function as `FunctionParam` struct
fn parse_function_parameters(
    parameters: String,
    imports: &mut HashSet<String>,
) -> Vec<FunctionParam> {
    let mut out = Vec::<FunctionParam>::new();

    if parameters.len() > 0 {
        let tokens = split(&parameters, " ", Some(remove_commas()));

        let mut mode = ArgsReader::ARGNAME;
        let mut param_type = convert_variable_type(tokens[0].to_owned(), imports);

        for j in 1..tokens.len() {
            if mode == ArgsReader::ARGTYPE {
                param_type = convert_variable_type(tokens[j].to_owned(), imports);
                mode = ArgsReader::ARGNAME;
            } else if mode == ArgsReader::ARGNAME {
                let name = tokens[j].to_owned();
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
/// `attributes` the raw representation of the attributes of the function
///
/// returns 0. external 1. view 2. payable
fn parse_function_attributes(attributes: String) -> (bool, bool, bool) {
    let mut external = false;
    let mut view = false;
    let mut payable = false;

    let tokens = split(&attributes, " ", Some(remove_commas()));

    for i in 0..tokens.len() {
        let attribute = tokens[i].to_owned();
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
fn parse_return_parameters(
    parameters: String,
    imports: &mut HashSet<String>,
) -> Vec<FunctionParam> {
    let mut out = Vec::<FunctionParam>::new();
    let tokens: Vec<String> = split(&parameters, " ", None);

    let mut iterator = tokens.iter();
    while let Some(token) = iterator.next() {
        token.to_owned().remove_matches(",");
        let param_type = convert_variable_type(token.to_owned(), imports);
        let name = if tokens.len() >= (parameters.matches(",").count() + 1) * 2 {
            iterator.next().unwrap()
        } else {
            "_"
        };
        out.push(FunctionParam {
            name: name.to_owned(),
            param_type,
        })
    }

    out
}

/// Parses Solidity event
///
/// `line` the first line where we found event definition
/// `imports` the set of imports of the contract
/// `iterator` the iterator over lines of the contract file
///
/// returns the event definition as `Event` struct
fn parse_event(imports: &mut HashSet<String>, chars: &mut Chars, comments: &Vec<String>) -> Event {
    let event_raw = read_until(chars, vec![SEMICOLON])
        .trim()
        .replace("( ", "(")
        .replace(" )", ")");

    let tokens = split(&event_raw, " ", None);
    let mut args_reader = ArgsReader::ARGNAME;
    let mut indexed = false;

    let split_brace = split(&tokens[0], "(", None);

    let name = split_brace[0].to_owned();
    let mut field_type = convert_variable_type(split_brace[1].to_owned(), imports);
    let mut fields = Vec::<EventField>::new();

    for i in 1..tokens.len() {
        let mut token = tokens[i].to_owned();
        if token == "indexed" {
            indexed = true;
            continue
        } else if args_reader == ArgsReader::ARGTYPE {
            field_type = convert_variable_type(token, imports);
            args_reader = ArgsReader::ARGNAME;
        } else {
            token.remove_matches(&[',', ')'][..]);
            fields.push(EventField {
                indexed,
                field_type: field_type.to_owned(),
                name: token.to_owned(),
            });
            indexed = false;
            args_reader = ArgsReader::ARGTYPE;
        }
    }

    Event {
        name,
        fields,
        comments: comments.to_vec(),
    }
}

/// Parses Solidity structure
///
/// `line` the first line where we found struct definition
/// `imports` the set of imports of the contract
/// `iterator` the iterator over lines of the contract file
///
/// returns the struct definition as `Struct` struct
fn parse_struct(
    imports: &mut HashSet<String>,
    chars: &mut Chars,
    comments: &Vec<String>,
) -> Struct {
    let struct_raw = read_until(chars, vec![CURLY_CLOSE]);
    let split_brace = split(&struct_raw, "{", None);
    let fields = split(&split_brace[1].trim().to_string(), ";", None);
    let struct_name = split_brace[0].to_owned();

    let mut struct_fields = Vec::<StructField>::new();

    for field in fields.iter() {
        if field.is_empty() {
            continue
        }
        struct_fields.push(parse_struct_field(field.trim().to_owned(), imports));
    }

    Struct {
        name: struct_name.to_owned(),
        fields: struct_fields,
        comments: comments.to_vec(),
    }
}

/// Parses struct fields
///
/// `line` the Solidity definition of the struct field
/// `imports` the HashSet of imports of the contract
///
/// returns the struct field as `StructField` struct
fn parse_struct_field(line: String, imports: &mut HashSet<String>) -> StructField {
    let tokens = split(&line, " ", None);
    let field_type = convert_variable_type(tokens[0].to_owned(), imports);
    let mut name = tokens[1].to_owned();
    name.remove_matches(";");
    StructField { name, field_type }
}

/// Parses Solidity enum
///
/// `line` the Solidity definition of enum
///
/// returns the enum as `Enum` struct
fn parse_enum(chars: &mut Chars, comments: &Vec<String>) -> Enum {
    let enum_raw = read_until(chars, vec![CURLY_CLOSE]);
    let tokens = split(&enum_raw, " ", None);
    let name = tokens[0].to_owned();
    let mut values = Vec::<String>::new();

    for i in 1..tokens.len() {
        let mut token = tokens[i].to_owned();
        if token == "{" {
            continue
        } else {
            token.remove_matches(",");
            values.push(token);
        }
    }
    Enum {
        name,
        values,
        comments: comments.to_vec(),
    }
}

/// Converts solidity variable type to ink! variable type (eg. address -> AccountId, uint -> u128, ...)
///
/// `arg_type` solidity argument type
/// `imports` the set of imports of the contract
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
        let type_args = split(
            &arg_type.substring(8, arg_type.len() - 1).to_string(),
            "=>",
            None,
        );
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
    let output_type = match convert_int(no_array_arg_type.to_string()).as_str() {
        str if str.contains("uint") => str.replace("uint", "u"),
        str if str.contains("int") => str.replace("int", "i"),
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

fn convert_int(arg_type: String) -> String {
    if arg_type.contains("int") {
        let int_size = if arg_type == "int" || arg_type == "uint" {
            128
        } else {
            let original_size = arg_type
                .substring(
                    if arg_type.substring(0, 3) == "int" {
                        3
                    } else {
                        4
                    },
                    arg_type.len(),
                )
                .parse::<i32>()
                .unwrap();

            match original_size {
                i if i <= 8 => 8,
                i if i <= 16 => 16,
                i if i <= 32 => 32,
                i if i <= 64 => 64,
                _ => 128,
            }
        };
        return if arg_type.contains("uint") {
            format!("uint{}", int_size)
        } else {
            format!("int{}", int_size)
        }
    }
    arg_type
}

fn read_until(chars: &mut Chars, until: Vec<char>) -> String {
    let mut buffer = String::new();
    while let Some(ch) = chars.next() {
        if until.contains(&ch) {
            break
        }
        match ch {
            NEW_LINE => {
                buffer.push(SPACE);
            }
            _ => {
                buffer.push(ch);
            }
        }
    }
    trim(&buffer)
}
