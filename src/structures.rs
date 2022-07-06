use std::collections::HashSet;

#[derive(Debug, PartialEq, Default)]
pub enum ContractType {
    #[default]
    INTERFACE,
    CONTRACT,
}

pub struct Contract {
    pub name: String,
    pub fields: Vec<ContractField>,
    pub constructor: Function,
    pub events: Vec<Event>,
    pub enums: Vec<Enum>,
    pub structs: Vec<Struct>,
    pub functions: Vec<Function>,
    pub imports: HashSet<String>,
    pub comments: Vec<String>,
}

pub struct Interface {
    pub name: String,
    pub events: Vec<Event>,
    pub enums: Vec<Enum>,
    pub structs: Vec<Struct>,
    pub function_headers: Vec<FunctionHeader>,
    pub imports: HashSet<String>,
    pub comments: Vec<String>,
}

pub struct ContractField {
    pub field_type: String,
    pub name: String,
}

pub struct Event {
    pub name: String,
    pub fields: Vec<EventField>,
    pub comments: Vec<String>,
}

pub struct EventField {
    pub indexed: bool,
    pub field_type: String,
    pub name: String,
}

pub struct Enum {
    pub name: String,
    pub values: Vec<String>,
    pub comments: Vec<String>,
}

pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
    pub comments: Vec<String>,
}

#[derive(Default, Clone)]
pub struct StructField {
    pub name: String,
    pub field_type: String,
}

#[derive(Default, Clone)]
pub struct Function {
    pub header: FunctionHeader,
    pub body: Vec<Statement>,
}

#[derive(Default, Clone)]
pub struct FunctionHeader {
    pub name: String,
    pub params: Vec<FunctionParam>,
    pub external: bool,
    pub view: bool,
    pub payable: bool,
    pub return_params: Vec<FunctionParam>,
    pub comments: Vec<String>,
}

#[derive(Clone)]
pub struct FunctionParam {
    pub name: String,
    pub param_type: String,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Raw(String),
    Comment(String),
    Inline(String),
    Block(Vec<Statement>)
}

#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<String>,
    pub constructor: bool,
}

#[derive(Debug)]
pub struct Condition {
    pub left: String,
    pub operation: Operation,
    pub right: Option<String>,
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Not,
    True,
    GreaterThanEqual,
    GreaterThan,
    LessThanEqual,
    LessThan,
    Equal,
    NotEqual,
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        return match self {
            Operation::Not => String::from("!"),
            Operation::True => String::from(""),
            Operation::GreaterThanEqual => String::from(">="),
            Operation::GreaterThan => String::from(">"),
            Operation::LessThanEqual => String::from("<="),
            Operation::LessThan => String::from("<"),
            Operation::Equal => String::from("=="),
            Operation::NotEqual => String::from("!="),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    FunctionCall(FunctionCall),
    Custom(String),
}

pub enum Block {
    Unchecked,
    If
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        return match self {
            Expression::Custom(exp) => exp.to_owned(),
            Expression::FunctionCall(function_call) => {
                format!(
                    "{}.{}({})?",
                    if function_call.constructor {
                        "instance"
                    } else {
                        "self"
                    },
                    function_call.name,
                    function_call.args.join(", ")
                )
            }
        }
    }
}
