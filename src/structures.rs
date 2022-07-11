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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Assign(Expression, Expression),
    Comment(String),
    Declaration(String, String, Option<Expression>),
    FunctionCall(Expression),
    If(Condition, Vec<Statement>),
    IfEnd,
    Raw(String),
    Require(Condition, String),
    Return(Expression),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Condition {
    pub left: Expression,
    pub operation: Operation,
    pub right: Option<Expression>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

impl Operation {
    pub fn negate(&self) -> Operation {
        match self {
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    EnvCaller(Option<String>),
    FunctionCall(String, Vec<Expression>, String, bool),
    IsZero(Box<Expression>),
    Literal(String),
    Member(String, Option<String>),
    Mapping(
        String,
        Vec<Expression>,
        Option<String>,
        Option<Box<Expression>>,
    ),
    Subtraction(Box<Expression>, Box<Expression>),
    ZeroAddressInto,
}

pub enum Block {
    Unchecked,
    If,
}
