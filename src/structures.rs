use std::collections::HashSet;

use convert_case::{
    Case::Snake,
    Casing,
};

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
    Comment(String),
    Declaration(String, String, Option<Expression>),
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
    FunctionCall(String, Vec<Expression>, String),
    IsZero(Box<Expression>),
    Literal(String),
    Member(String, Option<String>),
    Mapping(String, Vec<Expression>, Option<String>),
    ZeroAddressInto,
}

pub enum Block {
    Unchecked,
    If,
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        return match self {
            Expression::EnvCaller(selector) => {
                format!("{}.env().caller()", selector.clone().unwrap())
            }
            Expression::FunctionCall(function_name_raw, args, selector) => {
                format!(
                    "{}.{}({})?",
                    selector,
                    function_name_raw.to_case(Snake),
                    args.iter()
                        .map(|expression| expression.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Expression::IsZero(expression) => {
                format!("{}.is_zero()", expression.to_string())
            }
            Expression::Literal(content) => content.to_owned(),
            Expression::Member(expression_raw, selector_raw) => {
                let expression = expression_raw.to_case(Snake);
                if let Some(selector) = selector_raw {
                    format!("{selector}.{expression}")
                } else {
                    expression
                }
            }
            Expression::Mapping(name_raw, indices_raw, selector_raw) => {
                let indices = if indices_raw.len() > 1 {
                    format!(
                        "({})",
                        indices_raw
                            .iter()
                            .map(|expr| expr.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                } else {
                    indices_raw.get(0).unwrap().to_string()
                };
                let name = name_raw.to_case(Snake);
                if let Some(selector) = selector_raw {
                    format!("{selector}.{name}.get(&{indices}).unwrap()")
                } else {
                    format!("{name}.get(&{indices}).unwrap()")
                }
            }
            Expression::ZeroAddressInto => String::from("ZERO_ADDRESS.into()"),
        }
    }
}
