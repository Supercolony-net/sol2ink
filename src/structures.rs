use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Default)]
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
    pub contract_doc: Vec<String>,
    pub modifiers: Vec<Modifier>,
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

#[derive(Clone)]
pub struct ContractField {
    pub field_type: String,
    pub name: String,
    pub comments: Vec<String>,
    pub initial_value: Option<Expression>,
    pub constant: bool,
}

pub struct Modifier {
    pub header: FunctionHeader,
    pub statements: Vec<Statement>,
    pub comments: Vec<String>,
}

#[derive(Clone)]
pub struct Event {
    pub name: String,
    pub fields: Vec<EventField>,
    pub comments: Vec<String>,
}

#[derive(Clone)]
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

#[derive(Clone)]
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
    pub modifiers: Vec<Expression>,
}

#[derive(Clone, Debug)]
pub struct FunctionParam {
    pub name: String,
    pub param_type: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    AssemblyEnd,
    Assign(Expression, Expression, Operation),
    Catch(Vec<Statement>),
    CatchEnd,
    Comment(String),
    Declaration(String, String, Option<Expression>),
    Loop(
        Option<Box<Statement>>,
        Expression,
        Option<Box<Statement>>,
        Vec<Statement>,
    ),
    Else(Vec<Statement>),
    ElseIf(Condition, Vec<Statement>),
    Emit(String, Vec<Expression>),
    FunctionCall(Expression),
    Group(Vec<Statement>),
    If(Condition, Vec<Statement>),
    IfEnd,
    ModifierBody,
    Raw(String),
    Require(Condition, String),
    Return(Expression),
    Ternary(Condition, Box<Statement>, Box<Statement>),
    Try(Vec<Statement>),
    TryEnd,
    While(
        Option<Box<Statement>>,
        Expression,
        Option<Box<Statement>>,
        Vec<Statement>,
    ),
    WhileEnd,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Condition {
    pub left: Expression,
    pub operation: Operation,
    pub right: Option<Expression>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    Add,
    AddAssign,
    AddOne,
    AndAssign,
    Assign,
    BitwiseAnd,
    BitwiseOr,
    Div,
    DivAssign,
    Equal,
    GreaterThanEqual,
    GreaterThan,
    LessThanEqual,
    LessThan,
    LogicalAnd,
    LogicalOr,
    Modulo,
    Mul,
    MulAssign,
    Not,
    NotEqual,
    OrAssign,
    Pow,
    Subtract,
    SubtractOne,
    SubtractAssign,
    ShiftLeft,
    ShiftRight,
    True,
    Xor,
}

impl Operation {
    pub fn negate(&self) -> Operation {
        match self {
            Operation::BitwiseAnd => Operation::BitwiseOr,
            Operation::BitwiseOr => Operation::BitwiseAnd,
            Operation::Equal => Operation::NotEqual,
            Operation::GreaterThanEqual => Operation::LessThan,
            Operation::GreaterThan => Operation::LessThanEqual,
            Operation::LessThanEqual => Operation::GreaterThan,
            Operation::LessThan => Operation::GreaterThanEqual,
            // TODO a and b = neg(a) or neg (b)
            Operation::LogicalAnd => Operation::LogicalOr,
            Operation::LogicalOr => Operation::LogicalAnd,
            Operation::Not => Operation::True,
            Operation::NotEqual => Operation::Equal,
            _ => Operation::Not,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Arithmetic(Box<Expression>, Box<Expression>, Operation),
    Cast(bool, String, Box<Expression>),
    Condition(Box<Condition>),
    Constant(String),
    Enclosed(Box<Expression>),
    EnvCaller(Option<String>),
    FunctionCall(String, Vec<Expression>, Option<String>, bool),
    IsZero(Box<Expression>),
    Literal(String),
    Logical(Box<Expression>, Operation, Box<Expression>),
    Member(String, Option<String>),
    Mapping(Box<Expression>, Vec<Expression>, Option<Box<Expression>>),
    Modifier(String),
    NewArray(String, Box<Expression>),
    StructArg(String, Box<Expression>),
    StructInit(String, Vec<Expression>),
    Ternary(Box<Condition>, Box<Expression>, Box<Expression>),
    TransferredValue(Option<String>),
    WithSelector(Box<Expression>, Box<Expression>),
    ZeroAddressInto,
}

pub enum Block {
    Assembly,
    Catch,
    Else,
    ElseIf,
    If,
    Try,
    Unchecked,
    While,
}
