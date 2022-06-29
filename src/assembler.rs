use std::collections::HashSet;

use crate::structures::*;
use convert_case::{
    Case,
    Casing,
};
use proc_macro2::TokenStream;
use quote::*;

/// Assembles ink! contract from the parsed contract struct and return it as a vec of Strings
pub fn assemble_contract(contract: Contract) -> TokenStream {
    let contract_name = contract.name.to_case(Case::Snake);
    let signature = signature();
    let imports = assemble_imports(contract.imports);
    let events = assemble_events(contract.events);
    let enums = assemble_enums(contract.enums);
    let structs = assemble_structs(contract.structs);
    let storage = assemble_storage(contract.name.clone(), contract.fields);
    let constructor = assemble_constructor(contract.constructor);
    let functions = assemble_functions(contract.functions);

    let ink_contract = quote!(
        #signature

        #![cfg_attr(not(feature = "std"), no_std)]
        #![feature(min_specialization)]
        #[brush::contract]
        pub mod #contract_name {
            #imports
            #events
            #enums
            #structs
            #storage
            impl #contract_name {
                #constructor
                #functions
            }
        }
    );

    ink_contract
}

/// Assembles ink! interface(trait) from the parsed interface struct and return it as a vec of Strings
pub fn assemble_interface(interface: Interface) -> TokenStream {
    let interface_name = interface.name;
    let interface_name_ref = format!("{}Ref", interface_name);
    let signature = signature();
    let imports = assemble_imports(interface.imports);
    let events = assemble_events(interface.events);
    let enums = assemble_enums(interface.enums);
    let structs = assemble_structs(interface.structs);
    let function_headers = assemble_function_headers(interface.function_headers);

    let ink_contract = quote!(
        #signature
        #imports
        #events
        #enums
        #structs

        #[brush::wrapper]
        pub type #interface_name_ref = dyn #interface_name;

        #[brush::trait_definition]
        pub trait #interface_name {
            #function_headers
        }
    );

    ink_contract
}

/// Sorts the imports inside the HashSet and return it as a Vec of Strings
fn assemble_imports(imports: HashSet<String>) -> TokenStream {
    let mut output_vec = Vec::from_iter(imports);
    output_vec.sort();
    quote!(#(#output_vec)"\n"*)
}

/// Assembles ink! enums from the vec of parsed Enum structs and return them as a vec of Strings
fn assemble_enums(enums: Vec<Enum>) -> TokenStream {
    let mut output_vec = Vec::<String>::new();

    for enumeration in enums.iter() {
        output_vec.push(format!("pub enum {} {{\n", enumeration.name));
        for value in enumeration.values.iter() {
            output_vec.push(format!("\t{}, \n", value));
        }
        output_vec.push(String::from("}\n\n"));
    }
    quote!(#(#output_vec)*)
}

/// Assembles ink! events from the vec of parsed Event structs and return them as a vec of Strings
fn assemble_events(events: Vec<Event>) -> TokenStream {
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

    quote!(#(#output_vec)*)
}

/// Assembles ink! storage struct from the vec of parsed ContractField structs and return it as a vec of Strings
fn assemble_storage(contract_name: String, fields: Vec<ContractField>) -> TokenStream {
    let mut output_vec = Vec::<String>::new();

    output_vec.push(String::from("#[ink(storage)]\n"));
    output_vec.push(String::from("#[derive(Default, SpreadAllocate)]\n"));

    output_vec.push(format!("pub struct {} {{\n", contract_name));
    for field in fields.iter() {
        output_vec.push(format!("\t{}: {},\n", field.name, field.field_type));
    }

    output_vec.push(String::from("}\n"));
    output_vec.push(String::from("\n"));

    quote!(#(#output_vec)*)
}

/// Assembles ink! structs from the vec of parsed Struct structs and return them as a vec of Strings
fn assemble_structs(structs: Vec<Struct>) -> TokenStream {
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

    quote!(#(#output_vec)*)
}

/// Assembles ink! cosntructor from the parsed Function struct and return it as a vec of Strings
fn assemble_constructor(constructor: Function) -> TokenStream {
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

    quote!(#(#output_vec)*)
}

/// Assembles ink! functions from the vec of parsed Function structs and return them as a vec of Strings
fn assemble_functions(functions: Vec<Function>) -> TokenStream {
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

    quote!(#(#output_vec)*)
}

/// Assembles ink! trait function headers from the vec of parsed FunctionHeader structs and return them as a vec of Strings
fn assemble_function_headers(function_headers: Vec<FunctionHeader>) -> TokenStream {
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
            header.push_str(format!(", {}: {}", param.name, param.param_type).as_str());
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
                        function.return_params[i]
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

    quote!(#(#output_vec)*)
}

/// Adds a signature to the beginning of the file :)
fn signature() -> TokenStream {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let version = format!("// Generated with Sol2Ink v{}\n", VERSION);
    let link = String::from("// https://github.com/Supercolony-net/sol2ink\n\n");
    quote!(
        // Generated with Sol2Ink v\n
        #[doc = #version]
        #[doc = #link]
    )
}
