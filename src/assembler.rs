use std::{
    collections::HashSet,
    str::FromStr,
};

use crate::structures::*;
use convert_case::{
    Case,
    Casing,
};
use proc_macro2::TokenStream;
use quote::*;

/// Assembles ink! contract from the parsed contract struct and return it as a vec of Strings
pub fn assemble_contract(contract: Contract) -> TokenStream {
    let contract_name = format_ident!("{}", contract.name.to_case(Case::Snake));
    let signature = signature();
    let imports = assemble_imports(contract.imports);
    let events = assemble_events(contract.events);
    let enums = assemble_enums(contract.enums);
    let structs = assemble_structs(contract.structs);
    let storage = assemble_storage(contract.name.clone(), contract.fields);
    let constructor = assemble_constructor(contract.constructor);
    let functions = assemble_functions(contract.functions);

    let ink_contract = quote! {
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
    };

    ink_contract
}

/// Assembles ink! interface(trait) from the parsed interface struct and return it as a vec of Strings
pub fn assemble_interface(interface: Interface) -> TokenStream {
    let interface_name = format_ident!("{}", interface.name);
    let interface_name_ref = format_ident!("{}Ref", interface.name);
    let signature = signature();
    let imports = assemble_imports(interface.imports);
    let events = assemble_events(interface.events);
    let enums = assemble_enums(interface.enums);
    let structs = assemble_structs(interface.structs);
    let function_headers = assemble_function_headers(interface.function_headers);

    let ink_contract = quote! {
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
    };

    ink_contract
}

/// Sorts the imports inside the HashSet and return it as a Vec of Strings
fn assemble_imports(imports: HashSet<String>) -> TokenStream {
    let mut output = TokenStream::new();
    let mut output_vec = Vec::from_iter(imports);
    output_vec.sort();

    for import in output_vec {
        output.extend(TokenStream::from_str(&import).unwrap());
    }

    output
}

/// Assembles ink! enums from the vec of parsed Enum structs and return them as a vec of Strings
fn assemble_enums(enums: Vec<Enum>) -> TokenStream {
    let mut output = TokenStream::new();

    for enumeration in enums.iter() {
        let enum_name = format_ident!("{}", enumeration.name);
        let mut values = TokenStream::new();

        for value in enumeration.values.iter() {
            let value_ident = format_ident!("{}", value);

            values.extend(quote! {
                #value_ident,
            });
        }

        output.extend(quote! {
            pub enum #enum_name {
                #values
            }
        });
    }

    output
}

/// Assembles ink! events from the vec of parsed Event structs and return them as a vec of Strings
fn assemble_events(events: Vec<Event>) -> TokenStream {
    let mut output = TokenStream::new();

    for event in events.iter() {
        let event_name = format_ident!("{}", event.name);
        let mut event_fields = TokenStream::new();

        for event_field in event.fields.iter() {
            if event_field.indexed {
                event_fields.extend(quote! {
                    #[ink(topic)]
                });
            }

            let event_field_name = format_ident!("{}", event_field.name);
            let event_field_type = TokenStream::from_str(&event_field.field_type).unwrap();

            event_fields.extend(quote! {
                #event_field_name: #event_field_type,
            });
        }

        output.extend(quote! {
            #[ink(event)]
            pub struct #event_name
            {
                #event_fields
            }
        });
    }

    output
}

/// Assembles ink! storage struct from the vec of parsed ContractField structs and return it as a vec of Strings
fn assemble_storage(contract_name: String, fields: Vec<ContractField>) -> TokenStream {
    let mut output = TokenStream::new();
    let contract_name = format_ident!("{}", contract_name.to_case(Case::Snake));

    let mut storage_fields = TokenStream::new();

    for field in fields.iter() {
        let field_name = format_ident!("{}", field.name);
        let field_type = TokenStream::from_str(&field.field_type).unwrap();
        storage_fields.extend(quote! {
            #field_name: #field_type,
        });
    }

    output.extend(quote! {
        #[ink(storage)]
        #[derive(Default, SpreadAllocate)]
        pub struct #contract_name {
            #storage_fields
        }
    });

    output
}

/// Assembles ink! structs from the vec of parsed Struct structs and return them as a vec of Strings
fn assemble_structs(structs: Vec<Struct>) -> TokenStream {
    let mut output = TokenStream::new();

    for structure in structs.iter() {
        let struct_name = format_ident!("{}", structure.name);
        let mut struct_fields = TokenStream::new();

        for struct_field in structure.fields.iter() {
            let struct_field_name = format_ident!("{}", struct_field.name);
            let struct_field_type = TokenStream::from_str(&struct_field.field_type).unwrap();

            struct_fields.extend(quote! {
                #struct_field_name: #struct_field_type,
            });
        }
        output.extend(quote! {
            #[derive(Default, Encode, Decode)]
            #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
            pub struct #struct_name {
                #struct_fields
            }
        });
    }

    output
}

/// Assembles ink! cosntructor from the parsed Function struct and return it as a vec of Strings
fn assemble_constructor(constructor: Function) -> TokenStream {
    let mut output = TokenStream::new();
    let mut params = TokenStream::new();

    for param in constructor.header.params.iter() {
        let param_name = format_ident!("{}", param.name);
        let param_type = TokenStream::from_str(&param.param_type).unwrap();

        params.extend(quote! {
            #param_name: #param_type,
        });
    }

    let mut body = TokenStream::new();

    for statement in constructor.body.iter() {
        // TODO remove comments
        let content = format!("\t\t\t// {}\n", statement.content);
        body.extend(TokenStream::from_str(&content).unwrap());
    }

    output.extend(quote! {
        #[ink(constructor)]
        pub fn new(#params) -> Self{
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                #body
            })
        }
    });

    output
}

/// Assembles ink! functions from the vec of parsed Function structs and return them as a vec of Strings
fn assemble_functions(functions: Vec<Function>) -> TokenStream {
    let mut output = TokenStream::new();

    for function in functions.iter() {
        header = TokenStream::new();
        if function.header.external {
            let msg = format!(
                "\t#[ink(message{})]\n",
                if function.header.payable {
                    String::from(", payable")
                } else {
                    String::from("")
                }
            );
            header.extend(TokenStream::from_str(&msg).unwrap());
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

        let mut params = TokenStream::new();

        for param in function.header.params.iter() {
            let param_name = format_ident!("{}", param.name);
            let param_type = TokenStream::from_str(&param.param_type).unwrap();

            params.extend(quote! {
                #param_name: #param_type,
            });
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

    quote! {#(#output_vec)*}
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

    quote! {#(#output_vec)*}
}

/// Adds a signature to the beginning of the file :)
fn signature() -> TokenStream {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let version = format!("Generated with Sol2Ink v{}", VERSION);
    let link = String::from("https://github.com/Supercolony-net/sol2ink");
    quote! {
        // Generated with Sol2Ink v\n
        #[doc = #version]
        #[doc = #link]
    }
}
