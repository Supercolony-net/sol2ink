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

#[derive(Debug, PartialEq, Eq)]
enum ArgsReader {
    ArgType,
    ArgName,
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

macro_rules! bx {
    ($e:expr) => {
        Box::new($e)
    };
}

const DEFAULT_ERROR: &str = "SMART CONTRACT MAKE PANIC BEEP BEEP BEEP";

lazy_static! {
    static ref TYPES: HashMap<&'static str, (&'static str, Option<&'static str>, Option<&'static str>)> = {
        // solidityType -> (inkType, initializerMaybe, importMaybe)
        let mut map = HashMap::new();

        map.insert(
            "address",
            ("AccountId", None, None),
        );
        map.insert(
            "hex",
            ("[u8]", Some("&hex::decode"), None),
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
                Some("ink_prelude::vec::Vec"),
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
            ("String", None, Some("ink_prelude::string::String")),
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
        map.insert(String::from("++"), Operation::AddOne);
        map.insert(String::from("--"), Operation::SubtractOne);
        map.insert(String::from("^"), Operation::Xor);
        map
    };
    static ref SPECIFIC_EXPRESSION: HashMap<String, Expression> = {
        let mut map = HashMap::new();
        map.insert(String::from("address(0)"), Expression::ZeroAddressInto);
        map.insert(String::from("address(0x0)"), Expression::ZeroAddressInto);
        map.insert(String::from("msg.sender"), Expression::EnvCaller(None));
        map.insert(String::from("msg.value"), Expression::TransferredValue(None));
        map
    };
    static ref REGEX_RETURN: Regex =
        Regex::new(r#"(?x)^\s*return\s+(?P<output>.+?);*\s*$"#).unwrap();
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
        (,\s*["|'](?P<error>.*)["|']\s*)*\);\s*$"#
    )
    .unwrap();
    static ref REGEX_COMMENT: Regex = Regex::new(r#"(?x)^\s*///*\s*(?P<comment>.*)\s*$"#).unwrap();
    static ref REGEX_CONDITION_ONE_LINE: Regex = Regex::new(
        r#"(?x)
        ^\s*(?P<keyword>(else|if)(\s*if)*)\s*
        (\(\s*(?P<condition>.+)\s*\))*
        (?P<then>.+)\s*;\s*
        $"#
    ).unwrap();
    static ref REGEX_DO: Regex = Regex::new(r#"(?x)^\s*do\s*\{\s*"#).unwrap();
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
        (?P<right>[^=][^;]*)+?;*\s*$"#
    )
    .unwrap();
    static ref REGEX_FUNCTION_CALL: Regex = Regex::new(
        r#"(?x)
        ^\s*(?P<function_name>[a-zA-Z0-9_]+?)\s*\(
        \s*(?P<args>.*)\s*
        \);*\s*$"#,
    )
    .unwrap();
    static ref REGEX_BOOLEAN: Regex = Regex::new(
        r#"(?x)
        ^\s*(?P<left>.+?)
        \s*(?P<operation>[!=><^]+)\s*
        (?P<right>.+)
        \s*$"#,
    )
    .unwrap();
    static ref REGEX_BINARY_SUFFIX: Regex = Regex::new(
        r#"(?x)
        ^\s*(?P<value>.+?)
        \s*(?P<operation>[+-]{2});*
        \s*$"#,
    )
    .unwrap();
    static ref REGEX_BINARY_PREFIX: Regex = Regex::new(
        r#"(?x)
        ^\s*(?P<operation>[+-]{2})
        \s*(?P<value>.+?);*
        \s*$"#,
    )
    .unwrap();
    static ref REGEX_FOR: Regex = Regex::new(
        r#"(?x)
        ^\s*for\s*\(\s*
        (?P<assignment>.+?;)\s*
        (?P<condition>.+?)\s*;
        (?P<modification>.+)\s*
        \)\s*\{$"#,
    )
    .unwrap();
    static ref REGEX_WHILE: Regex = Regex::new(
        r#"(?x)
        ^\s*while\s*\(\s*
        (?P<condition>.+?)\s*
        \);*\s*\{*$"#,
    )
    .unwrap();
    static ref REGEX_TERNARY:Regex = Regex::new(
        r#"(?x)
        ^\s*(?P<condition>.+?)\s*\?
        \s*(?P<if_true>.+?)\s*:
        \s*(?P<if_false>.+?)\s*$"#,
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

const AMPERSAND: char = '&';
const ASTERISK: char = '*';
const BRACKET_CLOSE: char = ']';
const BRACKET_OPEN: char = '[';
const COMMA: char = ',';
const EXCLAMAITON: char = '!';
const EQUALS: char = '=';
const CURLY_CLOSE: char = '}';
const CURLY_OPEN: char = '{';
const MINUS: char = '-';
const NEW_LINE: char = '\n';
const PARENTHESIS_CLOSE: char = ')';
const PARENTHESIS_OPEN: char = '(';
const PERCENT: char = '%';
const PIPE: char = '|';
const PLUS: char = '+';
const SEMICOLON: char = ';';
const SLASH: char = '/';
const SPACE: char = ' ';

pub struct Parser<'a> {
    chars: &'a mut Chars<'a>,
    imports: &'a mut HashSet<String>,
    storage: &'a mut HashMap<String, ContractField>,
    functions: &'a mut HashMap<String, bool>,
    events: &'a mut HashMap<String, Event>,
    modifiers: &'a mut HashMap<String, ()>,
    structs: &'a mut HashMap<String, Struct>,
}

impl<'a> Parser<'a> {
    /// creates a new parser from the given parameters
    pub fn new(
        chars: &'a mut Chars<'a>,
        imports: &'a mut HashSet<String>,
        storage: &'a mut HashMap<String, ContractField>,
        functions: &'a mut HashMap<String, bool>,
        events: &'a mut HashMap<String, Event>,
        modifiers: &'a mut HashMap<String, ()>,
        structs: &'a mut HashMap<String, Struct>,
    ) -> Self {
        Parser {
            chars,
            imports,
            storage,
            functions,
            events,
            modifiers,
            structs,
        }
    }

    /// parses the file represented by the given chars iterator
    ///
    /// returns Some(contract) if a contract was successfully parsed
    /// returns Some(interface) if an interface was successfully parsed
    /// returns None if the file is not a valid contract or interface
    pub fn parse_file(&mut self) -> Result<(Option<Contract>, Option<Interface>), ParserError> {
        let mut comments = Vec::<String>::new();
        let mut action = Action::None;
        let mut buffer = String::new();

        while let Some(ch) = self.chars.next() {
            match ch {
                SLASH if action == Action::None => action = Action::Slash,
                SLASH if action == Action::Slash => {
                    let comment = self.parse_comment();
                    if !comment.is_empty() {
                        comments.push(comment);
                    }
                    action = Action::None;
                }
                ASTERISK if action == Action::Slash => {
                    let mut new_comments = self.parse_multiline_comment();
                    comments.append(&mut new_comments);
                    action = Action::None;
                }
                NEW_LINE | SPACE => {}
                _ => {
                    buffer.push(ch);
                    if buffer == "pragma" || buffer == "import" {
                        read_until(self.chars, vec![SEMICOLON]);
                        buffer.clear();
                    } else if buffer == "abstract" {
                        buffer.clear();
                    } else if buffer == "contract" {
                        let contract = self.parse_contract(comments)?;
                        return Ok((Some(contract), None))
                    } else if buffer == "interface" {
                        let interface = self.parse_interface(comments)?;
                        return Ok((None, Some(interface)))
                    } else if buffer == "library" {
                        return Err(ParserError::LibraryParsingNotImplemented)
                    }
                }
            }
        }

        Ok((None, None))
    }

    /// parses a line containing a comment and returns it as a string
    fn parse_comment(&mut self) -> String {
        let mut buffer = String::new();
        let mut reading = false;

        for ch in self.chars.by_ref() {
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

    /// parses mulitline comment starting with /* and ending with */
    /// returns the comments as a vector of strings
    fn parse_multiline_comment(&mut self) -> Vec<String> {
        let mut comments = Vec::<String>::new();
        let mut buffer = String::new();
        let mut reading = false;
        let mut new_line = false;
        let mut asterisk = false;

        for ch in self.chars.by_ref() {
            if ch == SLASH && asterisk {
                if !buffer.trim().is_empty() {
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
                    if !buffer.trim().is_empty() {
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
    /// `contract_doc` the documentation comments of the contract
    ///
    /// returns the representation of the contract as `Contract` struct
    fn parse_contract(&mut self, contract_doc: Vec<String>) -> Result<Contract, ParserError> {
        let mut buffer = String::new();
        let mut action = Action::None;

        let mut name = String::new();
        let mut comments = Vec::<String>::new();
        let mut fields = Vec::<ContractField>::new();
        let mut events = Vec::<Event>::new();
        let mut enums = Vec::<Enum>::new();
        let mut structs = Vec::<Struct>::new();
        let mut functions = Vec::<Function>::new();
        let mut constructor = Function::default();
        let mut modifiers = Vec::<Modifier>::new();

        while let Some(ch) = self.chars.next() {
            match ch {
                NEW_LINE if action == Action::None || action == Action::Contract => {}
                SLASH if action == Action::Contract => action = Action::Slash,
                SLASH if action == Action::Slash => {
                    let comment = self.parse_comment();
                    if !comment.is_empty() {
                        comments.push(comment);
                    }
                    action = Action::Contract;
                }
                ASTERISK if action == Action::Slash => {
                    let mut new_comments = self.parse_multiline_comment();
                    comments.append(&mut new_comments);
                    action = Action::Contract;
                }
                SPACE | CURLY_OPEN if action == Action::ContractName => {
                    name = buffer.trim().to_string();
                    buffer.clear();
                    // we skip everything regarding generalization
                    // TODO: cover generaliztaion
                    if ch != CURLY_OPEN {
                        read_until(self.chars, vec![CURLY_OPEN]);
                    }
                    action = Action::Contract;
                }
                _ if action == Action::None => {
                    buffer.push(ch);
                    action = Action::ContractName;
                }
                SEMICOLON if action == Action::Contract => {
                    buffer.push(ch);
                    fields.push(self.parse_contract_field(buffer.trim(), &comments));
                    buffer.clear();
                    comments.clear();
                }
                _ if action == Action::ContractName || action == Action::Contract => {
                    buffer.push(ch);
                    match buffer.trim() {
                        "event" => {
                            let event = self.parse_event(&comments);
                            self.events.insert(event.name.clone(), event.clone());
                            events.push(event);
                            comments.clear();
                            buffer.clear();
                        }
                        "enum" => {
                            enums.push(self.parse_enum(&comments));
                            comments.clear();
                            buffer.clear();
                        }
                        "struct" => {
                            structs.push(self.parse_struct(&comments));
                            comments.clear();
                            buffer.clear();
                        }
                        "function" => {
                            functions.push(self.parse_function(&comments)?);
                            comments.clear();
                            buffer.clear();
                        }
                        "constructor" => {
                            constructor = self.parse_function(&comments)?;
                            comments.clear();
                            buffer.clear();
                        }
                        "modifier" => {
                            modifiers.push(self.parse_modifier(&comments)?);
                            buffer.clear();
                            comments.clear();
                        }
                        "using" => {
                            read_until(self.chars, vec![SEMICOLON]);
                            buffer.clear();
                        }
                        "receive" | "fallback" => {
                            let mut function = self.parse_function(&comments)?;
                            function.header.name = buffer.trim().to_owned();
                            functions.push(function);
                            comments.clear();
                            buffer.clear();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        for contract_field in fields.iter() {
            self.storage
                .insert(contract_field.name.clone(), contract_field.clone());
        }
        for function in functions.iter() {
            self.functions
                .insert(function.header.name.clone(), function.header.external);
        }
        for modifier in modifiers.iter() {
            self.modifiers.insert(modifier.header.name.clone(), ());
        }
        for structure in structs.iter() {
            self.structs
                .insert(structure.name.clone(), structure.clone());
        }

        // now we know the contracts members and we can parse statements
        for function in functions.iter_mut() {
            function.header.modifiers = self.process_function_modifiers(&function.header.modifiers);
            function.body = self.parse_statements(&function.body, false);
        }
        for modifier in modifiers.iter_mut() {
            modifier.statements = self.parse_statements(&modifier.statements, false);
        }
        constructor.body = self.parse_statements(&constructor.body, true);

        Ok(Contract {
            name,
            fields,
            constructor,
            events,
            enums,
            structs,
            functions,
            imports: self.imports.clone(),
            contract_doc,
            modifiers,
        })
    }

    /// Parses the code of a Solidity interface
    ///
    /// `contract_doc` the documentation comments of the interface
    ///
    /// returns the representation of the interface as `Interface` struct
    pub fn parse_interface(
        &mut self,
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

        while let Some(ch) = self.chars.next() {
            match ch {
                NEW_LINE if action == Action::None || action == Action::Contract => {}
                SLASH if action == Action::Contract => action = Action::Slash,
                SLASH if action == Action::Slash => {
                    let comment = self.parse_comment();
                    if !comment.is_empty() {
                        comments.push(comment);
                    }
                    action = Action::Contract;
                }
                ASTERISK if action == Action::Slash => {
                    let mut new_comments = self.parse_multiline_comment();
                    comments.append(&mut new_comments);
                    action = Action::Contract;
                }
                CURLY_OPEN => {
                    action = Action::Contract;
                }
                SPACE if action == Action::ContractName => {
                    name = buffer.trim().substring(1, buffer.len()).to_owned();
                    buffer.clear();
                    action = Action::ContractNamed;
                }
                _ if action == Action::None => {
                    buffer.push(ch);
                    action = Action::ContractName;
                }
                _ if action == Action::ContractName || action == Action::Contract => {
                    buffer.push(ch);
                    if buffer.trim() == "event" {
                        events.push(self.parse_event(&comments));
                        comments.clear();
                        buffer.clear();
                    } else if buffer.trim() == "enum" {
                        enums.push(self.parse_enum(&comments));
                        comments.clear();
                        buffer.clear();
                    } else if buffer.trim() == "struct" {
                        structs.push(self.parse_struct(&comments));
                        comments.clear();
                        buffer.clear();
                    } else if buffer.trim() == "function" {
                        let function_header = self.parse_function_header(&comments);
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
            imports: self.imports.clone(),
            comments: contract_comments,
        })
    }

    /// Parses a field of the contract
    ///
    /// `line` the raw representation of the field
    /// `imports` the HashSet of imports of the contract
    ///
    /// returns the representation of contract field as `ContractField` struct
    fn parse_contract_field(&mut self, line_raw: &str, comments: &[String]) -> ContractField {
        let mut line = line_raw.to_owned();
        line = Regex::new(r"\s*=>\s*")
            .unwrap()
            .replace_all(&line, "=>")
            .to_string();
        line = Regex::new(r"\s*\(\s*")
            .unwrap()
            .replace_all(&line, "(")
            .to_string();
        line = Regex::new(r"\s*\)\s*")
            .unwrap()
            .replace_all(&line, ")")
            .to_string();

        let regex: Regex = Regex::new(
            r#"(?x)^\s*
        (?P<field_type>.+?(\s|\))+)
        (?P<attributes>(\s*constant\s*|\s*private\s*|\s*public\s*|\s*immutable\s*|\s*override\s*)*)*
        (?P<field_name>.+?)\s*
        (=\s*(?P<initial_value>[^>].+)\s*)*
        ;\s*$"#,
        )
        .unwrap();

        let field_type_raw = capture_regex(&regex, &line, "field_type").unwrap();
        let attributes_raw = capture_regex(&regex, &line, "attributes");
        let field_name = capture_regex(&regex, &line, "field_name").unwrap();
        let initial_value_maybe = capture_regex(&regex, &line, "initial_value");
        let initial_value =
            initial_value_maybe.map(|initial_raw| self.parse_expression(&initial_raw, false, None));
        let constant = attributes_raw
            .unwrap_or_else(|| String::from(""))
            .contains("constant");
        let field_type = self.convert_variable_type(trim(&field_type_raw));

        ContractField {
            field_type,
            name: field_name,
            comments: comments.to_vec(),
            initial_value,
            constant,
        }
    }

    /// Parses Solidity event
    ///
    /// `comments` the documentation comments of the event
    ///
    /// returns the event definition as `Event` struct
    fn parse_event(&mut self, comments: &[String]) -> Event {
        let event_raw = read_until(self.chars, vec![SEMICOLON])
            .trim()
            .replace("( ", "(")
            .replace(" )", ")");

        let tokens = split(&event_raw, " ", None);
        let mut args_reader = ArgsReader::ArgName;
        let mut indexed = false;

        let split_brace = split(&tokens[0], "(", None);

        let name = split_brace[0].to_owned();
        let mut field_type = self.convert_variable_type(split_brace[1].to_owned());
        let mut fields = Vec::<EventField>::new();

        for item in tokens.iter().skip(1) {
            let mut token = item.clone();
            if token == "indexed" {
                indexed = true;
                continue
            } else if args_reader == ArgsReader::ArgType {
                field_type = self.convert_variable_type(token);
                args_reader = ArgsReader::ArgName;
            } else {
                token.remove_matches(&[',', ')'][..]);
                fields.push(EventField {
                    indexed,
                    field_type: field_type.to_owned(),
                    name: token.to_owned(),
                });
                indexed = false;
                args_reader = ArgsReader::ArgType;
            }
        }

        Event {
            name,
            fields,
            comments: comments.to_vec(),
        }
    }

    /// Parses Solidity enum
    ///
    /// `comments` the documentation comments of the enum
    ///
    /// returns the enum as `Enum` struct
    fn parse_enum(&mut self, comments: &[String]) -> Enum {
        let enum_raw = read_until(self.chars, vec![CURLY_CLOSE]);
        let tokens = split(&enum_raw, " ", None);
        let name = tokens[0].to_owned();
        let mut values = Vec::<String>::new();

        for item in tokens.iter().skip(1) {
            let mut token = item.clone();
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

    /// Parses Solidity structure
    ///
    /// `comments` the documentation comments of the struct
    ///
    /// returns the struct definition as `Struct` struct
    fn parse_struct(&mut self, comments: &[String]) -> Struct {
        let mut struct_raw = read_until(self.chars, vec![CURLY_CLOSE]);
        struct_raw = struct_raw.replace(" => ", "=>");
        let split_brace = split(&struct_raw, "{", None);
        let fields = split(split_brace[1].trim(), ";", None);
        let struct_name = split_brace[0].to_owned();

        let mut struct_fields = Vec::<StructField>::new();

        for field in fields.iter() {
            if field.is_empty() {
                continue
            }
            struct_fields.push(self.parse_struct_field(field.trim().to_owned()));
        }

        Struct {
            name: struct_name,
            fields: struct_fields,
            comments: comments.to_vec(),
        }
    }

    /// Parses struct fields
    ///
    /// `line` the Solidity definition of the struct field
    ///
    /// returns the struct field as `StructField` struct
    fn parse_struct_field(&mut self, line: String) -> StructField {
        let tokens = split(&line, " ", None);
        let field_type = self.convert_variable_type(tokens[0].to_owned());
        let mut name = tokens[1].to_owned();
        name.remove_matches(";");
        StructField { name, field_type }
    }

    /// Parses the Solidity function
    ///
    /// `comments` the documentation comments of the function
    ///
    /// returns the function definition as `Function` struct
    fn parse_function(&mut self, comments: &[String]) -> Result<Function, ParserError> {
        Ok(Function {
            header: self.parse_function_header(comments),
            body: self.parse_body(),
        })
    }

    /// Parses the function header of a Solidity function
    ///
    /// `comments` the documentation comments of the function
    ///
    /// returns the representation of the function header as `FunctionHeader` struct
    fn parse_function_header(&mut self, comments: &[String]) -> FunctionHeader {
        let mut function_header_raw = read_until(self.chars, vec![SEMICOLON, CURLY_OPEN]);
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

        let (name, params, return_params, modifiers) = if let Some(return_parameters_raw) =
            return_parameters_maybe
        {
            let function_name = capture_regex(
                &regex_return_function,
                &function_header_raw,
                "function_name",
            )
            .unwrap();
            let parameters_raw =
                capture_regex(&regex_return_function, &function_header_raw, "parameters").unwrap();
            let parameters = self.parse_function_parameters(parameters_raw);
            let return_parameters = self.parse_return_parameters(return_parameters_raw);
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
            let parameters = self.parse_function_parameters(parameters_raw);
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

    /// Parses parameters of a function
    ///
    /// `parameters` the raw representation of the paramters of the function
    ///
    /// returns the vec of function parameters of this function as `FunctionParam` struct
    fn parse_function_parameters(&mut self, parameters: String) -> Vec<FunctionParam> {
        let mut out = Vec::<FunctionParam>::new();

        if !parameters.is_empty() {
            let tokens = split(&parameters, " ", Some(remove_commas()));

            let mut mode = ArgsReader::ArgName;
            let mut param_type = self.convert_variable_type(tokens[0].to_owned());

            for item in tokens.iter().skip(1) {
                if mode == ArgsReader::ArgType {
                    param_type = self.convert_variable_type(item.to_owned());
                    mode = ArgsReader::ArgName;
                } else if mode == ArgsReader::ArgName {
                    let name = item.to_owned();
                    out.push(FunctionParam {
                        name,
                        param_type: param_type.to_owned(),
                    });
                    mode = ArgsReader::ArgType;
                }
            }
        }

        out
    }

    /// Parses return parameters of a function
    ///
    /// `parameters` the String which contains the return paramters of the function
    ///
    /// returns the vec of function return parameters of this function
    fn parse_return_parameters(&mut self, parameters: String) -> Vec<FunctionParam> {
        let mut out = Vec::<FunctionParam>::new();
        let tokens: Vec<String> = split(&parameters, " ", None);

        let mut iterator = tokens.iter();
        while let Some(token) = iterator.next() {
            let mut param_raw = token.to_owned();
            param_raw.remove_matches(",");
            let param_type = self.convert_variable_type(param_raw);
            let mut name = if tokens.len() >= (parameters.matches(',').count() + 1) * 2 {
                iterator.next().unwrap().to_owned()
            } else {
                String::from("_")
            };
            name.remove_matches(",");
            out.push(FunctionParam { name, param_type })
        }

        out
    }

    /// Parses the body of a function or a modifier and returns them in a vector of `Statement`
    fn parse_body(&mut self) -> Vec<Statement> {
        let mut buffer = String::new();
        let mut open_braces = 1;
        let mut close_braces = 0;
        let mut statements = Vec::<Statement>::new();
        let mut action = Action::None;

        while let Some(ch) = self.chars.next() {
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
                let regex_struct_initializer =
                    Regex::new(r#"(?x)^\s*(?P<code>.+)\s*\(\{$"#).unwrap();
                if regex_struct_initializer.is_match(&buffer) {
                    let left_code =
                        capture_regex(&regex_struct_initializer, &buffer, "code").unwrap();
                    let right_code = read_until(self.chars, vec![';']);
                    buffer = format!("{left_code}({{{right_code}");
                    close_braces += 1;
                }
                statements.push(Statement::Raw(buffer.clone()));
                if action == Action::Assembly {
                    action = Action::None;
                }
                buffer.clear();
            } else if ch == SLASH {
                let next_maybe = self.chars.next();
                if next_maybe == Some(SLASH) {
                    statements.push(Statement::Raw(format!("// {}", self.parse_comment())));
                    continue
                } else if next_maybe == Some(ASTERISK) {
                    for comment in self.parse_multiline_comment().iter() {
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
                    let for_block = read_until(self.chars, vec!['{']);
                    statements.push(Statement::Raw(format!("for{for_block}{{")));
                    buffer.clear();
                }
            }
        }

        statements
    }

    /// Parses the Solidity modifier
    ///
    /// `comments` the documentation comments of the modifier
    ///
    /// returns the representation of the modifier header as `Modifier` struct
    fn parse_modifier(&mut self, comments: &[String]) -> Result<Modifier, ParserError> {
        self.imports
            .insert(String::from("use openbrush::modifier_definition;"));
        self.imports
            .insert(String::from("use openbrush::modifiers;"));
        Ok(Modifier {
            header: self.parse_function_header(comments),
            statements: self.parse_body(),
            comments: comments.to_vec(),
        })
    }

    /// Parses all modifiers of a function and returns them as a vector of `Modifier` expressions
    ///
    /// `raw_modifiers` the raw representations of modifiers of the function
    ///
    /// returns the vector of `Modifier` expressions as function calls of the specific modifiers
    fn process_function_modifiers(&mut self, raw_modifiers: &[Expression]) -> Vec<Expression> {
        let regex_modifier_name = Regex::new(r#"(?x)^\s*(?P<name>.+?)\(.\)*"#).unwrap();
        let mut out = Vec::default();
        for raw_modifier in raw_modifiers.iter() {
            if let Expression::Modifier(modifier) = raw_modifier {
                let modifier_name =
                    capture_regex(&regex_modifier_name, modifier, "name").unwrap_or_default();
                if self.modifiers.contains_key(&modifier_name) {
                    let function_call = self.parse_function_call(modifier, false, None);
                    out.push(function_call)
                }
            }
        }
        out
    }

    /// Parses raw statements of a function or modifier and returns them in a vector of `Statement`
    ///
    /// `statements` the raw statements of the function or modifier
    /// `constructor` if the function is a constructor
    fn parse_statements(&mut self, statements: &[Statement], constructor: bool) -> Vec<Statement> {
        let mut iterator = statements.iter();
        let mut stack = VecDeque::<Block>::new();
        let mut out = Vec::default();

        while let Some(statement) = iterator.next() {
            if let Statement::Raw(line_raw) = statement {
                out.push(self.parse_statement(line_raw, constructor, &mut stack, &mut iterator));
            }
        }

        out
    }

    /// Parses a soldity statement and returns it in a form of `Statement`
    ///
    /// `line_raw` the solidity statement
    /// `constructor` if the statement is inside a constructor
    /// `stack` the current statement stack
    /// `iterator` the iterator of the statements of currently parsed function
    ///
    /// returns the statement as `Statement` struct
    fn parse_statement(
        &mut self,
        line_raw: &str,
        constructor: bool,
        stack: &mut VecDeque<Block>,
        iterator: &mut Iter<Statement>,
    ) -> Statement {
        let mut line = trim(line_raw);
        line = line.replace(" memory ", " ");
        line = line.replace(" calldata ", " ");
        line = line.replace(" storage ", " ");

        if line == "_;" {
            return Statement::ModifierBody
        } else if REGEX_RETURN.is_match(&line) {
            return self.parse_return(&line)
        } else if REGEX_DECLARE.is_match(&line) {
            return self.parse_declaration(&line, constructor)
        } else if REGEX_REQUIRE.is_match(&line) {
            return self.parse_require(&line, constructor)
        } else if REGEX_COMMENT.is_match(&line) {
            let comment = capture_regex(&REGEX_COMMENT, &line, "comment").unwrap();
            return Statement::Comment(comment)
        } else if REGEX_CONDITION_ONE_LINE.is_match(&line) {
            return self.parse_condition_one_line(&line, constructor, stack, iterator)
        } else if REGEX_IF.is_match(&line) {
            stack.push_back(Block::If);
            return self.parse_if(&line, constructor, stack, iterator)
        } else if REGEX_ELSE.is_match(&line) {
            stack.push_back(Block::Else);
            return self.parse_else(constructor, stack, iterator)
        } else if REGEX_ELSE_IF.is_match(&line) {
            stack.push_back(Block::ElseIf);
            return self.parse_else_if(&line, constructor, stack, iterator)
        } else if REGEX_UNCHECKED.is_match(&line) {
            stack.push_back(Block::Unchecked);
            return Statement::Comment(String::from("Please handle unchecked blocks manually >>>"))
        } else if REGEX_END_BLOCK.is_match(&line) {
            if stack.is_empty() {
                return Statement::Comment(String::from(
                    "Sol2Ink Not Implemented yet End Block here",
                ))
            }
            match stack.pop_back().unwrap() {
                Block::Assembly => return Statement::AssemblyEnd,
                Block::Catch => return Statement::CatchEnd,
                Block::Unchecked => {}
                Block::If => return Statement::IfEnd,
                Block::Else => return Statement::IfEnd,
                Block::ElseIf => return Statement::IfEnd,
                Block::Try => return Statement::TryEnd,
                Block::While => return Statement::WhileEnd,
            }
            return Statement::Comment(String::from("<<< Please handle unchecked blocks manually"))
        } else if REGEX_TRY.is_match(&line) {
            stack.push_back(Block::Try);
            return self.parse_try(&line, constructor, stack, iterator)
        } else if REGEX_FOR.is_match(&line) {
            stack.push_back(Block::While);
            return self.parse_for(&line, constructor, stack, iterator)
        } else if REGEX_DO.is_match(&line) {
            stack.push_back(Block::While);
            return self.parse_do(constructor, stack, iterator)
        } else if REGEX_WHILE.is_match(&line) {
            stack.push_back(Block::While);
            return self.parse_while(&line, constructor, stack, iterator)
        } else if REGEX_ASSEMBLY.is_match(&line) {
            stack.push_back(Block::Assembly);
            return self.parse_assembly(stack, iterator)
        } else if REGEX_CATCH.is_match(&line) {
            stack.push_back(Block::Catch);
            return self.parse_catch(&line, constructor, stack, iterator)
        } else if REGEX_EMIT.is_match(&line) {
            return self.parse_emit(&line, constructor)
        } else if REGEX_ASSIGN.is_match(&line) {
            return self.parse_assign(&line, constructor)
        } else if REGEX_TERNARY.is_match(&line) {
            return self.parse_ternary_statement(&line, constructor, stack, iterator)
        } else if REGEX_BINARY_SUFFIX.is_match(&line) {
            return self.parse_binary_operation(&line, constructor, &REGEX_BINARY_SUFFIX, None)
        } else if REGEX_BINARY_PREFIX.is_match(&line) {
            return self.parse_binary_operation(&line, constructor, &REGEX_BINARY_PREFIX, None)
        } else if REGEX_FUNCTION_CALL.is_match(&line) {
            let expression = self.parse_function_call(&line, constructor, None);
            return Statement::FunctionCall(expression)
        }

        Statement::Comment(format!("Sol2Ink Not Implemented yet: {}", line.clone()))
    }

    /// Parses a return statement
    ///
    /// `line` the soldiity return statement
    ///
    /// returns the statements in form of `Statement::Return`
    fn parse_return(&mut self, line: &str) -> Statement {
        let raw_output = capture_regex(&REGEX_RETURN, line, "output").unwrap();
        let output = self.parse_expression(&raw_output, false, None);

        Statement::Return(output)
    }

    /// Parses a declaration statement
    ///
    /// `line` the soldity declaration statement
    ///
    /// returns the statements in form of `Statement::Declaration`
    fn parse_declaration(&mut self, line: &str, constructor: bool) -> Statement {
        let field_type_raw = capture_regex(&REGEX_DECLARE, line, "field_type").unwrap();
        let field_name = capture_regex(&REGEX_DECLARE, line, "field_name").unwrap();
        let value_raw = capture_regex(&REGEX_DECLARE, line, "value");
        let field_type = self.convert_variable_type(field_type_raw);

        if let Some(value) = value_raw {
            let expression = self.parse_expression(&value, constructor, None);
            Statement::Declaration(field_name, field_type, Some(expression))
        } else {
            Statement::Declaration(field_name, field_type, None)
        }
    }

    /// Parses a require statement
    ///
    /// `line` the soldity require statement
    /// `constructor` whether the require is in a constructor or not
    ///
    /// returns the statements in form of `Statement::Require`
    fn parse_require(&mut self, line: &str, constructor: bool) -> Statement {
        self.imports
            .insert(String::from("use ink_prelude::string::String;"));

        let condition = capture_regex(&REGEX_REQUIRE, line, "condition");
        let error = capture_regex(&REGEX_REQUIRE, line, "error");

        let condition = self.parse_condition(&condition.unwrap(), constructor, true, None);
        let error_output = if constructor {
            format!(
                "panic!(\"{}\")",
                error.unwrap_or_else(|| DEFAULT_ERROR.to_owned())
            )
        } else {
            format!(
                "return Err(Error::Custom(String::from(\"{}\")))",
                error.unwrap_or_else(|| DEFAULT_ERROR.to_owned())
            )
        };

        Statement::Require(condition, error_output)
    }

    /// Parses a solidity condition which is not enclosed in curly brackets
    ///
    /// `line_raw` the solidity condition
    /// `constructor` if the statement is inside a constructor
    /// `stack` the current statement stack
    /// `iterator` the iterator of the statements of currently parsed function
    ///
    /// Return the statement in form of `Statement::If` or `Statement::ElseIf` or `Statement::Else`
    fn parse_condition_one_line(
        &mut self,
        line: &str,
        constructor: bool,
        stack: &mut VecDeque<Block>,
        iterator: &mut Iter<Statement>,
    ) -> Statement {
        let keyword = capture_regex(&REGEX_CONDITION_ONE_LINE, line, "keyword").unwrap();
        let condition_raw = capture_regex(&REGEX_CONDITION_ONE_LINE, line, "condition").unwrap();
        let then_raw = capture_regex(&REGEX_CONDITION_ONE_LINE, line, "then").unwrap();
        let then = self.parse_statement(&then_raw, constructor, stack, iterator);
        let statements = vec![then];

        let condition = if keyword == "if" || keyword == "else if" {
            Some(self.parse_condition(&condition_raw, constructor, false, None))
        } else {
            None
        };

        match condition {
            Some(condition) => {
                if keyword == "if" {
                    Statement::If(condition, statements)
                } else {
                    Statement::ElseIf(condition, statements)
                }
            }
            None => Statement::Else(statements),
        }
    }

    /// Parses a solidity if statement and the statements inside the if block
    ///
    /// `line` the solidity condition
    /// `constructor` if the statement is inside a constructor
    /// `stack` the current statement stack
    /// `iterator` the iterator of the statements of currently parsed function
    ///
    /// Return the statement in form of `Statement::If`
    fn parse_if(
        &mut self,
        line: &str,
        constructor: bool,
        stack: &mut VecDeque<Block>,
        iterator: &mut Iter<Statement>,
    ) -> Statement {
        let condition_raw = capture_regex(&REGEX_IF, line, "condition").unwrap();
        let condition = self.parse_condition(&condition_raw, constructor, false, None);
        let mut statements = Vec::default();

        self.parse_block(
            constructor,
            stack,
            iterator,
            &mut statements,
            Statement::IfEnd,
        );

        Statement::If(condition, statements)
    }

    /// Parses a solidity else statement and the statements inside the else block
    ///
    /// `line` the solidity condition
    /// `constructor` if the statement is inside a constructor
    /// `stack` the current statement stack
    /// `iterator` the iterator of the statements of currently parsed function
    ///
    /// Return the statement in form of `Statement::ElseIf`
    fn parse_else_if(
        &mut self,
        line: &str,
        constructor: bool,
        stack: &mut VecDeque<Block>,
        iterator: &mut Iter<Statement>,
    ) -> Statement {
        let condition_raw = capture_regex(&REGEX_ELSE_IF, line, "condition");
        let condition = self.parse_condition(&condition_raw.unwrap(), constructor, false, None);
        let mut statements = Vec::default();

        self.parse_block(
            constructor,
            stack,
            iterator,
            &mut statements,
            Statement::IfEnd,
        );

        Statement::ElseIf(condition, statements)
    }

    /// Parses a solidity else statement and the statements inside the else block
    ///
    /// `constructor` if the statement is inside a constructor
    /// `stack` the current statement stack
    /// `iterator` the iterator of the statements of currently parsed function
    ///
    /// Return the statement in form of `Statement::Else`
    fn parse_else(
        &mut self,
        constructor: bool,
        stack: &mut VecDeque<Block>,
        iterator: &mut Iter<Statement>,
    ) -> Statement {
        let mut statements = Vec::default();

        self.parse_block(
            constructor,
            stack,
            iterator,
            &mut statements,
            Statement::IfEnd,
        );

        Statement::Else(statements)
    }

    /// Parses a solidity condition and returns it as `Condition` struct
    ///
    /// `line` the raw representation of the solidity condition
    /// `constructor` if the condition is inside a constructor
    /// `inverted` if the condition is inverted (for example in require statements)
    /// `enclosed_expressions` the previously parsed enclosed expressions
    fn parse_condition(
        &mut self,
        line: &String,
        constructor: bool,
        inverted: bool,
        enclosed_expressions: Option<HashMap<String, Expression>>,
    ) -> Condition {
        let (mut left, mut operation, mut right) = if REGEX_BOOLEAN.is_match(line) {
            let left_raw = capture_regex(&REGEX_BOOLEAN, line, "left").unwrap();
            let operation_raw = capture_regex(&REGEX_BOOLEAN, line, "operation").unwrap();
            let right_raw = capture_regex(&REGEX_BOOLEAN, line, "right").unwrap();

            let left = self.parse_expression(&left_raw, constructor, enclosed_expressions.clone());
            let operation = *OPERATIONS.get(&operation_raw).unwrap();
            let right = self.parse_expression(&right_raw, constructor, enclosed_expressions);

            (left, operation, Some(right))
        } else {
            let regex_negative = Regex::new(r#"(?x)^\s*!(?P<value>.+?)\s*$"#).unwrap();
            if regex_negative.is_match(line) {
                let left_raw = capture_regex(&regex_negative, line, "value").unwrap();
                let left = self.parse_expression(&left_raw, constructor, enclosed_expressions);
                (left, Operation::Not, None)
            } else {
                let left = self.parse_expression(line, constructor, enclosed_expressions);
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
            left = Expression::IsZero(bx!(left));
            self.imports
                .insert(String::from("use openbrush::traits::AccountIdExt;\n"));
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

    /// Parses a solidity try statement and the statements inside the try block
    /// The try statement is inserted as a comment and the actual try statement
    /// is rendered as if true block
    ///
    /// `line` the solidity condition
    /// `constructor` if the statement is inside a constructor
    /// `stack` the current statement stack
    /// `iterator` the iterator of the statements of currently parsed function
    ///
    /// Return the statement in form of `Statement::Try`
    fn parse_try(
        &mut self,
        line: &str,
        constructor: bool,
        stack: &mut VecDeque<Block>,
        iterator: &mut Iter<Statement>,
    ) -> Statement {
        let mut statements = Vec::default();
        statements.push(Statement::Comment(line.to_owned()));

        self.parse_block(
            constructor,
            stack,
            iterator,
            &mut statements,
            Statement::TryEnd,
        );

        Statement::Try(statements)
    }

    /// Parses a solidity catch statement and the statements inside the catch block
    /// The catch statement is inserted as a comment and the actual catch statement
    /// is rendered as if false block
    ///
    /// `line` the solidity condition
    /// `constructor` if the statement is inside a constructor
    /// `stack` the current statement stack
    /// `iterator` the iterator of the statements of currently parsed function
    ///
    /// Return the statement in form of `Statement::Catch`
    fn parse_catch(
        &mut self,
        line: &str,
        constructor: bool,
        stack: &mut VecDeque<Block>,
        iterator: &mut Iter<Statement>,
    ) -> Statement {
        let mut statements = Vec::default();
        statements.push(Statement::Comment(line.to_owned()));

        self.parse_block(
            constructor,
            stack,
            iterator,
            &mut statements,
            Statement::CatchEnd,
        );

        Statement::Catch(statements)
    }

    /// Parses a solidity for statement and the statements inside the for block
    ///
    /// `line` the solidity condition
    /// `constructor` if the statement is inside a constructor
    /// `stack` the current statement stack
    /// `iterator` the iterator of the statements of currently parsed function
    ///
    /// Return the statement in form of `Statement::While`
    fn parse_for(
        &mut self,
        line: &str,
        constructor: bool,
        stack: &mut VecDeque<Block>,
        iterator: &mut Iter<Statement>,
    ) -> Statement {
        let assignment_raw = capture_regex(&REGEX_FOR, line, "assignment").unwrap();
        let condition_raw = capture_regex(&REGEX_FOR, line, "condition").unwrap();
        let modification_raw = capture_regex(&REGEX_FOR, line, "modification").unwrap();

        let assignment = self.parse_statement(&assignment_raw, constructor, stack, iterator);
        let condition = self.parse_expression(&condition_raw, constructor, None);
        let modification = self.parse_statement(&modification_raw, constructor, stack, iterator);

        let mut statements = Vec::default();

        self.parse_block(
            constructor,
            stack,
            iterator,
            &mut statements,
            Statement::WhileEnd,
        );

        Statement::While(
            Some(bx!(assignment)),
            condition,
            Some(bx!(modification)),
            statements,
        )
    }

    /// Parses a solidity do/while statement and the statements inside the do block
    ///
    /// `constructor` if the statement is inside a constructor
    /// `stack` the current statement stack
    /// `iterator` the iterator of the statements of currently parsed function
    ///
    /// Return the statement in form of `Statement::Loop`
    fn parse_do(
        &mut self,
        constructor: bool,
        stack: &mut VecDeque<Block>,
        iterator: &mut Iter<Statement>,
    ) -> Statement {
        let mut statements = Vec::default();

        self.parse_block(
            constructor,
            stack,
            iterator,
            &mut statements,
            Statement::WhileEnd,
        );

        let next_statement = iterator.next().unwrap();
        let condition = if let Statement::Raw(content) = next_statement {
            let condition_raw = capture_regex(&REGEX_WHILE, content, "condition").unwrap();
            self.parse_expression(&condition_raw, constructor, None)
        } else {
            panic!("Expected Raw statement after do block")
        };

        Statement::Loop(None, condition, None, statements)
    }

    /// Parses a solidity while statement and the statements inside the while block
    ///
    /// `line` the solidity while condition
    /// `constructor` if the statement is inside a constructor
    /// `stack` the current statement stack
    /// `iterator` the iterator of the statements of currently parsed function
    ///
    /// Return the statement in form of `Statement::While`
    fn parse_while(
        &mut self,
        line: &str,
        constructor: bool,
        stack: &mut VecDeque<Block>,
        iterator: &mut Iter<Statement>,
    ) -> Statement {
        let condition_raw = capture_regex(&REGEX_WHILE, line, "condition").unwrap();
        let condition = self.parse_expression(&condition_raw, constructor, None);
        let mut statements = Vec::default();

        self.parse_block(
            constructor,
            stack,
            iterator,
            &mut statements,
            Statement::WhileEnd,
        );

        Statement::While(None, condition, None, statements)
    }

    /// Parses a block of statements (if, else if, else, ...)
    ///
    /// `constructor` if the statement is inside a constructor
    /// `stack` the current statement stack
    /// `iterator` the iterator of the statements of currently parsed function
    /// `statements` the vector of statements into which we are adding the parsed statements
    /// `until` the statement which ends the block
    fn parse_block(
        &mut self,
        constructor: bool,
        stack: &mut VecDeque<Block>,
        iterator: &mut Iter<Statement>,
        statements: &mut Vec<Statement>,
        until: Statement,
    ) {
        while let Some(statement_raw) = iterator.next() {
            if let Statement::Raw(line_raw) = statement_raw {
                let statement = self.parse_statement(line_raw, constructor, stack, iterator);
                if statement == until {
                    break
                } else {
                    statements.push(statement)
                }
            }
        }
    }

    /// Parses a solidity assembly statement and the statements inside the assembly block
    /// A notice is inserted to the beginning and to the end of the assembly block
    /// All statements of the assembly block are parsed as a `Statement::Comment`
    ///
    /// `stack` the current statement stack
    /// `iterator` the iterator of the statements of currently parsed function
    ///
    /// Return the statement in form of `Statement::Group`
    fn parse_assembly(
        &self,
        stack: &mut VecDeque<Block>,
        iterator: &mut Iter<Statement>,
    ) -> Statement {
        let mut statements = Vec::default();
        statements.push(Statement::Comment(String::from(
            "Please handle assembly blocks manually >>>",
        )));

        for statement_raw in iterator.by_ref() {
            if let Statement::Raw(content_raw) = statement_raw {
                let content = trim(content_raw);
                if content == "}" {
                    stack.pop_back();
                    statements.push(Statement::AssemblyEnd);
                    break
                } else {
                    statements.push(Statement::Comment(content.clone()));
                }
            }
        }

        statements.push(Statement::Comment(String::from(
            "<<< Please handle assembly blocks manually",
        )));

        Statement::Group(statements)
    }

    /// Parses a solidity emit statement
    ///
    /// `line` the solidity line of the emit statement
    /// `constructor` if the statement is inside a constructor
    ///
    /// Return the statement in form of `Statement::Emit`
    fn parse_emit(&mut self, line: &str, constructor: bool) -> Statement {
        let event_name_raw = capture_regex(&REGEX_EMIT, line, "event_name").unwrap();
        let args_raw = capture_regex(&REGEX_EMIT, line, "args").unwrap();

        let mut args = Vec::<Expression>::new();
        let mut buffer = String::new();
        let mut open_parentheses = 0;
        let mut close_parenthesis = 0;
        let mut event_count = 0;

        for ch in args_raw.chars() {
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
                            self.events
                                .get(&event_name_raw)
                                .unwrap_or_else(|| panic!("Event {event_name_raw} not defined"))
                                .fields
                                .get(event_count)
                                .unwrap()
                                .name
                                .clone(),
                            bx!(self.parse_expression(&trim(&buffer), constructor, None)),
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
            self.events
                .get(&event_name_raw)
                .unwrap()
                .fields
                .get(event_count)
                .unwrap()
                .name
                .clone(),
            bx!(self.parse_expression(&trim(&buffer), constructor, None)),
        ));

        Statement::Emit(event_name_raw, args)
    }

    /// Parses a solidity assignment statement
    ///
    /// `line` the solidity representation of the assign statement
    /// `constructor` if the statement is inside a constructor
    ///
    /// Return the statement in form of `Statement::Assign`
    fn parse_assign(&mut self, line: &str, constructor: bool) -> Statement {
        let left_raw = capture_regex(&REGEX_ASSIGN, line, "left").unwrap();
        let operation_raw =
            capture_regex(&REGEX_ASSIGN, line, "operation").unwrap_or_else(|| String::from("="));
        let right_raw = capture_regex(&REGEX_ASSIGN, line, "right").unwrap();

        let left = self.parse_expression(&left_raw, constructor, None);
        let operation = *OPERATIONS.get(&operation_raw).unwrap();
        let right = self.parse_expression(&right_raw, constructor, None);

        if REGEX_BINARY_PREFIX.is_match(&right_raw) {
            let value_raw = capture_regex(&REGEX_BINARY_PREFIX, &right_raw, "value").unwrap();
            let value = self.parse_expression(&value_raw, constructor, None);
            let assign = Statement::Assign(left, value, operation);
            let arithmetic =
                self.parse_binary_operation(&right_raw, constructor, &REGEX_BINARY_PREFIX, None);
            return Statement::Group(vec![arithmetic, assign])
        } else if REGEX_BINARY_SUFFIX.is_match(&right_raw) {
            let value_raw = capture_regex(&REGEX_BINARY_SUFFIX, &right_raw, "value").unwrap();
            let value = self.parse_expression(&value_raw, constructor, None);
            let assign = Statement::Assign(left, value, operation);
            let arithmetic =
                self.parse_binary_operation(&right_raw, constructor, &REGEX_BINARY_SUFFIX, None);
            return Statement::Group(vec![assign, arithmetic])
        }

        if let Expression::Mapping(name, indices, None) = left {
            let converted_operation = match operation {
                Operation::AddAssign => Operation::Add,
                Operation::MulAssign => Operation::Mul,
                Operation::DivAssign => Operation::Div,
                Operation::SubtractAssign => Operation::Subtract,
                _ => operation,
            };
            let right_mapping = match converted_operation {
                Operation::Add | Operation::Mul | Operation::Div | Operation::Subtract => {
                    Some(bx!(Expression::Arithmetic(
                        bx!(Expression::Mapping(name.clone(), indices.clone(), None,)),
                        bx!(right),
                        converted_operation,
                    )))
                }
                _ => Some(bx!(right)),
            };
            Statement::FunctionCall(Expression::Mapping(name, indices, right_mapping))
        } else {
            Statement::Assign(left, right, operation)
        }
    }

    /// Parses a solidity ternary operation
    ///
    /// `line` the solidity representation of the ternary operation
    /// `constructor` if the statement is inside a constructor
    /// `stack` the current statement stack
    /// `iterator` the iterator of the statements of currently parsed function
    ///
    /// Return the statement in form of `Statement::Ternary`
    fn parse_ternary_statement(
        &mut self,
        line: &str,
        constructor: bool,
        stack: &mut VecDeque<Block>,
        iterator: &mut Iter<Statement>,
    ) -> Statement {
        let condition_raw = capture_regex(&REGEX_TERNARY, line, "condition").unwrap();
        let if_true_raw = capture_regex(&REGEX_TERNARY, line, "if_true").unwrap();
        let if_false_raw = capture_regex(&REGEX_TERNARY, line, "if_false").unwrap();

        let condition = self.parse_condition(&condition_raw, constructor, false, None);
        let if_true = self.parse_statement(&if_true_raw, constructor, stack, iterator);
        let if_false = self.parse_statement(&if_false_raw, constructor, stack, iterator);
        Statement::Ternary(condition, bx!(if_true), bx!(if_false))
    }

    /// Parses a solidity expression and returns it as `Expression`
    ///
    /// `raw` the raw representation of the expression
    /// `constructor` if the expression is inside a constructor
    /// `enclosed_expressions` the previously parsed enclosed expressions
    fn parse_expression(
        &mut self,
        raw: &String,
        constructor: bool,
        enclosed_expressions: Option<HashMap<String, Expression>>,
    ) -> Expression {
        if is_literal(raw) {
            return Expression::Literal(raw.clone())
        } else if let Some(expression) = SPECIFIC_EXPRESSION.get(raw) {
            if expression == &Expression::ZeroAddressInto {
                self.imports
                    .insert(String::from("use openbrush::traits::ZERO_ADDRESS;"));
            } else if expression == &Expression::EnvCaller(None) {
                return Expression::EnvCaller(Some(selector!(constructor)))
            } else if expression == &Expression::TransferredValue(None) {
                return Expression::TransferredValue(Some(selector!(constructor)))
            }

            return expression.clone()
        } else if let Some(new_type) = TYPES.get(raw.as_str()) {
            return Expression::Literal(new_type.0.to_owned())
        }

        if let Some(expression) = enclosed_expressions.clone().unwrap_or_default().get(raw) {
            return Expression::Enclosed(bx!(expression.clone()))
        }

        let extracted = self.extract_parentheses(raw, constructor);
        if extracted.1 > 0 {
            return self.parse_expression(&extracted.0, constructor, Some(extracted.2))
        }

        let regex_hex_string = Regex::new(r#"(?x)^\s*hex"(?P<value>.+?)"\s*$"#).unwrap();
        let regex_hex = Regex::new(r#"(?x)^\s*(?P<value>0x[0-9A-Fa-f]*?)\s*$"#).unwrap();
        if regex_hex_string.is_match(raw) || regex_hex.is_match(raw) {
            let value = capture_regex(&regex_hex_string, raw, "value")
                .unwrap_or_else(|| capture_regex(&regex_hex, raw, "value").unwrap());
            return self.parse_expression(
                &format!("hex(\"{value}\")"),
                constructor,
                enclosed_expressions,
            )
        }

        let regex_struct_init = Regex::new(
            r#"(?x)^\s*
            (?P<struct_name>.+)\s*\(\{\s*
            (?P<args>.+)\s*\}\)\s*$"#,
        )
        .unwrap();
        if regex_struct_init.is_match(raw) {
            let struct_name_raw = capture_regex(&regex_struct_init, raw, "struct_name").unwrap();
            let mut args_string = trim(&capture_regex(&regex_struct_init, raw, "args").unwrap());
            args_string = args_string.replace(": ", ":");
            args_string = args_string.replace(", ", ",");

            if args_string.contains(':') {
                // named params
                let args_raw = split(&args_string, ",", None);
                let regex_named_param = Regex::new(
                    r#"(?x)^\s*
                    (?P<param_name>.+)\s*:\s*
                    (?P<value>.+)\s*$"#,
                )
                .unwrap();
                let args = args_raw
                    .iter()
                    .map(|raw| {
                        let param_name =
                            capture_regex(&regex_named_param, raw, "param_name").unwrap();
                        let value_raw = capture_regex(&regex_named_param, raw, "value").unwrap();
                        let value = self.parse_expression(
                            &value_raw,
                            constructor,
                            enclosed_expressions.clone(),
                        );
                        Expression::StructArg(param_name, bx!(value))
                    })
                    .collect::<Vec<Expression>>();
                return Expression::StructInit(struct_name_raw, args)
            } else {
                let args_raw = split(&args_string, ",", None);
                let mut args = Vec::default();
                for (i, raw) in args_raw.iter().enumerate() {
                    let value =
                        self.parse_expression(raw, constructor, enclosed_expressions.clone());
                    let param_name = self
                        .structs
                        .get(&struct_name_raw)
                        .unwrap_or_else(|| panic!("Struct {struct_name_raw} not defined"))
                        .fields[i]
                        .name
                        .clone();

                    args.push(Expression::StructArg(param_name, bx!(value)));
                }
                return Expression::StructInit(struct_name_raw, args)
            }
        }

        if REGEX_FUNCTION_CALL.is_match(raw) {
            return self.parse_function_call(raw, constructor, enclosed_expressions)
        }

        let regex_new_array = Regex::new(
            r#"(?x)^\s*new\s+(?P<array_type>.+?)\s*
            \[\s*\]\s*\((?P<array_size>.+?)\s*\)\s*$"#,
        )
        .unwrap();
        if regex_new_array.is_match(raw) {
            let array_type_raw = capture_regex(&regex_new_array, raw, "array_type").unwrap();
            let array_size_raw = capture_regex(&regex_new_array, raw, "array_size").unwrap();

            let array_type = self.convert_variable_type(array_type_raw);
            let array_size =
                self.parse_expression(&array_size_raw, constructor, enclosed_expressions);
            return Expression::NewArray(array_type, bx!(array_size))
        }

        let regex_type = Regex::new(
            r#"(?x)
            ^\s*type\s*\(\s*(?P<selector>.+?)
            \)\.(?P<member>.+?)\s*$"#,
        )
        .unwrap();
        if regex_type.is_match(raw) {
            let selector_raw = capture_regex(&regex_type, raw, "selector").unwrap();
            let member_raw = capture_regex(&regex_type, raw, "member").unwrap();

            let selector =
                self.parse_expression(&selector_raw, constructor, enclosed_expressions.clone());
            let member = self.parse_expression(&member_raw, constructor, enclosed_expressions);

            return Expression::WithSelector(bx!(selector), bx!(member))
        }

        let regex_arithmetic = Regex::new(
            r#"(?x)
            ^\s*(?P<left>[^+\-]*?)
            \s*(?P<operation>[/+\-*%][*]*)
            \s*(?P<right>[^+\-=].*)
            \s*$"#,
        )
        .unwrap();
        if regex_arithmetic.is_match(raw) {
            let left_raw = capture_regex(&regex_arithmetic, raw, "left").unwrap();
            let right_raw = capture_regex(&regex_arithmetic, raw, "right").unwrap();
            let operation_raw = capture_regex(&regex_arithmetic, raw, "operation").unwrap();
            let left = self.parse_expression(&left_raw, constructor, enclosed_expressions.clone());
            let right = self.parse_expression(&right_raw, constructor, enclosed_expressions);
            let operation = *OPERATIONS.get(&operation_raw).unwrap();

            return Expression::Arithmetic(bx!(left), bx!(right), operation)
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
            let left = self.parse_expression(&left_raw, constructor, enclosed_expressions.clone());
            let operation = *OPERATIONS.get(&operation_raw).unwrap();
            let right = self.parse_expression(&right_raw, constructor, enclosed_expressions);

            return Expression::Logical(bx!(left), operation, bx!(right))
        }

        if REGEX_TERNARY.is_match(raw) {
            return self.parse_ternary(raw, constructor, enclosed_expressions)
        }

        if REGEX_BOOLEAN.is_match(raw) {
            let condition = self.parse_condition(raw, constructor, false, enclosed_expressions);
            return Expression::Condition(bx!(condition))
        }

        let regex_with_selector =
            Regex::new(r#"(?x)^\s*(?P<left>.+?)\.(?P<right>.+?);*\s*$"#).unwrap();
        if regex_with_selector.is_match(raw) {
            let left_raw = capture_regex(&regex_with_selector, raw, "left").unwrap();
            let right_raw = capture_regex(&regex_with_selector, raw, "right").unwrap();
            let left = self.parse_expression(&left_raw, constructor, enclosed_expressions.clone());
            let right = self.parse_expression(&right_raw, constructor, enclosed_expressions);

            match &right {
                Expression::FunctionCall(function_name, expressions, _, external) => {
                    return Expression::WithSelector(
                        bx!(left),
                        bx!(Expression::FunctionCall(
                            function_name.clone(),
                            expressions.clone(),
                            None,
                            *external,
                        )),
                    )
                }
                Expression::Member(member_name, _) => {
                    return Expression::WithSelector(
                        bx!(left),
                        bx!(Expression::Member(member_name.clone(), None)),
                    )
                }
                _ => {}
            };

            return Expression::WithSelector(bx!(left), bx!(right))
        }

        let regex_mapping = Regex::new(
            r#"(?x)
            ^\s*(?P<mapping_name>.+?)\s*
            (?P<index>(\[\s*.+\s*\]))+?
            \s*$"#,
        )
        .unwrap();
        if regex_mapping.is_match(raw) {
            let mapping_raw = capture_regex(&regex_mapping, raw, "mapping_name").unwrap();
            let mapping =
                self.parse_expression(&mapping_raw, constructor, enclosed_expressions.clone());
            let indices_raw = capture_regex(&regex_mapping, raw, "index").unwrap();
            let mut indices = Vec::<Expression>::new();
            let mut buffer = String::new();
            let mut open_braces = 0;
            let mut close_braces = 0;

            for ch in indices_raw.chars() {
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
                            indices.push(self.parse_expression(
                                &buffer,
                                constructor,
                                enclosed_expressions.clone(),
                            ));
                            buffer.clear();
                        } else {
                            buffer.push(ch)
                        }
                    }
                    _ => buffer.push(ch),
                }
            }

            return Expression::Mapping(bx!(mapping), indices, None)
        }

        if REGEX_BINARY_SUFFIX.is_match(raw) {
            let statement = self.parse_binary_operation(
                raw,
                constructor,
                &REGEX_BINARY_SUFFIX,
                enclosed_expressions.clone(),
            );
            if let Statement::Assign(member, _, _) = statement {
                return member
            }
        }

        if REGEX_BINARY_PREFIX.is_match(raw) {
            let statement = self.parse_binary_operation(
                raw,
                constructor,
                &REGEX_BINARY_PREFIX,
                enclosed_expressions,
            );
            if let Statement::Assign(member, _, _) = statement {
                return member
            }
        }

        if let Some(contract_field) = self.storage.get(raw) {
            if contract_field.constant {
                return Expression::Constant(contract_field.name.clone())
            }
        }

        let selector = self.get_selector(constructor, raw);

        Expression::Member(raw.clone(), selector)
    }

    /// Extracts enclosed expression from expression
    ///
    /// The function replaces all enclosed expressions with an expression identifier
    /// and maps the enclosed expressions to a parsed expression
    /// Then we can parse all the expressions in the correct order
    ///
    /// ```
    /// let extracted = self.extract_parentheses("((1 + 2) + 3) + 4", false);
    /// assert_eq!(extracted.0, String::from("___0___ + 4"));
    /// assert_eq!(extracted.1, 1);
    /// assert_eq!(extracted.2.get("___0___"), Some(
    /// Expression::Arithmetic(Expression::Enclosed(Expression::Arithmetic(
    /// Expression::Arithmetic(1, 2, Operation::Add)), 3, Operation::Add), 4, Operation::Add)
    /// ));
    /// ```
    ///
    /// `raw` the raw representation of the expression
    /// `constructor` if the expression is inside a constructor
    ///
    /// Return `0` the statement without brackets (enclosed expressions are substituted with identifiers)
    /// Return `1` the number of returned enclosed expressions
    /// Return `2` the map of expression_identifier => Expression
    fn extract_parentheses(
        &mut self,
        raw: &str,
        constructor: bool,
    ) -> (String, i32, HashMap<String, Expression>) {
        let mut buffer_out = String::new();
        let mut buffer_tmp = String::new();
        let mut open_parentheses = 0;
        let mut close_parentheses = 0;
        let mut map = HashMap::<String, Expression>::new();
        let mut args = 0;
        let mut group_possible = true;
        let mut group = false;

        for ch in raw.chars() {
            match ch {
                PARENTHESIS_CLOSE => {
                    close_parentheses += 1;
                }
                PARENTHESIS_OPEN => {
                    open_parentheses += 1;
                }
                _ => {}
            }
            match ch {
                ASTERISK | SLASH | EQUALS | PLUS | MINUS | AMPERSAND | PIPE | PERCENT
                | EXCLAMAITON
                    if open_parentheses == close_parentheses =>
                {
                    group_possible = true;
                    buffer_out.push(ch);
                }
                SPACE if group_possible => {
                    buffer_out.push(ch);
                }
                PARENTHESIS_OPEN if group_possible => {
                    group = true;
                    group_possible = false;
                }
                PARENTHESIS_CLOSE if group && open_parentheses == close_parentheses => {
                    group = false;
                    let arg_name = format!("___{args}___");
                    let expression = self.parse_expression(&buffer_tmp, constructor, None);
                    map.insert(arg_name.clone(), expression);
                    buffer_out.push_str(&arg_name);
                    buffer_tmp.clear();
                    args += 1;
                }
                _ => {
                    if group_possible {
                        group_possible = false;
                    }
                    if group {
                        buffer_tmp.push(ch);
                    } else {
                        buffer_out.push(ch);
                    }
                }
            }
        }

        (buffer_out, args, map)
    }

    /// Parses a solidity ternary expression
    ///
    /// `raw` the raw representation of the expression
    /// `constructor` if the expression is inside a constructor
    /// `enclosed_expressions` the previously parsed enclosed expressions
    ///
    /// Returns the expression as `Expression::Ternary`
    fn parse_ternary(
        &mut self,
        raw: &str,
        constructor: bool,
        enclosed_expressions: Option<HashMap<String, Expression>>,
    ) -> Expression {
        let condition_raw = capture_regex(&REGEX_TERNARY, raw, "condition").unwrap();
        let if_true_raw = capture_regex(&REGEX_TERNARY, raw, "if_true").unwrap();
        let if_false_raw = capture_regex(&REGEX_TERNARY, raw, "if_false").unwrap();

        let condition = self.parse_condition(
            &condition_raw,
            constructor,
            false,
            enclosed_expressions.clone(),
        );
        let if_true =
            self.parse_expression(&if_true_raw, constructor, enclosed_expressions.clone());
        let if_false = self.parse_expression(&if_false_raw, constructor, enclosed_expressions);
        Expression::Ternary(bx!(condition), bx!(if_true), bx!(if_false))
    }

    /// Parses a solidity binary operation
    ///
    /// `line` the solidity representation of the binary operation statement
    /// `constructor` if the statement is inside a constructor
    /// `regex` the regex we use (suffix or prefix)
    /// `enclosed_expressions` the previously parsed enclosed expressions
    ///
    /// Return the statement in form of `Statement::Assign`
    fn parse_binary_operation(
        &mut self,
        line: &str,
        constructor: bool,
        regex: &Regex,
        enclosed_expressions: Option<HashMap<String, Expression>>,
    ) -> Statement {
        let member_raw = capture_regex(regex, line, "value").unwrap();
        let operation_raw =
            capture_regex(regex, line, "operation").unwrap_or_else(|| String::from("="));

        let member = self.parse_expression(&member_raw, constructor, enclosed_expressions);
        let original_operation = *OPERATIONS.get(&operation_raw).unwrap();
        let operation = match original_operation {
            Operation::AddOne => Operation::AddAssign,
            Operation::SubtractOne => Operation::SubtractAssign,
            _ => original_operation,
        };

        Statement::Assign(member, Expression::Literal(String::from("1")), operation)
    }

    /// Parses a solidity function call
    ///
    /// `line` the solidity representation of the function call
    /// `constructor` if the expression is inside a constructor
    /// `enclosed_expressions` the previously parsed enclosed expressions
    ///
    /// Return the expression in form of `Expression::FunctionCall`
    fn parse_function_call(
        &mut self,
        line: &str,
        constructor: bool,
        enclosed_expressions: Option<HashMap<String, Expression>>,
    ) -> Expression {
        let function_name_raw = capture_regex(&REGEX_FUNCTION_CALL, line, "function_name").unwrap();
        let args_raw = capture_regex(&REGEX_FUNCTION_CALL, line, "args").unwrap();
        let mut args = Vec::<Expression>::new();
        let mut buffer = String::new();
        let mut open_parentheses = 0;
        let mut close_parenthesis = 0;

        if TYPES.contains_key(&function_name_raw.as_str()) {
            let the_type = TYPES.get(&function_name_raw.as_str()).unwrap();
            if let Some(unique_cast) = the_type.1 {
                return Expression::Cast(
                    true,
                    unique_cast.to_string(),
                    bx!(self.parse_expression(&args_raw, constructor, enclosed_expressions)),
                )
            } else {
                return Expression::Cast(
                    false,
                    the_type.0.to_string(),
                    bx!(self.parse_expression(&args_raw, constructor, enclosed_expressions)),
                )
            }
        }

        for ch in args_raw.chars() {
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
                        args.push(self.parse_expression(
                            &trim(&buffer),
                            constructor,
                            enclosed_expressions.clone(),
                        ));
                        buffer.clear();
                    } else {
                        buffer.push(ch)
                    }
                }
                _ => buffer.push(ch),
            }
        }

        if !trim(&buffer).is_empty() {
            args.push(self.parse_expression(&buffer, constructor, enclosed_expressions));
        }

        let selector = if self.functions.get(&function_name_raw).is_some() {
            Some(selector!(constructor))
        } else {
            None
        };

        return Expression::FunctionCall(
            function_name_raw.clone(),
            args,
            selector,
            *self.functions.get(&function_name_raw).unwrap_or(&true),
        )
    }

    /// Converts solidity variable type to ink! variable type (eg. address -> AccountId, uint -> u128, ...)
    ///
    /// `arg_type` solidity argument type
    /// `imports` the set of imports of the contract
    ///
    /// return the converted type
    fn convert_variable_type(&mut self, arg_type: String) -> String {
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
            self.imports
                .insert(String::from("use openbrush::storage::Mapping;\n"));
            let mut from_raw = capture_regex(&regex_mapping, &arg_type, "type_from").unwrap();
            let mut to_raw = capture_regex(&regex_mapping, &arg_type, "type_to").unwrap();

            let mut from_vec = vec![self.convert_variable_type(from_raw)];
            while regex_mapping.is_match(&to_raw) {
                from_raw = capture_regex(&regex_mapping, &to_raw, "type_from").unwrap();
                to_raw = capture_regex(&regex_mapping, &to_raw, "type_to").unwrap();
                from_vec.push(self.convert_variable_type(from_raw));
            }

            let to = self.convert_variable_type(to_raw);

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
                    self.imports.insert(format!("use {import};\n"));
                }
                the_type.0.to_string()
            }
            _ => no_array_arg_type.to_owned(),
        };
        return if is_vec {
            self.imports
                .insert(String::from("use ink_prelude::vec::Vec;\n"));
            format!("Vec<{}>", output_type)
        } else {
            output_type
        }
    }

    /// returns the selctor of the field (function or a variable)
    ///
    /// `constructor` if we access this field from a constructor
    /// `field_name` the name of the field
    ///
    /// returns Some if the field needs a selector, None otherwise
    fn get_selector(&self, constructor: bool, field_name: &String) -> Option<String> {
        if self.storage.contains_key(field_name) {
            Some(selector!(constructor))
        } else {
            None
        }
    }
}

/// Captures a regex group and returns it
///
/// `regex` the regex to use
/// `line` the string on which we will use the regex
/// `capture_name` the name of the group to capture
#[inline]
fn capture_regex(regex: &Regex, line: &str, capture_name: &str) -> Option<String> {
    regex.captures(line).and_then(|cap| {
        cap.name(capture_name)
            .map(|value| value.as_str().to_string())
    })
}

/// Returns true if expression passed is a literal
///
/// `expression` the expression to check
fn is_literal(expression: &String) -> bool {
    let string_regex = Regex::new(r#"^\s*".*"\s*$"#).unwrap();
    let char_regex = Regex::new(r#"^\s*'.*'\s*$"#).unwrap();
    expression.parse::<i32>().is_ok()
        || string_regex.is_match(expression)
        || char_regex.is_match(expression)
        || expression == "true"
        || expression == "false"
}

/// Parses attributes of a function like payable, external, view
///
/// `attributes` the raw representation of the attributes of the function
///
/// returns 0. external 1. view 2. payable
fn parse_function_attributes(attributes: &str) -> (bool, bool, bool) {
    let external = attributes.contains("external") || attributes.contains("public");
    let view = attributes.contains("view") || attributes.contains("pure");
    let payable = attributes.contains("payable");

    (external, view, payable)
}

/// Parses the modifiers of a function
///
/// `attributes` the raw representation of the attributes of the function
///
/// returns the modifiers of the function in a vec of expressions
fn parse_modifiers(attributes: &str) -> Vec<Expression> {
    let mut adjusted = attributes.to_owned();
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
            .iter()
            .map(|raw_modifier| Expression::Modifier(raw_modifier.to_owned()))
            .collect()
    }
}

/// Converts size of solidity int argument to match rust int size
/// we use u128 or i128 for integers bigger than or equal to 128 bits
/// we use u64 or i64 for integers bigger than or equal to 64 bits
/// we use u32 or i32 for integers bigger than or equal to 32 bits
/// we use u32 or i16 for integers bigger than or equal to 16 bits
/// we use u8 or i8 for the smallest integers
///
/// Returns the updated int argument or the original argument if the argument provided is not an integer
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

/// Skips the characters in a char array until one of characters specified is found
///
/// `chars` the char array where to skip the characters
/// `until` the vector of characters which we want to find, the function ends when it matches one of the characters
///
/// Returns the chars skipped as a String
fn read_until(chars: &mut Chars, until: Vec<char>) -> String {
    let mut buffer = String::new();
    for ch in chars.by_ref() {
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
