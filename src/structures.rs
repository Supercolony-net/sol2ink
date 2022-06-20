use std::collections::HashSet;

#[derive(Debug, PartialEq, Default)]
pub enum ContractType {
    #[default]
    INTERFACE,
    CONTRACT,
}

/// `contract_name` the name of the contract
/// `next_line` n-th line where we found contract definition
/// `contract_type` type of parsed contract (contract/interface)
#[derive(Debug, Default)]
pub struct ContractDefinition {
    pub contract_name: String,
    pub next_line: usize,
    pub contract_type: ContractType,
}

pub struct Interface {
    pub imports: HashSet<String>,
    pub functions: Vec<String>,
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
}

pub struct ContractField {
    pub field_type: String,
    pub name: String,
}

pub struct Event {
    pub name: String,
    pub fields: Vec<EventField>,
}

pub struct EventField {
    pub indexed: bool,
    pub field_type: String,
    pub name: String,
}

pub struct Enum {
    pub name: String,
    pub values: Vec<String>,
}

pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Default, Clone)]
pub struct StructField {
    pub name: String,
    pub field_type: String,
}

#[derive(Default, Clone)]
pub struct Function {
    pub header: FunctionHeader,
    pub cosntructor: bool,
    pub body: Vec<Statement>,
}

#[derive(Default, Clone)]
pub struct FunctionHeader {
    pub name: String,
    pub params: Vec<FunctionParam>,
    pub external: bool,
    pub view: bool,
    pub payable: bool,
    pub return_params: Vec<String>,
}

#[derive(Clone)]
pub struct FunctionParam {
    pub name: String,
    pub param_type: String,
}

#[derive(Clone, Debug)]
pub struct Statement {
    pub content: String,
}
