use crate::{
    formatter::*,
    structures::*,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{
        HashMap,
        HashSet,
        VecDeque,
    },
    slice::Iter,
    str::Chars,
};
use substring::Substring;

#[derive(Debug, PartialEq)]
enum ArgsReader {
    ARGTYPE,
    ARGNAME,
}

macro_rules! selector {
    ($constructor:ident) => {
        if $constructor {
            String::from("instance")
        } else {
            String::from("self")
        }
    };
}

const DEFAULT_ERROR: &str = "SMART CONTRACTZ MAKE PANIC BEEP BEEP BEEP";

lazy_static! {
    static ref TYPES: HashMap<&'static str, (&'static str, Option<&'static str>, Option<&'static str>)> = {
        let mut map = HashMap::new();
        map.insert(
            "address",
            ("AccountId", None, Some("brush::traits::AccountId")),
        );
        map.insert("bool", ("bool", None, None));
        map.insert("bytes1", ("[u8; 1]", None, None));
        map.insert("bytes2", ("[u8; 2]", None, None));
        map.insert("bytes3", ("[u8; 3]", None, None));
        map.insert("bytes4", ("[u8; 4]", None, None));
        map.insert("bytes5", ("[u8; 5]", None, None));
        map.insert("bytes6", ("[u8; 6]", None, None));
        map.insert("bytes7", ("[u8; 7]", None, None));
        map.insert("bytes8", ("[u8; 8]", None, None));
        map.insert("bytes9", ("[u8; 9]", None, None));
        map.insert("bytes10", ("[u8; 10]", None, None));
        map.insert("bytes11", ("[u8; 11]", None, None));
        map.insert("bytes12", ("[u8; 12]", None, None));
        map.insert("bytes13", ("[u8; 13]", None, None));
        map.insert("bytes14", ("[u8; 14]", None, None));
        map.insert("bytes15", ("[u8; 15]", None, None));
        map.insert("bytes16", ("[u8; 16]", None, None));
        map.insert("bytes17", ("[u8; 17]", None, None));
        map.insert("bytes18", ("[u8; 18]", None, None));
        map.insert("bytes19", ("[u8; 19]", None, None));
        map.insert("bytes20", ("[u8; 20]", None, None));
        map.insert("bytes21", ("[u8; 21]", None, None));
        map.insert("bytes22", ("[u8; 22]", None, None));
        map.insert("bytes23", ("[u8; 23]", None, None));
        map.insert("bytes24", ("[u8; 24]", None, None));
        map.insert("bytes25", ("[u8; 25]", None, None));
        map.insert("bytes26", ("[u8; 26]", None, None));
        map.insert("bytes27", ("[u8; 27]", None, None));
        map.insert("bytes28", ("[u8; 28]", None, None));
        map.insert("bytes29", ("[u8; 29]", None, None));
        map.insert("bytes30", ("[u8; 30]", None, None));
        map.insert("bytes31", ("[u8; 31]", None, None));
        map.insert("bytes32", ("[u8; 32]", None, None));
        map.insert(
            "bytes",
            (
                "Vec<u8>",
                Some("Vec::<u8>::from"),
                Some("ink::prelude::vec::Vec"),
            ),
        );
        map.insert("byte", ("u8", None, None));
        map.insert("int8", ("i8", None, None));
        map.insert("int16", ("i16", None, None));
        map.insert("int24", ("i32", None, None));
        map.insert("int32", ("i32", None, None));
        map.insert("int40", ("i64", None, None));
        map.insert("int48", ("i64", None, None));
        map.insert("int56", ("i64", None, None));
        map.insert("int64", ("i64", None, None));
        map.insert("int72", ("i128", None, None));
        map.insert("int80", ("i128", None, None));
        map.insert("int88", ("i128", None, None));
        map.insert("int96", ("i128", None, None));
        map.insert("int104", ("i128", None, None));
        map.insert("int112", ("i128", None, None));
        map.insert("int120", ("i128", None, None));
        map.insert("int128", ("i128", None, None));
        map.insert("int136", ("i128", None, None));
        map.insert("int144", ("i128", None, None));
        map.insert("int152", ("i128", None, None));
        map.insert("int160", ("i128", None, None));
        map.insert("int168", ("i128", None, None));
        map.insert("int176", ("i128", None, None));
        map.insert("int184", ("i128", None, None));
        map.insert("int192", ("i128", None, None));
        map.insert("int200", ("i128", None, None));
        map.insert("int208", ("i128", None, None));
        map.insert("int216", ("i128", None, None));
        map.insert("int224", ("i128", None, None));
        map.insert("int232", ("i128", None, None));
        map.insert("int240", ("i128", None, None));
        map.insert("int248", ("i128", None, None));
        map.insert("int256", ("i128", None, None));
        map.insert("int", ("i128", None, None));
        map.insert("mapping", ("Mapping", None, None));
        map.insert(
            "string",
            ("String", None, Some("ink::prelude::string::String")),
        );
        map.insert("uint8", ("u8", None, None));
        map.insert("uint16", ("u16", None, None));
        map.insert("uint24", ("u32", None, None));
        map.insert("uint32", ("u32", None, None));
        map.insert("uint40", ("u64", None, None));
        map.insert("uint48", ("u64", None, None));
        map.insert("uint56", ("u64", None, None));
        map.insert("uint64", ("u64", None, None));
        map.insert("uint72", ("u128", None, None));
        map.insert("uint80", ("u128", None, None));
        map.insert("uint88", ("u128", None, None));
        map.insert("uint96", ("u128", None, None));
        map.insert("uint104", ("u128", None, None));
        map.insert("uint112", ("u128", None, None));
        map.insert("uint120", ("u128", None, None));
        map.insert("uint128", ("u128", None, None));
        map.insert("uint136", ("u128", None, None));
        map.insert("uint144", ("u128", None, None));
        map.insert("uint152", ("u128", None, None));
        map.insert("uint160", ("u128", None, None));
        map.insert("uint168", ("u128", None, None));
        map.insert("uint176", ("u128", None, None));
        map.insert("uint184", ("u128", None, None));
        map.insert("uint192", ("u128", None, None));
        map.insert("uint200", ("u128", None, None));
        map.insert("uint208", ("u128", None, None));
        map.insert("uint216", ("u128", None, None));
        map.insert("uint224", ("u128", None, None));
        map.insert("uint232", ("u128", None, None));
        map.insert("uint240", ("u128", None, None));
        map.insert("uint248", ("u128", None, None));
        map.insert("uint256", ("u128", None, None));
        map.insert("uint", ("u128", None, None));
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
        map.insert(String::from("&&"), Operation::LogicalAnd);
        map.insert(String::from("||"), Operation::LogicalOr);
        map.insert(String::from("+"), Operation::Add);
        map.insert(String::from("-"), Operation::Subtract);
        map.insert(String::from("="), Operation::Assign);
        map.insert(String::from(">>"), Operation::ShiftRight);
        map.insert(String::from("<<"), Operation::ShiftLeft);
        map.insert(String::from("+="), Operation::AddAssign);
        map.insert(String::from("-="), Operation::SubtractAssign);
        map.insert(String::from("*"), Operation::Mul);
        map.insert(String::from("*="), Operation::MulAssign);
        map.insert(String::from("/"), Operation::Div);
        map.insert(String::from("/="), Operation::DivAssign);
        map.insert(String::from("&"), Operation::BitwiseAnd);
        map.insert(String::from("|"), Operation::BitwiseOr);
        map.insert(String::from("&="), Operation::AndAssign);
        map.insert(String::from("|="), Operation::OrAssign);
        map.insert(String::from("**"), Operation::Pow);
        map.insert(String::from("%"), Operation::Modulo);
        map
    };
    static ref SPECIFIC_EXPRESSION: HashMap<String, Expression> = {
        let mut map = HashMap::new();
        map.insert(String::from("address(0)"), Expression::ZeroAddressInto);
        map.insert(
            String::from("type(uint256).max"),
            Expression::Literal(String::from("u128::MAX")),
        );
        map.insert(String::from("msg.sender"), Expression::EnvCaller(None));
        map
    };
    static ref REGEX_RETURN: Regex = Regex::new(r#"(?x)^\s*return\s+(?P<output>.+);\s*$"#).unwrap();
    static ref REGEX_DECLARE: Regex = Regex::new(
        r#"(?x)^\s*
        (?P<field_type>[a-zA-Z0-9\[\]]+)\s+
        (?P<field_name>[_a-zA-Z0-9]+)\s*
        (=\s*(?P<value>.+))*;\s*$"#
    )
    .unwrap();
    static ref REGEX_REQUIRE: Regex = Regex::new(
        r#"(?x)
        ^\s*require\s*\((?P<condition>.+?)\s*
        (,\s*"(?P<error>.*)"\s*)*\);\s*$"#
    )
    .unwrap();
    static ref REGEX_COMMENT: Regex = Regex::new(r#"(?x)^\s*///*\s*(?P<comment>.*)\s*$"#).unwrap();
    static ref REGEX_IF: Regex =
        Regex::new(r#"(?x)^\s*if\s*\((?P<condition>.+)\s*\)\s*\{\s*"#).unwrap();
    static ref REGEX_ELSE: Regex = Regex::new(r#"^\s*else\s*\{\s*"#).unwrap();
    static ref REGEX_ELSE_IF: Regex =
        Regex::new(r#"(?x)^\s*else\s+if\s*\((?P<condition>.+)\s*\)\s*\{\s*"#).unwrap();
    static ref REGEX_UNCHECKED: Regex = Regex::new(r#"(?x)^\s*unchecked\s*\{\s*"#).unwrap();
    static ref REGEX_END_BLOCK: Regex = Regex::new(r#"^\s*\}\s*"#).unwrap();
    static ref REGEX_TRY: Regex = Regex::new(r#"(?x)^\s*try\s*.*$"#).unwrap();
    static ref REGEX_ASSEMBLY: Regex = Regex::new(r#"(?x)^\s*assembly\s*\{\s*"#).unwrap();
    static ref REGEX_CATCH: Regex = Regex::new(r#"(?x)^\s*catch\s*.*$"#).unwrap();
    static ref REGEX_EMIT: Regex = Regex::new(
        r#"(?x)
        ^\s*emit\s+(?P<event_name>.+?)\s*\(\s*
        (?P<args>.+)+?\);\s*$"#
    )
    .unwrap();
    static ref REGEX_ASSIGN: Regex = Regex::new(
        r#"(?x)
        ^\s*(?P<left>[0-9a-zA-Z_\[\].]+?)\s*
        (?P<operation>[+\-*/&|]*=)\s*
        (?P<right>.+)+?;\s*$"#
    )
    .unwrap();
    static ref REGEX_FUNCTION_CALL: Regex = Regex::new(
        r#"(?x)
        ^\s*(?P<function_name>[a-zA-Z0-9_]+?)\s*\(
        \s*(?P<args>.+)\s*
        \);*\s*$"#,
    )
    .unwrap();
    static ref REGEX_BOOLEAN: Regex = Regex::new(
        r#"(?x)
        ^\s*(?P<left>.+?)
        \s*(?P<operation>[!=><]=*)\s*
        (?P<right>.+)
        \s*$"#,
    )
    .unwrap();
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParserError {
    FileError(String),
    FileCorrupted,
    LibraryParsingNotImplemented,
}

impl From<std::io::Error> for ParserError {
    fn from(error: std::io::Error) -> Self {
        ParserError::FileError(error.to_string())
    }
}

#[derive(Eq, PartialEq)]
enum Action {
    None,
    AssemblyStart,
    Assembly,
    ContractName,
    ContractNamed,
    Contract,
    Slash,
}

const ASTERISK: char = '*';
const BRACKET_CLOSE: char = ']';
const BRACKET_OPEN: char = '[';
const COMMA: char = ',';
const CURLY_CLOSE: char = '}';
const CURLY_OPEN: char = '{';
const NEW_LINE: char = '\n';
const PARENTHESIS_CLOSE: char = ')';
const PARENTHESIS_OPEN: char = '(';
const SEMICOLON: char = ';';
const SLASH: char = '/';
const SPACE: char = ' ';

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
                    read_until(&mut chars, vec![SEMICOLON]);
                    buffer = String::new();
                } else if buffer == "abstract" {
                    buffer.clear();
                } else if buffer == "contract" {
                    let contract = parse_contract(&mut chars, comments)?;
                    return Ok((Some(contract), None))
                } else if buffer == "interface" {
                    let interface = parse_interface(&mut chars, comments)?;
                    return Ok((None, Some(interface)))
                } else if buffer == "library" {
                    return Err(ParserError::LibraryParsingNotImplemented)
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
            if buffer.trim().len() > 0 {
                comments.push(format!(" {}", buffer.trim()));
            }
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

/// Parses the code of a Solidity contract
///
/// `contract_definition` the definition of the contract
/// `lines` the solidity code of the contract
///
/// returns the representation of the contract as `Contract` struct
fn parse_contract(chars: &mut Chars, contract_doc: Vec<String>) -> Result<Contract, ParserError> {
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
    let mut modifiers = Vec::<Modifier>::new();

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
                } else if buffer.trim() == "using" {
                    read_until(chars, vec![SEMICOLON]);
                    buffer.clear();
                } else if buffer.trim() == "modifier" {
                    modifiers.push(parse_modifier(&mut imports, chars, &comments)?);
                    comments.clear();
                    buffer.clear();
                } else if ch == SEMICOLON {
                    fields.push(parse_contract_field(
                        buffer.trim().to_owned(),
                        &mut imports,
                        &comments,
                    ));
                    buffer.clear();
                    comments.clear();
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

    let mut functions_map = HashMap::<String, bool>::new();
    for function in functions.iter() {
        functions_map.insert(function.header.name.clone(), function.header.external);
    }

    let mut events_map = HashMap::<String, Event>::new();
    for event in events.iter() {
        events_map.insert(event.name.clone(), event.clone());
    }

    let mut modifiers_map = HashMap::<String, ()>::new();
    for modifier in modifiers.iter() {
        modifiers_map.insert(modifier.header.name.clone(), ());
    }

    // now we know the contracts members and we can parse statements
    for function in functions.iter_mut() {
        function.header.modifiers = process_function_modifiers(
            &function.header.modifiers,
            &modifiers_map,
            &storage,
            &mut imports,
            &functions_map,
        );
        function.body = parse_statements(
            &function.body,
            &storage,
            &mut imports,
            &functions_map,
            &events_map,
            false,
        );
    }

    for modifier in modifiers.iter_mut() {
        modifier.statements = parse_statements(
            &modifier.statements,
            &storage,
            &mut imports,
            &functions_map,
            &events_map,
            false,
        );
    }

    constructor.body = parse_statements(
        &constructor.body,
        &storage,
        &mut imports,
        &functions_map,
        &events_map,
        true,
    );

    Ok(Contract {
        name,
        fields,
        constructor,
        events,
        enums,
        structs,
        functions,
        imports,
        contract_doc,
        modifiers,
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
fn parse_contract_field(
    line_raw: String,
    imports: &mut HashSet<String>,
    comments: &Vec<String>,
) -> ContractField {
    let mut line = line_raw.replace(" => ", "=>");
    line = line.replace(" ( ", "(");
    line = line.replace(" ) ", ")");
    let regex: Regex = Regex::new(
        r#"(?x)^\s*
        (?P<field_type>.+?)\s
        (?P<attributes>(\s*constant\s*|\s*private\s*|\s*public\s*)*)*
        (?P<field_name>.+?)\s*
        (=\s*(?P<initial_value>.+)\s*)*
        ;\s*$"#,
    )
    .unwrap();

    let field_type_raw = capture_regex(&regex, &line, "field_type").unwrap();
    let attributes_raw = capture_regex(&regex, &line, "attributes");
    let field_name = capture_regex(&regex, &line, "field_name").unwrap();
    let initial_value = capture_regex(&regex, &line, "initial_value");
    let constant = attributes_raw
        .unwrap_or(String::from(""))
        .contains("constant");
    let field_type = convert_variable_type(field_type_raw, imports);

    ContractField {
        field_type,
        name: field_name,
        comments: comments.clone(),
        initial_value,
        constant,
    }
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

    let regex_return_function = Regex::new(
        r#"(?x)
        ^\s*(?P<function_name>[a-zA-Z0-9_]*?)\s*
        \(\s*(?P<parameters>[a-zA-Z0-9_,\s\[\]]*?)\s*\)\s*
        (?P<attributes>.*)\s+returns\s*\(\s*(?P<return_parameters>[a-zA-Z0-9_,\s\[\]]*?)\s*\)
        .*$"#,
    )
    .unwrap();
    let return_parameters_maybe = capture_regex(
        &regex_return_function,
        &function_header_raw,
        "return_parameters",
    );

    let (name, params, return_params, modifiers) =
        if let Some(return_parameters_raw) = return_parameters_maybe {
            let function_name = capture_regex(
                &regex_return_function,
                &function_header_raw,
                "function_name",
            )
            .unwrap();
            let parameters_raw =
                capture_regex(&regex_return_function, &function_header_raw, "parameters").unwrap();
            let parameters = parse_function_parameters(parameters_raw, imports);
            let return_parameters = parse_return_parameters(return_parameters_raw, imports);
            let attribs_raw =
                capture_regex(&regex_return_function, &function_header_raw, "attributes").unwrap();
            (
                function_name,
                parameters,
                return_parameters,
                parse_modifiers(&attribs_raw),
            )
        } else {
            let regex_no_return = Regex::new(
                r#"(?x)
            ^\s*(?P<function_name>[a-zA-Z0-9_]*?)\s*
            \(\s*(?P<parameters>[a-zA-Z0-9_,\s\[\]]*?)\s*\)
            \s*(?P<attributes>.*)\s*$"#,
            )
            .unwrap();
            let function_name =
                capture_regex(&regex_no_return, &function_header_raw, "function_name").unwrap();
            let parameters_raw =
                capture_regex(&regex_no_return, &function_header_raw, "parameters").unwrap();
            let attribs_raw =
                capture_regex(&regex_no_return, &function_header_raw, "attributes").unwrap();
            let parameters = parse_function_parameters(parameters_raw, imports);
            (
                function_name,
                parameters,
                Vec::default(),
                parse_modifiers(&attribs_raw),
            )
        };

    let (external, view, payable) = parse_function_attributes(&function_header_raw);

    FunctionHeader {
        name,
        params,
        external,
        view,
        payable,
        return_params,
        comments: comments.to_vec(),
        modifiers,
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
    Ok(Function {
        header: parse_function_header(chars, imports, comments),
        body: parse_body(chars),
    })
}

fn parse_modifier(
    imports: &mut HashSet<String>,
    chars: &mut Chars,
    comments: &Vec<String>,
) -> Result<Modifier, ParserError> {
    imports.insert(String::from("use brush::modifier_definition;"));
    imports.insert(String::from("use brush::modifiers;"));
    Ok(Modifier {
        header: parse_function_header(chars, imports, comments),
        statements: parse_body(chars),
        comments: comments.clone(),
    })
}

fn parse_body(chars: &mut Chars) -> Vec<Statement> {
    let mut buffer = String::new();
    let mut open_braces = 1;
    let mut close_braces = 0;
    let mut statements = Vec::<Statement>::new();
    let mut action = Action::None;

    while let Some(ch) = chars.next() {
        if ch == CURLY_OPEN {
            open_braces += 1;
        } else if ch == CURLY_CLOSE {
            close_braces += 1
        }

        if ch == NEW_LINE {
            if action == Action::AssemblyStart {
                action = Action::Assembly;
            } else if action == Action::Assembly {
                statements.push(Statement::Raw(buffer.clone()));
                buffer.clear();
            } else {
                buffer.push(SPACE);
            }
        } else if ch == SEMICOLON || ch == CURLY_CLOSE || ch == CURLY_OPEN {
            buffer.push(ch);
            if open_braces == close_braces {
                break
            }
            statements.push(Statement::Raw(buffer.clone()));
            if action == Action::Assembly {
                action = Action::None;
            }
            buffer.clear();
        } else if ch == SLASH {
            let next_maybe = chars.next();
            if next_maybe == Some(SLASH) {
                statements.push(Statement::Raw(format!("// {}", parse_comment(chars))));
                continue
            } else if next_maybe == Some(ASTERISK) {
                for comment in parse_multiline_comment(chars).iter() {
                    statements.push(Statement::Raw(format!("// {comment}")));
                }
                continue
            }
            buffer.push(ch);
            buffer.push(next_maybe.unwrap());
        } else {
            buffer.push(ch);
            if trim(&buffer) == "assembly" {
                action = Action::AssemblyStart;
            } else if trim(&buffer) == "for" {
                open_braces += 1;
                let for_block = read_until(chars, vec!['{']);
                statements.push(Statement::Raw(format!("for{for_block}{{")));
                buffer.clear();
            }
        }
    }

    statements
}

fn process_function_modifiers(
    raw_modifiers: &Vec<String>,
    modifiers_map: &HashMap<String, ()>,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
) -> Vec<String> {
    let regex_modifier_name = Regex::new(r#"(?x)^\s*(?P<name>.+?)\(.\)*"#).unwrap();
    let mut out = Vec::default();
    for raw_modifier in raw_modifiers.iter() {
        let modifier_name =
            capture_regex(&regex_modifier_name, raw_modifier, "name").unwrap_or(String::new());
        if modifiers_map.contains_key(&modifier_name) {
            let function_call =
                parse_function_call(&raw_modifier, false, storage, imports, functions);
            let mut function_call_string_raw = function_call.to_string();
            // openbrush modifiers do not support function calls
            // so if such situation occurs, we remove self. and ? for formatting
            function_call_string_raw.remove_matches("self.");
            function_call_string_raw.remove_matches("?");
            out.push(function_call_string_raw)
        }
    }
    out
}

fn parse_statements(
    statements: &Vec<Statement>,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
    events: &HashMap<String, Event>,
    constructor: bool,
) -> Vec<Statement> {
    let mut iterator = statements.iter();
    let mut stack = VecDeque::<Block>::new();
    let mut out = Vec::default();

    while let Some(statement) = iterator.next() {
        match statement {
            Statement::Raw(content) => {
                out.push(parse_statement(
                    &content,
                    constructor,
                    &storage,
                    imports,
                    &functions,
                    &mut stack,
                    &mut iterator,
                    events,
                ));
            }
            _ => {}
        }
    }

    out
}

/// Parses the statement of a Solidity function
///
/// `line` the statement
///
/// returns the statement as `Statement` struct
fn parse_statement(
    line_raw: &String,
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
    stack: &mut VecDeque<Block>,
    iterator: &mut Iter<Statement>,
    events: &HashMap<String, Event>,
) -> Statement {
    let mut line = trim(line_raw);
    line = line.replace(" memory ", " ");
    line = line.replace(" calldata ", " ");
    line = line.replace(" storage ", " ");

    if line == "_;" {
        return Statement::ModifierBody
    } else if REGEX_RETURN.is_match(&line) {
        return parse_return(&line, storage, imports, functions)
    } else if REGEX_DECLARE.is_match(&line) {
        return parse_declaration(&line, constructor, storage, imports, functions)
    } else if REGEX_REQUIRE.is_match(&line) {
        return parse_require(&line, constructor, storage, imports, functions)
    } else if REGEX_COMMENT.is_match(&line) {
        let comment = capture_regex(&REGEX_COMMENT, &line, "comment").unwrap();
        return Statement::Comment(comment)
    } else if REGEX_IF.is_match(&line) {
        stack.push_back(Block::If);
        return parse_if(
            &line,
            constructor,
            storage,
            imports,
            functions,
            stack,
            iterator,
            events,
        )
    } else if REGEX_ELSE.is_match(&line) {
        stack.push_back(Block::Else);
        return parse_else(
            constructor,
            storage,
            imports,
            functions,
            stack,
            iterator,
            events,
        )
    } else if REGEX_ELSE_IF.is_match(&line) {
        stack.push_back(Block::ElseIf);
        return parse_else_if(
            &line,
            constructor,
            storage,
            imports,
            functions,
            stack,
            iterator,
            events,
        )
    } else if REGEX_UNCHECKED.is_match(&line) {
        stack.push_back(Block::Unchecked);
        return Statement::Comment(String::from("Please handle unchecked blocks manually >>>"))
    } else if REGEX_END_BLOCK.is_match(&line) {
        if stack.is_empty() {
            return Statement::Comment(String::from("Sol2Ink Not Implemented yet End Block here"))
        }
        match stack.pop_back().unwrap() {
            Block::Assembly => return Statement::AssemblyEnd,
            Block::Catch => return Statement::CatchEnd,
            Block::Unchecked => {}
            Block::If => return Statement::IfEnd,
            Block::Else => return Statement::IfEnd,
            Block::ElseIf => return Statement::IfEnd,
            Block::Try => return Statement::TryEnd,
        }
        return Statement::Comment(String::from("<<< Please handle unchecked blocks manually"))
    } else if REGEX_TRY.is_match(&line) {
        stack.push_back(Block::Try);
        return parse_try(
            &line,
            constructor,
            storage,
            imports,
            functions,
            stack,
            iterator,
            events,
        )
    } else if REGEX_ASSEMBLY.is_match(&line) {
        stack.push_back(Block::Assembly);
        return parse_assembly(stack, iterator)
    } else if REGEX_CATCH.is_match(&line) {
        stack.push_back(Block::Catch);
        return parse_catch(
            &line,
            constructor,
            storage,
            imports,
            functions,
            stack,
            iterator,
            events,
        )
    } else if REGEX_EMIT.is_match(&line) {
        return parse_emit(&line, constructor, storage, imports, functions, events)
    } else if REGEX_ASSIGN.is_match(&line) {
        return parse_assign(&line, constructor, storage, imports, functions)
    } else if REGEX_FUNCTION_CALL.is_match(&line) {
        let expression = parse_function_call(&line, constructor, storage, imports, functions);
        return Statement::FunctionCall(expression)
    }

    Statement::Comment(format!("Sol2Ink Not Implemented yet: {}", line.clone()))
}

fn parse_assign(
    line: &String,
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
) -> Statement {
    let left_raw = capture_regex(&REGEX_ASSIGN, line, "left").unwrap();
    let operation_raw =
        capture_regex(&REGEX_ASSIGN, line, "operation").unwrap_or(String::from("="));
    let right_raw = capture_regex(&REGEX_ASSIGN, line, "right").unwrap();

    let left = parse_member(&left_raw, constructor, storage, imports, functions);
    let operation = *OPERATIONS.get(&operation_raw).unwrap();
    let right = parse_member(&right_raw, constructor, storage, imports, functions);

    return if let Expression::Mapping(name, indices, selector, None) = left {
        let converted_operation = match operation {
            Operation::AddAssign => Operation::Add,
            Operation::MulAssign => Operation::Mul,
            Operation::DivAssign => Operation::Div,
            Operation::SubtractAssign => Operation::Subtract,
            _ => operation,
        };
        let right_mapping = match converted_operation {
            Operation::Add | Operation::Mul | Operation::Div | Operation::Subtract => {
                Some(Box::new(Expression::Arithmetic(
                    Box::new(Expression::Mapping(
                        name.clone(),
                        indices.clone(),
                        selector.clone(),
                        None,
                    )),
                    Box::new(right),
                    converted_operation,
                )))
            }
            _ => Some(Box::new(right)),
        };
        Statement::FunctionCall(Expression::Mapping(name, indices, selector, right_mapping))
    } else {
        Statement::Assign(left, right, operation)
    }
}

fn parse_emit(
    line: &String,
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
    events: &HashMap<String, Event>,
) -> Statement {
    imports.insert(String::from("use ink_lang::codegen::EmitEvent;"));

    let event_name_raw = capture_regex(&REGEX_EMIT, line, "event_name").unwrap();
    let args_raw = capture_regex(&REGEX_EMIT, line, "args").unwrap();

    let mut args = Vec::<Expression>::new();
    let mut chars = args_raw.chars();
    let mut buffer = String::new();
    let mut open_parentheses = 0;
    let mut close_parenthesis = 0;
    let mut event_count = 0;

    while let Some(ch) = chars.next() {
        match ch {
            PARENTHESIS_OPEN => {
                open_parentheses += 1;
                buffer.push(ch)
            }
            PARENTHESIS_CLOSE => {
                close_parenthesis += 1;
                buffer.push(ch)
            }
            COMMA => {
                if open_parentheses == close_parenthesis {
                    args.push(Expression::StructArg(
                        events
                            .get(&event_name_raw)
                            .unwrap_or_else(|| panic!("Event {event_name_raw} not defined"))
                            .fields
                            .get(event_count)
                            .unwrap()
                            .name
                            .clone(),
                        Box::new(parse_member(
                            &trim(&buffer),
                            constructor,
                            storage,
                            imports,
                            functions,
                        )),
                    ));
                    event_count += 1;
                    buffer.clear();
                } else {
                    buffer.push(ch)
                }
            }
            _ => buffer.push(ch),
        }
    }

    args.push(Expression::StructArg(
        events
            .get(&event_name_raw)
            .unwrap()
            .fields
            .get(event_count)
            .unwrap()
            .name
            .clone(),
        Box::new(parse_member(
            &trim(&buffer),
            constructor,
            storage,
            imports,
            functions,
        )),
    ));

    Statement::Emit(event_name_raw, args)
}

fn parse_return(
    line: &String,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
) -> Statement {
    let raw_output = capture_regex(&REGEX_RETURN, line, "output").unwrap();
    let output = parse_member(&raw_output, false, storage, imports, functions);

    Statement::Return(output)
}

fn parse_block(
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
    stack: &mut VecDeque<Block>,
    iterator: &mut Iter<Statement>,
    events: &HashMap<String, Event>,
    statements: &mut Vec<Statement>,
    until: Statement,
) {
    while let Some(statement_raw) = iterator.next() {
        match statement_raw {
            Statement::Raw(content) => {
                let statement = parse_statement(
                    &content,
                    constructor,
                    storage,
                    imports,
                    functions,
                    stack,
                    iterator,
                    events,
                );
                if statement == until {
                    break
                } else {
                    statements.push(statement)
                }
            }
            _ => {}
        }
    }
}

fn parse_if(
    line: &String,
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
    stack: &mut VecDeque<Block>,
    iterator: &mut Iter<Statement>,
    events: &HashMap<String, Event>,
) -> Statement {
    let condition_raw = capture_regex(&REGEX_IF, line, "condition").unwrap();
    let condition = parse_condition(
        &condition_raw,
        constructor,
        false,
        storage,
        imports,
        functions,
    );
    let mut statements = Vec::default();

    parse_block(
        constructor,
        storage,
        imports,
        functions,
        stack,
        iterator,
        events,
        &mut statements,
        Statement::IfEnd,
    );

    Statement::If(condition, statements)
}

fn parse_else(
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
    stack: &mut VecDeque<Block>,
    iterator: &mut Iter<Statement>,
    events: &HashMap<String, Event>,
) -> Statement {
    let mut statements = Vec::default();

    parse_block(
        constructor,
        storage,
        imports,
        functions,
        stack,
        iterator,
        events,
        &mut statements,
        Statement::IfEnd,
    );

    Statement::Else(statements)
}

fn parse_else_if(
    line: &String,
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
    stack: &mut VecDeque<Block>,
    iterator: &mut Iter<Statement>,
    events: &HashMap<String, Event>,
) -> Statement {
    let condition_raw = capture_regex(&REGEX_IF, line, "condition");
    let condition = parse_condition(
        &condition_raw.unwrap(),
        constructor,
        false,
        storage,
        imports,
        functions,
    );
    let mut statements = Vec::default();

    parse_block(
        constructor,
        storage,
        imports,
        functions,
        stack,
        iterator,
        events,
        &mut statements,
        Statement::IfEnd,
    );

    Statement::ElseIf(condition, statements)
}

fn parse_try(
    line: &String,
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
    stack: &mut VecDeque<Block>,
    iterator: &mut Iter<Statement>,
    events: &HashMap<String, Event>,
) -> Statement {
    let mut statements = Vec::default();
    statements.push(Statement::Comment(line.clone()));

    parse_block(
        constructor,
        storage,
        imports,
        functions,
        stack,
        iterator,
        events,
        &mut statements,
        Statement::TryEnd,
    );

    Statement::Try(statements)
}

fn parse_assembly(stack: &mut VecDeque<Block>, iterator: &mut Iter<Statement>) -> Statement {
    let mut statements = Vec::default();
    statements.push(Statement::Comment(String::from(
        "Please handle assembly blocks manually >>>",
    )));

    while let Some(statement_raw) = iterator.next() {
        match statement_raw {
            Statement::Raw(content_raw) => {
                let content = trim(&content_raw);
                if content == "}" {
                    stack.pop_back();
                    statements.push(Statement::AssemblyEnd);
                    break
                } else {
                    statements.push(Statement::Comment(content.clone()));
                }
            }
            _ => {}
        }
    }

    statements.push(Statement::Comment(String::from(
        "<<< Please handle assembly blocks manually",
    )));

    Statement::Assembly(statements)
}

fn parse_catch(
    line: &String,
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
    stack: &mut VecDeque<Block>,
    iterator: &mut Iter<Statement>,
    events: &HashMap<String, Event>,
) -> Statement {
    let mut statements = Vec::default();
    statements.push(Statement::Comment(line.clone()));

    parse_block(
        constructor,
        storage,
        imports,
        functions,
        stack,
        iterator,
        events,
        &mut statements,
        Statement::CatchEnd,
    );

    Statement::Catch(statements)
}

fn parse_require(
    line: &String,
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
) -> Statement {
    imports.insert(String::from("use ink::prelude::string::String;"));

    let condition = capture_regex(&REGEX_REQUIRE, line, "condition");
    let error = capture_regex(&REGEX_REQUIRE, line, "error");

    let condition = parse_condition(
        &condition.unwrap(),
        constructor,
        true,
        storage,
        imports,
        functions,
    );
    let error_output = if constructor {
        format!("panic!(\"{}\")", error.unwrap_or(DEFAULT_ERROR.to_owned()))
    } else {
        format!(
            "return Err(Error::Custom(String::from(\"{}\")))",
            error.unwrap_or(DEFAULT_ERROR.to_owned())
        )
    };

    Statement::Require(condition, error_output)
}

fn parse_member(
    raw: &String,
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
) -> Expression {
    if is_literal(raw) {
        return Expression::Literal(raw.clone())
    } else if let Some(expression) = SPECIFIC_EXPRESSION.get(raw) {
        if expression == &Expression::ZeroAddressInto {
            imports.insert(String::from("use brush::traits::ZERO_ADDRESS;"));
        } else if expression == &Expression::EnvCaller(None) {
            imports.insert(String::from("use ink_lang::codegen::Env;"));
            return Expression::EnvCaller(Some(selector!(constructor)))
        }

        return expression.clone()
    }

    let regex_hex = Regex::new(r#"(?x)^(?P<before>.*)hex"(?P<value>.+?)"(?P<after>.*)$"#).unwrap();
    if regex_hex.is_match(raw) {
        let before = capture_regex(&regex_hex, raw, "before").unwrap();
        let value = capture_regex(&regex_hex, raw, "value").unwrap();
        let after = capture_regex(&regex_hex, raw, "after").unwrap();
        return parse_member(
            &format!("{before}hex(\"{value}\"){after}"),
            constructor,
            storage,
            imports,
            functions,
        )
    }

    let regex_new_array = Regex::new(
        r#"(?x)^\s*new\s+(?P<array_type>.+?)\s*
        \[\s*\]\s*\((?P<array_size>.+?)\s*\)\s*$"#,
    )
    .unwrap();
    if regex_new_array.is_match(raw) {
        let array_type_raw = capture_regex(&regex_new_array, raw, "array_type").unwrap();
        let array_size = capture_regex(&regex_new_array, raw, "array_size").unwrap();

        let array_type = convert_variable_type(array_type_raw, imports);
        return Expression::Member(format!("vec![{array_type}::default(); {array_size}]"), None)
    }

    let regex_type = Regex::new(
        r#"(?x)
        ^\s*type\((?P<rest>.+?)
        \s*$"#,
    )
    .unwrap();
    if regex_type.is_match(raw) {
        // TODO: type(IERC721).interfaceId
        let rest = capture_regex(&regex_type, raw, "rest").unwrap();
        return Expression::Member(format!("type_of({rest}"), None)
    }

    let regex_arithmetic = Regex::new(
        r#"(?x)
        ^\s*(?P<left>.+?)
        \s*(?P<operation>[/+\-*]+)
        [^=]\s*(?P<right>.+)
        \s*$"#,
    )
    .unwrap();
    if regex_arithmetic.is_match(raw) {
        let left_raw = capture_regex(&regex_arithmetic, raw, "left").unwrap();
        let right_raw = capture_regex(&regex_arithmetic, raw, "right").unwrap();
        let operation_raw = capture_regex(&regex_arithmetic, raw, "operation").unwrap();
        let left = parse_member(&left_raw, constructor, storage, imports, functions);
        let right = parse_member(&right_raw, constructor, storage, imports, functions);
        let operation = *OPERATIONS.get(&operation_raw).unwrap();

        return Expression::Arithmetic(Box::new(left), Box::new(right), operation)
    }

    let regex_logical = Regex::new(
        r#"(?x)
        ^\s*(?P<left>.+?)
        \s*(?P<operation>[|&]+)\s*
        (?P<right>.+)
        \s*$"#,
    )
    .unwrap();
    if regex_logical.is_match(raw) {
        let left_raw = capture_regex(&regex_logical, raw, "left").unwrap();
        let operation_raw = capture_regex(&regex_logical, raw, "operation").unwrap();
        let right_raw = capture_regex(&regex_logical, raw, "right").unwrap();
        let left = parse_member(&left_raw, constructor, storage, imports, functions);
        let operation = *OPERATIONS.get(&operation_raw).unwrap();
        let right = parse_member(&right_raw, constructor, storage, imports, functions);

        return Expression::Logical(Box::new(left), operation, Box::new(right))
    }

    let regex_ternary = Regex::new(
        r#"(?x)
        ^\s*(?P<condition>.+?)\s*\?
        \s*(?P<if_true>.+?)\s*:
        \s*(?P<if_false>.+?)\s*$"#,
    )
    .unwrap();
    if regex_ternary.is_match(raw) {
        println!("ternary_raw: {raw}");
        let condition_raw = capture_regex(&regex_ternary, raw, "condition").unwrap();
        let if_true_raw = capture_regex(&regex_ternary, raw, "if_true").unwrap();
        let if_false_raw = capture_regex(&regex_ternary, raw, "if_false").unwrap();

        let condition = parse_condition(
            &condition_raw,
            constructor,
            false,
            storage,
            imports,
            functions,
        );
        let if_true = parse_member(&if_true_raw, constructor, storage, imports, functions);
        let if_false = parse_member(&if_false_raw, constructor, storage, imports, functions);
        return Expression::Ternary(Box::new(condition), Box::new(if_true), Box::new(if_false))
    }

    if REGEX_BOOLEAN.is_match(raw) {
        let condition = parse_condition(raw, constructor, false, storage, imports, functions);
        return Expression::Condition(Box::new(condition))
    }

    if REGEX_FUNCTION_CALL.is_match(raw) {
        return parse_function_call(raw, constructor, storage, imports, functions)
    }

    let regex_with_selector = Regex::new(r#"(?x)^\s*(?P<left>.+?)\.(?P<right>.+?);*\s*$"#).unwrap();
    if regex_with_selector.is_match(raw) {
        let left_raw = capture_regex(&regex_with_selector, raw, "left").unwrap();
        let right_raw = capture_regex(&regex_with_selector, raw, "right").unwrap();
        let left = parse_member(&left_raw, constructor, storage, imports, functions);
        let right = parse_member(&right_raw, constructor, storage, imports, functions);

        match &right {
            Expression::FunctionCall(function_name, expressions, _, external) => {
                return Expression::WithSelector(
                    Box::new(left),
                    Box::new(Expression::FunctionCall(
                        function_name.clone(),
                        expressions.clone(),
                        None,
                        *external,
                    )),
                )
            }
            Expression::Member(member_name, _) => {
                return Expression::WithSelector(
                    Box::new(left),
                    Box::new(Expression::Member(member_name.clone(), None)),
                )
            }
            _ => {}
        };

        return Expression::WithSelector(Box::new(left), Box::new(right))
    }

    let regex_mapping = Regex::new(
        r#"(?x)
        ^\s*(?P<mapping_name>.+?)\s*
        (?P<index>(\[\s*.+\s*\]))+?
        \s*$"#,
    )
    .unwrap();
    if regex_mapping.is_match(raw) {
        let mapping_name_raw = capture_regex(&regex_mapping, raw, "mapping_name").unwrap();
        let indices_raw = capture_regex(&regex_mapping, raw, "index").unwrap();
        let mut indices = Vec::<Expression>::new();
        let mut chars = indices_raw.chars();
        let mut buffer = String::new();
        let mut open_braces = 0;
        let mut close_braces = 0;

        while let Some(ch) = chars.next() {
            match ch {
                BRACKET_OPEN => {
                    if open_braces > close_braces {
                        buffer.push(ch)
                    }
                    open_braces += 1;
                }
                BRACKET_CLOSE => {
                    close_braces += 1;
                    if open_braces == close_braces {
                        indices.push(parse_member(
                            &buffer,
                            constructor,
                            storage,
                            imports,
                            functions,
                        ));
                        buffer.clear();
                    } else {
                        buffer.push(ch)
                    }
                }
                _ => buffer.push(ch),
            }
        }

        let selector = get_selector(storage, constructor, &mapping_name_raw);

        return Expression::Mapping(mapping_name_raw, indices, selector, None)
    }

    let selector = get_selector(storage, constructor, &raw);

    return Expression::Member(raw.clone(), selector)
}

fn parse_function_call(
    raw: &String,
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
) -> Expression {
    let function_name_raw = capture_regex(&REGEX_FUNCTION_CALL, raw, "function_name").unwrap();
    let args_raw = capture_regex(&REGEX_FUNCTION_CALL, raw, "args").unwrap();
    let mut args = Vec::<Expression>::new();
    let mut chars = args_raw.chars();
    let mut buffer = String::new();
    let mut open_parentheses = 0;
    let mut close_parenthesis = 0;

    if TYPES.contains_key(&function_name_raw.as_str()) {
        let the_type = TYPES.get(&function_name_raw.as_str()).unwrap();
        if let Some(unique_cast) = the_type.1 {
            return Expression::Cast(
                true,
                unique_cast.to_string(),
                Box::new(parse_member(
                    &args_raw,
                    constructor,
                    storage,
                    imports,
                    functions,
                )),
            )
        } else {
            return Expression::Cast(
                false,
                the_type.0.to_string(),
                Box::new(parse_member(
                    &args_raw,
                    constructor,
                    storage,
                    imports,
                    functions,
                )),
            )
        }
    }

    while let Some(ch) = chars.next() {
        match ch {
            PARENTHESIS_OPEN => {
                open_parentheses += 1;
                buffer.push(ch)
            }
            PARENTHESIS_CLOSE => {
                close_parenthesis += 1;
                buffer.push(ch)
            }
            COMMA => {
                if open_parentheses == close_parenthesis {
                    args.push(parse_member(
                        &trim(&buffer),
                        constructor,
                        storage,
                        imports,
                        functions,
                    ));
                    buffer.clear();
                } else {
                    buffer.push(ch)
                }
            }
            _ => buffer.push(ch),
        }
    }

    args.push(parse_member(
        &buffer,
        constructor,
        storage,
        imports,
        functions,
    ));

    return Expression::FunctionCall(
        function_name_raw.clone(),
        args,
        Some(selector!(constructor)),
        *functions.get(&function_name_raw).unwrap_or(&false),
    )
}

fn get_selector(
    storage: &HashMap<String, String>,
    constructor: bool,
    field_name: &String,
) -> Option<String> {
    if storage.contains_key(field_name) {
        Some(selector!(constructor))
    } else {
        None
    }
}

fn parse_condition(
    line: &String,
    constructor: bool,
    inverted: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
) -> Condition {
    let (mut left, mut operation, mut right) = if REGEX_BOOLEAN.is_match(line) {
        let left_raw = capture_regex(&REGEX_BOOLEAN, line, "left").unwrap();
        let operation_raw = capture_regex(&REGEX_BOOLEAN, line, "operation").unwrap();
        let right_raw = capture_regex(&REGEX_BOOLEAN, line, "right").unwrap();

        let left = parse_member(&left_raw, constructor, storage, imports, functions);
        let operation = *OPERATIONS.get(&operation_raw).unwrap();
        let right = parse_member(&right_raw, constructor, storage, imports, functions);

        (left, operation, Some(right))
    } else {
        let regex_negative = Regex::new(r#"(?x)^\s*!(?P<value>.+?)\s*$"#).unwrap();
        if regex_negative.is_match(line) {
            let left_raw = capture_regex(&regex_negative, line, "value").unwrap();
            let left = parse_member(&left_raw, constructor, storage, imports, functions);
            (left, Operation::Not, None)
        } else {
            let left = parse_member(&line, constructor, storage, imports, functions);
            (left, Operation::True, None)
        }
    };

    if let Some(Expression::ZeroAddressInto) = right {
        operation = match operation {
            Operation::Equal => Operation::True,
            Operation::NotEqual => Operation::Not,
            _ => operation,
        };
        right = None;
        left = Expression::IsZero(Box::new(left));
        imports.insert(String::from("use brush::traits::AcountIdExt;\n"));
    }

    Condition {
        left,
        operation: if inverted {
            operation.negate()
        } else {
            operation
        },
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
    line: &String,
    constructor: bool,
    storage: &HashMap<String, String>,
    imports: &mut HashSet<String>,
    functions: &HashMap<String, bool>,
) -> Statement {
    let field_type_raw = capture_regex(&REGEX_DECLARE, line, "field_type").unwrap();
    let field_name = capture_regex(&REGEX_DECLARE, line, "field_name").unwrap();
    let value_raw = capture_regex(&REGEX_DECLARE, line, "value");
    let field_type = convert_variable_type(field_type_raw, imports);

    return if let Some(value) = value_raw {
        let expression = parse_member(&value, constructor, storage, imports, functions);
        Statement::Declaration(field_name, field_type, Some(expression))
    } else {
        Statement::Declaration(field_name, field_type, None)
    }
}

fn is_literal(line: &String) -> bool {
    let string_regex = Regex::new(r#"^\s*".*"\s*$"#).unwrap();
    let char_regex = Regex::new(r#"^\s*'.*'\s*$"#).unwrap();
    return line.parse::<i32>().is_ok()
        || string_regex.is_match(line)
        || char_regex.is_match(line)
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
fn parse_function_attributes(attributes: &String) -> (bool, bool, bool) {
    let external = attributes.contains("external") || attributes.contains("public");
    let view = attributes.contains("view") || attributes.contains("pure");
    let payable = attributes.contains("payable");

    (external, view, payable)
}

fn parse_modifiers(attributes: &String) -> Vec<String> {
    let mut adjusted = attributes.clone();
    adjusted.remove_matches("payable");
    adjusted.remove_matches("external");
    adjusted.remove_matches("internal");
    adjusted.remove_matches("virtual");
    adjusted.remove_matches("override");
    adjusted.remove_matches("public");
    adjusted.remove_matches("private");
    adjusted.remove_matches("view");
    adjusted.remove_matches("pure");
    adjusted = trim(&adjusted);
    adjusted = adjusted.replace(", ", ",");
    if adjusted.is_empty() {
        Vec::default()
    } else {
        split(&adjusted, " ", None)
    }
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
    let mut struct_raw = read_until(chars, vec![CURLY_CLOSE]);
    struct_raw = struct_raw.replace(" => ", "=>");
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
    let regex_mapping: Regex = Regex::new(
        r#"(?x)^\s*mapping\(
                (?P<type_from>.+?)=>
                (?P<type_to>.+)
                \)\s*$"#,
    )
    .unwrap();
    if regex_mapping.is_match(&arg_type) {
        imports.insert(String::from("use ink_storage::Mapping;\n"));
        let mut from_raw = capture_regex(&regex_mapping, &arg_type, "type_from").unwrap();
        let mut to_raw = capture_regex(&regex_mapping, &arg_type, "type_to").unwrap();

        let mut from_vec = vec![convert_variable_type(from_raw, imports)];
        while regex_mapping.is_match(&to_raw) {
            from_raw = capture_regex(&regex_mapping, &to_raw, "type_from").unwrap();
            to_raw = capture_regex(&regex_mapping, &to_raw, "type_to").unwrap();
            from_vec.push(convert_variable_type(from_raw, imports));
        }

        let to = convert_variable_type(to_raw, imports);

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
        str if TYPES.contains_key(str) => {
            let the_type = TYPES.get(str).unwrap();
            if let Some(import) = the_type.2 {
                imports.insert(format!("use {import};\n"));
            }
            the_type.0.to_string()
        }
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
    let regex_int: Regex = Regex::new(
        r#"(?x)^\s*
        (?P<int_type>u*int)
        (?P<int_size>[0-9]*)
        \s*$"#,
    )
    .unwrap();

    let int_type_raw = capture_regex(&regex_int, &arg_type, "int_type");
    if let Some(int_type) = int_type_raw {
        let int_size_raw = capture_regex(&regex_int, &arg_type, "int_size").unwrap();
        let int_size_original = if int_size_raw.is_empty() {
            128
        } else {
            int_size_raw.parse::<i32>().unwrap()
        };
        let int_size = match int_size_original {
            i if i <= 8 => 8,
            i if i <= 16 => 16,
            i if i <= 32 => 32,
            i if i <= 64 => 64,
            _ => 128,
        };

        return format!("{int_type}{int_size}")
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
