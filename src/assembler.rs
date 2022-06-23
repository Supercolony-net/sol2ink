use std::collections::HashSet;

use crate::{
    formatter::append_and_tab,
    structures::*,
};
use convert_case::{
    Case,
    Casing,
};

/// Assembles ink! contract from the parsed contract struct and return it as a vec of Strings
pub fn assemble_contract(contract: Contract) -> Vec<String> {
    let mut output_vec = Vec::<String>::new();

    output_vec.append(signature().as_mut());
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
    append_and_tab(&mut output_vec, assemble_imports(contract.imports));
    output_vec.push(String::from("\n"));
    // enums
    append_and_tab(&mut output_vec, assemble_events(contract.events));
    // events
    append_and_tab(&mut output_vec, assemble_enums(contract.enums));
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

/// Assembles ink! interface(trait) from the parsed interface struct and return it as a vec of Strings
pub fn assemble_interface(interface: Interface) -> Vec<String> {
    let mut output_vec = Vec::<String>::new();

    output_vec.append(signature().as_mut());
    // imports
    output_vec.append(assemble_imports(interface.imports).as_mut());
    output_vec.push(String::from("\n"));

    // events
    output_vec.append(assemble_events(interface.events).as_mut());
    // enums
    output_vec.append(assemble_enums(interface.enums).as_mut());
    // structs
    output_vec.append(assemble_structs(interface.structs).as_mut());

    output_vec.push(format!(
        "#[brush::wrapper]\npub type {0}Ref = dyn {0};\n\n",
        interface.name
    ));
    output_vec.push(format!(
        "#[brush::trait_definition]\npub trait {} {{\n",
        interface.name
    ));

    // functions
    output_vec.append(assemble_function_headers(interface.function_headers).as_mut());
    output_vec.push(String::from("}\n"));

    output_vec
}

/// Sorts the imports inside the HashSet and return it as a Vec of Strings
fn assemble_imports(imports: HashSet<String>) -> Vec<String> {
    let mut output_vec = Vec::from_iter(imports);
    output_vec.sort();
    output_vec
}

/// Assembles ink! enums from the vec of parsed Enum structs and return them as a vec of Strings
fn assemble_enums(enums: Vec<Enum>) -> Vec<String> {
    let mut output_vec = Vec::<String>::new();

    for enumeration in enums.iter() {
        output_vec.push(format!("pub enum {} {{\n", enumeration.name));
        for value in enumeration.values.iter() {
            output_vec.push(format!("\t{}, \n", value));
        }
        output_vec.push(String::from("}\n\n"));
    }

    output_vec
}

/// Assembles ink! events from the vec of parsed Event structs and return them as a vec of Strings
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
                event_field.name.to_case(Case::Snake),
                event_field.field_type
            ));
        }
        output_vec.push(String::from("}\n\n"));
    }

    output_vec
}

/// Assembles ink! storage struct from the vec of parsed ContractField structs and return it as a vec of Strings
fn assemble_storage(contract_name: String, fields: Vec<ContractField>) -> Vec<String> {
    let mut output_vec = Vec::<String>::new();

    output_vec.push(String::from("#[ink(storage)]\n"));
    output_vec.push(String::from("#[derive(Default, SpreadAllocate)]\n"));

    output_vec.push(format!("pub struct {} {{\n", contract_name));
    for field in fields.iter() {
        output_vec.push(format!(
            "\t{}: {},\n",
            field.name.to_case(Case::Snake),
            field.field_type
        ));
    }

    output_vec.push(String::from("}\n"));
    output_vec.push(String::from("\n"));

    output_vec
}

/// Assembles ink! structs from the vec of parsed Struct structs and return them as a vec of Strings
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
                struct_field.name.to_case(Case::Snake),
                struct_field.field_type
            ));
        }
        output_vec.push(String::from("}\n\n"));
    }

    output_vec
}

/// Assembles ink! cosntructor from the parsed Function struct and return it as a vec of Strings
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
            .map(|param| format!("{}: {}", param.name.to_case(Case::Snake), param.param_type))
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
        output_vec.push(format!(
            "\t\t\t{}{}\n",
            if statement.comment { "// " } else { "" },
            statement.content
        ));
    }
    output_vec.push(String::from("\t\t})\n"));
    output_vec.push(String::from("\t}\n\n"));

    output_vec
}

/// Assembles ink! functions from the vec of parsed Function structs and return them as a vec of Strings
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
            header.push_str(
                format!(
                    ", {}: {}",
                    param.name.to_case(Case::Snake),
                    param.param_type
                )
                .as_str(),
            );
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
                        function.header.return_params[i].param_type
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
            output_vec.push(format!(
                "\t\t{}{}\n",
                if statement.comment { "// " } else { "" },
                statement.content
            ));
        }
        // TODO remove todo
        output_vec.push(String::from("\t\ttodo!()\n"));
        output_vec.push(String::from("\t}\n"));
        output_vec.push(String::from("\n"));
    }

    output_vec
}

/// Assembles ink! trait function headers from the vec of parsed FunctionHeader structs and return them as a vec of Strings
fn assemble_function_headers(function_headers: Vec<FunctionHeader>) -> Vec<String> {
    let mut output_vec = Vec::<String>::new();
    // we do this so we dont put tab between each string that we insert (function params)
    let mut header: String;

    for function in function_headers.iter() {
        header = String::new();
        output_vec.push(format!(
            "\t#[ink(message{})]\n",
            if function.payable {
                String::from(", payable")
            } else {
                String::from("")
            }
        ));
        header.push_str(format!("\tfn {}(", function.name.to_case(Case::Snake)).as_str());
        // arguments
        header.push_str(
            format!(
                "&{}self",
                if function.view {
                    String::from("")
                } else {
                    String::from("mut ")
                }
            )
            .as_str(),
        );
        for param in function.params.iter() {
            header.push_str(
                format!(
                    ", {}: {}",
                    param.name.to_case(Case::Snake),
                    param.param_type
                )
                .as_str(),
            );
        }
        header.push_str(")");
        // return params
        if !function.return_params.is_empty() {
            header.push_str(" -> ");
            if function.return_params.len() > 1 {
                header.push_str("(");
            }
            for i in 0..function.return_params.len() {
                header.push_str(
                    format!(
                        "{}{}",
                        if i > 0 {
                            String::from(", ")
                        } else {
                            String::from("")
                        },
                        function.return_params[i].param_type
                    )
                    .as_str(),
                );
            }
            if function.return_params.len() > 1 {
                header.push_str(")");
            }
        }
        header.push_str(";\n\n");
        output_vec.push(header.to_owned());
    }

    output_vec
}

/// Adds a signature to the beginning of the file :)
fn signature() -> Vec<String> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    vec![
        format!("// Generated with Sol2Ink v{}\n", VERSION),
        String::from("// https://github.com/Supercolony-net/sol2ink\n\n"),
    ]
}
