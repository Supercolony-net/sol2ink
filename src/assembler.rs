use crate::{
    formatter::append_and_tab,
    structures::*,
};
use convert_case::{
    Case,
    Casing,
};

/// This function will assemble ink! contract from the parsed contract struct and save it to a file
pub fn assemble_contract(contract: Contract) -> Vec<String> {
    let mut output_vec = Vec::<String>::new();
    output_vec.push(String::from(
        "#![cfg_attr(not(feature = \"std\"), no_std)]\n",
    ));
    output_vec.push(String::from("#![feature(min_specialization)]\n\n"));
    output_vec.push(String::from("#[brush::contract]\n"));
    output_vec.push(format!(
        "pub mod {} {{\n",
        contract.name.to_case(Case::Snake)
    ));

    // imports
    append_and_tab(&mut output_vec, Vec::from_iter(contract.imports));
    output_vec.push(String::from("\n"));
    // events
    append_and_tab(&mut output_vec, assemble_events(contract.events));
    // structs
    append_and_tab(&mut output_vec, assemble_structs(contract.structs));
    // fields
    append_and_tab(
        &mut output_vec,
        assemble_storage(contract.name.clone(), contract.fields),
    );
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
            output_vec.push(format!(
                "\t{}: {},\n",
                event_field.name, event_field.field_type
            ));
        }
        output_vec.push(String::from("}\n\n"));
    }

    output_vec
}

/// This function will assemble ink! storage from the parsed contract
fn assemble_storage(contract_name: String, fields: Vec<ContractField>) -> Vec<String> {
    let mut output_vec = Vec::<String>::new();

    output_vec.push(String::from("#[ink(storage)]\n"));
    output_vec.push(String::from("#[derive(Default, SpreadAllocate)]\n"));

    output_vec.push(format!("pub struct {} {{\n", contract_name));
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
            output_vec.push(format!(
                "\tpub {}: {},\n",
                struct_field.name, struct_field.field_type
            ));
        }
        output_vec.push(String::from("}\n\n"));
    }

    output_vec
}

/// This function will assemble the constructor of the ink! contract from the parsed contract
fn assemble_constructor(constructor: Function) -> Vec<String> {
    let mut output_vec = Vec::<String>::new();
    // we do this so we dont put tab between each string that we insert (function params)
    let mut header = String::new();

    output_vec.push(String::from("\t#[ink(constructor)]\n"));
    header.push_str("\tpub fn new(");

    header.push_str(
        constructor
            .header
            .params
            .iter()
            .map(|param| format!("{}: {}", param.name, param.param_type))
            .collect::<Vec<String>>()
            .join(", ")
            .as_str(),
    );
    header.push_str(") -> Self {\n");
    output_vec.push(header);

    output_vec.push(String::from(
        "\t\tink_lang::codegen::initialize_contract(|instance: &mut Self| {\n",
    ));
    for statement in constructor.body.iter() {
        // TODO remove comments
        output_vec.push(format!("\t\t\t// {}\n", statement.content));
    }
    output_vec.push(String::from("\t\t})\n"));
    output_vec.push(String::from("\t}\n\n"));

    output_vec
}

/// This function will assemble the constructor of the ink! contract from the parsed contract
fn assemble_functions(functions: Vec<Function>) -> Vec<String> {
    let mut output_vec = Vec::<String>::new();
    // we do this so we dont put tab between each string that we insert (function params)
    let mut header: String;

    for function in functions.iter() {
        header = String::new();
        if function.header.external {
            output_vec.push(format!(
                "\t#[ink(message{})]\n",
                if function.header.payable {
                    String::from(", payable")
                } else {
                    String::from("")
                }
            ));
        }
        header.push_str(
            format!(
                "\t{}{}(",
                if function.header.external {
                    String::from("pub fn ")
                } else {
                    String::from("fn _")
                },
                function.header.name.to_case(Case::Snake)
            )
            .as_str(),
        );
        // arguments
        header.push_str(
            format!(
                "&{}self",
                if function.header.view {
                    String::from("")
                } else {
                    String::from("mut ")
                }
            )
            .as_str(),
        );
        for param in function.header.params.iter() {
            header.push_str(format!(", {}: {}", param.name, param.param_type).as_str());
        }
        header.push_str(")");
        // return params
        if !function.header.return_params.is_empty() {
            header.push_str(" -> ");
            if function.header.return_params.len() > 1 {
                header.push_str("(");
            }
            for i in 0..function.header.return_params.len() {
                header.push_str(
                    format!(
                        "{}{}",
                        if i > 0 {
                            String::from(", ")
                        } else {
                            String::from("")
                        },
                        function.header.return_params[i]
                    )
                    .as_str(),
                );
            }
            if function.header.return_params.len() > 1 {
                header.push_str(")");
            }
        }
        header.push_str(" {\n");
        output_vec.push(header.to_owned());
        // body
        for statement in function.body.iter() {
            // TODO remove comments
            output_vec.push(format!("\t\t// {}\n", statement.content));
        }
        // TODO remove todo
        output_vec.push(String::from("\t\ttodo!()\n"));
        output_vec.push(String::from("\t}\n"));
        output_vec.push(String::from("\n"));
    }

    output_vec
}
