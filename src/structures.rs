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
    pub constructor: Constructor,
    pub events: Vec<Event>,
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

pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
}

pub struct StructField {
    pub name: String,
    pub field_type: String,
}

pub struct Function {
    pub name: String,
    pub params: Vec<FunctionParam>,
    pub external: bool,
    pub view: bool,
    pub payable: bool,
    pub return_params: Vec<String>,
    pub body: Vec<Statement>,
}

pub struct Constructor {
    pub params: Vec<FunctionParam>,
    pub body: Vec<Statement>,
}

pub struct FunctionParam {
    pub name: String,
    pub param_type: String,
}

pub struct Statement {
    pub content: String,
}
