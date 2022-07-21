use std::{
    collections::HashSet,
    str::FromStr,
};

use crate::structures::*;
use convert_case::{
    Case::Snake,
    Casing,
};
use proc_macro2::TokenStream;
use quote::*;

/// Assembles ink! contract from the parsed contract struct and return it as a vec of Strings
pub fn assemble_contract(contract: Contract) -> TokenStream {
    let mod_name = format_ident!("{}", contract.name.to_case(Snake));
    let contract_name = format_ident!("{}", contract.name);
    let signature = signature();
    let imports = assemble_imports(contract.imports);
    let events = assemble_events(contract.events);
    let enums = assemble_enums(contract.enums);
    let structs = assemble_structs(contract.structs);
    let storage = assemble_storage(&contract.name, contract.fields);
    let constructor = assemble_constructor(contract.constructor);
    let functions = assemble_functions(contract.functions);
    let comments = assemble_contract_comments(contract.comments);

    let contract = quote! {
        #![cfg_attr(not(feature = "std"), no_std)]
        #![feature(min_specialization)]
        _blank_!();
        #signature
        #comments
        #[brush::contract]
        pub mod #mod_name {
            #imports

            #[derive(Debug, Encode, Decode, PartialEq)]
            #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
            pub enum Error {
                Custom(String),
            }
            _blank_!();

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

    contract
}

/// Assembles ink! interface(trait) from the parsed interface struct and return it as a vec of Strings
pub fn assemble_interface(interface: Interface) -> TokenStream {
    let interface_name = TokenStream::from_str(&format!("{}", interface.name)).unwrap();
    let interface_name_ref = TokenStream::from_str(&format!("{}Ref", interface.name)).unwrap();
    let signature = signature();
    let imports = assemble_imports(interface.imports);
    let events = assemble_events(interface.events);
    let enums = assemble_enums(interface.enums);
    let structs = assemble_structs(interface.structs);
    let function_headers = assemble_function_headers(interface.function_headers);

    let interface = quote! {
        #signature
        #imports
        #events
        #enums
        #structs
        #[brush::wrapper]
        pub type #interface_name_ref = dyn #interface_name;
        _blank_!();
        #[brush::trait_definition]
        pub trait #interface_name {
            #function_headers
        }
    };

    interface
}

fn assemble_contract_comments(comments: Vec<String>) -> TokenStream {
    let mut output = TokenStream::new();

    // assemble comments
    for comment in comments.iter() {
        output.extend(quote! {
            #[doc = #comment]
        });
    }

    output
}

/// Sorts the imports inside the HashSet and return it as a Vec of Strings
fn assemble_imports(imports: HashSet<String>) -> TokenStream {
    let mut output = TokenStream::new();
    let mut output_vec = Vec::from_iter(imports);
    output_vec.sort();

    for import in output_vec {
        output.extend(TokenStream::from_str(&import).unwrap());
    }

    output.extend(quote! {
        _blank_!();
    });

    output
}

/// Assembles ink! enums from the vec of parsed Enum structs and return them as a vec of Strings
fn assemble_enums(enums: Vec<Enum>) -> TokenStream {
    let mut output = TokenStream::new();

    for enumeration in enums.iter() {
        let enum_name = TokenStream::from_str(&format!("{}", enumeration.name)).unwrap();
        let mut enum_comments = TokenStream::new();
        let mut values = TokenStream::new();

        // assemble comments
        for comment in enumeration.comments.iter() {
            enum_comments.extend(quote! {
                #[doc = #comment]
            });
        }

        // assemble enum values
        for value in enumeration.values.iter() {
            let value_name = TokenStream::from_str(value).unwrap();

            values.extend(quote! {
                #value_name,
            });
        }

        output.extend(quote! {
            #enum_comments
            pub enum #enum_name {
                #values
            }
            _blank_!();
        });
    }

    output
}

/// Assembles ink! events from the vec of parsed Event structs and return them as a vec of Strings
fn assemble_events(events: Vec<Event>) -> TokenStream {
    let mut output = TokenStream::new();

    for i in 0..events.len() {
        let event = &events[i];
        let event_name = TokenStream::from_str(&format!("{}", event.name)).unwrap();
        let mut event_comments = TokenStream::new();
        let mut event_fields = TokenStream::new();

        // assemble comments
        for comment in event.comments.iter() {
            event_comments.extend(quote! {
                #[doc = #comment]
            });
        }

        // assemble event fields
        for event_field in event.fields.iter() {
            if event_field.indexed {
                event_fields.extend(quote! {
                    #[ink(topic)]
                });
            }

            let event_field_name = format_ident!("{}", event_field.name.to_case(Snake));
            let event_field_type = TokenStream::from_str(&event_field.field_type).unwrap();

            event_fields.extend(quote! {
                #event_field_name: #event_field_type,
            });
        }

        output.extend(quote! {
            #event_comments
            #[ink(event)]
            pub struct #event_name
            {
                #event_fields
            }
            _blank_!();
        });
    }

    output
}

/// Assembles ink! storage struct from the vec of parsed ContractField structs and return it as a vec of Strings
fn assemble_storage(contract_name: &String, fields: Vec<ContractField>) -> TokenStream {
    let mut output = TokenStream::new();
    let contract_name = format_ident!("{}", contract_name);
    let mut storage_fields = TokenStream::new();

    // assemble storage fields
    for field in fields.iter() {
        let field_name = format_ident!("{}", field.name.to_case(Snake));
        let field_type = TokenStream::from_str(&field.field_type).unwrap();

        for comment in field.comments.iter() {
            storage_fields.extend(quote! {
                #[doc = #comment]
            });
        }
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
        _blank_!();
    });

    output
}

/// Assembles ink! structs from the vec of parsed Struct structs and return them as a vec of Strings
fn assemble_structs(structs: Vec<Struct>) -> TokenStream {
    let mut output = TokenStream::new();

    for i in 0..structs.len() {
        let structure = &structs[i];
        let struct_name = TokenStream::from_str(&structure.name).unwrap();
        let mut struct_comments = TokenStream::new();
        let mut struct_fields = TokenStream::new();

        // assemble comments
        for comment in structure.comments.iter() {
            struct_comments.extend(quote! {
                #[doc = #comment]
            });
        }

        // assemble struct fields
        for struct_field in structure.fields.iter() {
            let struct_field_name = format_ident!("{}", struct_field.name.to_case(Snake));
            let struct_field_type = TokenStream::from_str(&struct_field.field_type).unwrap();

            struct_fields.extend(quote! {
                #struct_field_name: #struct_field_type,
            });
        }

        output.extend(quote! {
            #struct_comments
            #[derive(Default, Encode, Decode)]
            #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
            pub struct #struct_name {
                #struct_fields
            }
        });

        output.extend(quote! {
            _blank_!();
        });
    }

    output
}

/// Assembles ink! cosntructor from the parsed Function struct and return it as a vec of Strings
fn assemble_constructor(constructor: Function) -> TokenStream {
    let mut output = TokenStream::new();
    let mut params = TokenStream::new();
    let mut comments = TokenStream::new();
    let constructor_functions = constructor.body;

    // assemble comments
    for comment in constructor.header.comments.iter() {
        comments.extend(quote! {
            #[doc = #comment]
        });
    }

    // assemble params
    for param in constructor.header.params.iter() {
        let param_name = format_ident!("{}", param.name.to_case(Snake));
        let param_type = TokenStream::from_str(&param.param_type).unwrap();

        params.extend(quote! {
            #param_name: #param_type,
        });
    }

    let mut body = TokenStream::new();

    // assemble body
    body.extend(quote! {
        #(#constructor_functions)*
    });

    output.extend(quote! {
        #comments
        #[ink(constructor)]
        pub fn new(#params) -> Self{
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                #body
            })
        }
        _blank_!();
    });

    output
}

/// Assembles ink! functions from the vec of parsed Function structs and return them as a vec of Strings
fn assemble_functions(functions: Vec<Function>) -> TokenStream {
    let mut output = TokenStream::new();

    for function in functions.iter() {
        let mut message = TokenStream::new();
        let mut function_name = TokenStream::new();
        let mut view = TokenStream::new();
        let mut params = TokenStream::new();
        let mut return_params = TokenStream::new();
        let mut body = TokenStream::new();
        let mut comments = TokenStream::new();

        // assemble comments
        for comment in function.header.comments.iter() {
            comments.extend(quote! {
                #[doc = #comment]
            });
        }
        let statements = &function.body;

        // assemble message
        if function.header.external {
            if function.header.payable {
                message.extend(quote! {
                    #[ink(message, payable)]
                });
            } else {
                message.extend(quote! {
                    #[ink(message)]
                });
            }
        }

        // assemble function name
        function_name.extend(
            TokenStream::from_str(&format!(
                "{}{}",
                if function.header.external {
                    String::from("pub fn ")
                } else {
                    String::from("fn _")
                },
                function.header.name.to_case(Snake)
            ))
            .unwrap(),
        );

        // assemble view
        view.extend(
            TokenStream::from_str(match function.header.view {
                true => "&self",
                false => "&mut self",
            })
            .unwrap(),
        );

        // assemble params
        for param in function.header.params.iter() {
            let param_name = format_ident!("{}", param.name.to_case(Snake));
            let param_type = TokenStream::from_str(&param.param_type).unwrap();

            params.extend(quote! {
                , #param_name: #param_type
            });
        }

        // assemble return params
        if function.header.return_params.len() > 0 {
            let mut params = TokenStream::new();
            for i in 0..function.header.return_params.len() {
                let param_type =
                    TokenStream::from_str(&function.header.return_params[i].param_type).unwrap();

                if i > 0 {
                    params.extend(quote! {,});
                }
                params.extend(quote! {
                    #param_type
                });
            }

            if function.header.return_params.len() > 1 {
                return_params.extend(quote! {
                    (#params)
                });
            } else {
                return_params.extend(quote! {
                    #params
                });
            }
        } else {
            return_params.extend(quote! {
                ()
            });
        }

        // body
        body.extend(quote! {
            #(#statements)*
        });

        if function.header.return_params.is_empty() {
            body.extend(quote! {
                Ok(())
            });
        }

        output.extend(quote! {
            #comments
            #message
            #function_name(#view #params) -> Result<#return_params, Error> {
                #body
            }
        });

        output.extend(quote! {
            _blank_!();
        });
    }

    output
}

impl ToTokens for Operation {
    fn to_tokens(&self, stream: &mut TokenStream) {
        stream.extend(match self {
            Operation::Not => quote!(!),
            Operation::True => quote!(),
            Operation::GreaterThanEqual => quote!(>=),
            Operation::GreaterThan => quote!(>),
            Operation::LessThanEqual => quote!(<=),
            Operation::LessThan => quote!(<),
            Operation::LogicalAnd => quote!(&&),
            Operation::LogicalOr => quote!(||),
            Operation::Equal => quote!(==),
            Operation::NotEqual => quote!(!=),
        })
    }
}

/// Assembles ink! trait function headers from the vec of parsed FunctionHeader structs and return them as a vec of Strings
fn assemble_function_headers(function_headers: Vec<FunctionHeader>) -> TokenStream {
    let mut output = TokenStream::new();

    for i in 0..function_headers.len() {
        let function = &function_headers[i];
        let mut function_comments = TokenStream::new();
        let mut message = TokenStream::new();
        let mut function_name = TokenStream::new();
        let mut view = TokenStream::new();
        let mut params = TokenStream::new();
        let mut return_params = TokenStream::new();

        // assemble comments
        for comment in function.comments.iter() {
            function_comments.extend(quote! {
                #[doc = #comment]
            });
        }

        // assemble message
        if function.external {
            if function.payable {
                message.extend(quote! {
                    #[ink(message, payable)]
                });
            } else {
                message.extend(quote! {
                    #[ink(message)]
                });
            }
        }

        // assemble function name
        function_name.extend(
            TokenStream::from_str(&format!("fn {}", function.name.to_case(Snake))).unwrap(),
        );

        // assemble view
        view.extend(
            TokenStream::from_str(match function.view {
                true => "&self",
                false => "&mut self",
            })
            .unwrap(),
        );

        // assemble params
        for param in function.params.iter() {
            let param_name = format_ident!("{}", param.name.to_case(Snake));
            let param_type = TokenStream::from_str(&param.param_type).unwrap();

            params.extend(quote! {
                , #param_name: #param_type
            });
        }

        // assemble return params
        if function.return_params.len() > 0 {
            let mut params = TokenStream::new();
            for i in 0..function.return_params.len() {
                let param_type =
                    TokenStream::from_str(&function.return_params[i].param_type).unwrap();

                if i > 0 {
                    params.extend(quote! {,});
                }
                params.extend(quote! {
                    #param_type
                });
            }

            if function.return_params.len() > 1 {
                return_params.extend(quote! {
                    (#params)
                });
            } else {
                return_params.extend(quote! {
                    #params
                });
            }
        } else {
            return_params.extend(quote! {
                ()
            });
        }

        output.extend(quote! {
            #function_comments
            #message
            #function_name(#view #params) -> Result<#return_params, Error>;
        });

        if i < function_headers.len() - 1 {
            output.extend(quote! {
                _blank_!();
            });
        }
    }

    output
}

/// Adds a signature to the beginning of the file :)
fn signature() -> TokenStream {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let version = &format!("Generated with Sol2Ink v{}\n", VERSION);
    let link = "https://github.com/Supercolony-net/sol2ink\n";
    quote! {
        _comment_!(#version);
        _comment_!(#link);
        _blank_!();
    }
}

impl ToTokens for Statement {
    fn to_tokens(&self, stream: &mut TokenStream) {
        match self {
            Statement::AssemblyEnd => {}
            Statement::Assign(left_raw, right_raw) => {
                let left = TokenStream::from_str(&left_raw.to_string()).unwrap();
                let right = TokenStream::from_str(&right_raw.to_string()).unwrap();
                stream.extend(quote! {
                    #left = #right;
                })
            }
            Statement::AddAssign(left_raw, right_raw) => {
                let left = TokenStream::from_str(&left_raw.to_string()).unwrap();
                let right = TokenStream::from_str(&right_raw.to_string()).unwrap();
                stream.extend(quote! {
                    #left += #right;
                })
            }
            Statement::Assembly(statements) => {
                stream.extend(quote! {
                        #(#statements)*
                })
            }
            Statement::Catch(statements) => {
                stream.extend(quote! {
                    else {
                        #(#statements)*
                    }
                    _comment_!("<<< Please handle try/catch blocks manually");
                })
            }
            Statement::CatchEnd => {}
            Statement::Comment(content) => {
                stream.extend(quote! {
                    _comment_!(#content);
                })
            }
            Statement::Declaration(var_name_raw, var_type_raw, initial_value_maybe) => {
                let var_name = format_ident!("{}", var_name_raw.to_case(Snake));
                let var_type = TokenStream::from_str(var_type_raw).unwrap();
                if let Some(initial_value_raw) = &initial_value_maybe {
                    let initial_value =
                        TokenStream::from_str(&initial_value_raw.to_string()).unwrap();
                    stream.extend(quote!(let #var_name : #var_type = #initial_value;));
                } else {
                    stream.extend(quote!(let #var_name : #var_type;));
                }
            }
            Statement::Else(statements) => {
                stream.extend(quote! {
                    else {
                        #(#statements)*
                    }
                })
            }
            Statement::Emit(event_name_raw, args_raw) => {
                let args_str = args_raw
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                let args = TokenStream::from_str(args_str.as_str()).unwrap();
                let event_name = TokenStream::from_str(event_name_raw).unwrap();
                stream.extend(quote! {
                    self.env().emit_event(#event_name {
                        #args
                    });
                })
            }
            Statement::FunctionCall(expression_raw) => {
                let expression =
                    TokenStream::from_str(&expression_raw.to_string()).unwrap_or_else(|_| {
                        panic!(
                            "expression_raw: {expression_raw:?} to_string: {}",
                            expression_raw.to_string()
                        )
                    });
                stream.extend(quote! {
                    #expression;
                })
            }
            Statement::If(condition_raw, statements) => {
                let left = TokenStream::from_str(&condition_raw.left.to_string()).unwrap();
                let operation = condition_raw.operation;
                let condition = if let Some(condition) = &condition_raw.right {
                    let right = TokenStream::from_str(&condition.to_string()).unwrap();
                    quote!(#left #operation #right)
                } else {
                    quote!(#operation #left)
                };
                stream.extend(quote! {
                    if #condition {
                        #(#statements)*
                    }
                })
            }
            Statement::IfEnd => {}
            Statement::Raw(_) => {}
            Statement::Require(condition_raw, error_raw) => {
                let left = TokenStream::from_str(&condition_raw.left.to_string()).unwrap();
                let operation = condition_raw.operation;
                let error = TokenStream::from_str(&error_raw).unwrap();
                let condition = if let Some(condition) = &condition_raw.right {
                    let right = TokenStream::from_str(&condition.to_string()).unwrap();
                    quote!(#left #operation #right)
                } else {
                    quote!(#operation #left)
                };
                stream.extend(quote! {
                    if #condition {
                        #error
                    }
                })
            }
            Statement::Return(output_raw) => {
                let output = TokenStream::from_str(&output_raw.to_string()).unwrap();
                stream.extend(quote! {
                    return Ok(#output)
                })
            }
            Statement::SubAssign(left_raw, right_raw) => {
                let left = TokenStream::from_str(&left_raw.to_string()).unwrap();
                let right = TokenStream::from_str(&right_raw.to_string()).unwrap();
                stream.extend(quote! {
                    #left -= #right;
                })
            }
            Statement::Try(statements) => {
                stream.extend(quote! {
                    _comment_!("Please handle try/catch blocks manually >>>");
                    if true {
                        #(#statements)*
                    }
                })
            }
            Statement::TryEnd => {}
        }
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        return match self {
            Expression::Addition(left, right) => {
                format!("{} + {}", left.to_string(), right.to_string())
            }
            Expression::Condition(condition_raw) => {
                let left = condition_raw.left.to_string();
                let operation = condition_raw.operation.to_string();
                if let Some(right_raw) = &condition_raw.right {
                    let right = right_raw.to_string();
                    format!("{left} {operation} {right}")
                } else {
                    format!("{operation} {left}")
                }
            }
            Expression::EnvCaller(selector) => {
                format!("{}.env().caller()", selector.clone().unwrap())
            }
            Expression::FunctionCall(function_name_raw, args, selector, external) => {
                format!(
                    "{}.{}{}({})?",
                    selector,
                    if *external {
                        String::from("")
                    } else {
                        String::from("_")
                    },
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
            Expression::Logical(left_raw, operation_raw, right_raw) => {
                let left = left_raw.to_string();
                let operation = operation_raw.to_string();
                let right = right_raw.to_string();
                format!("{left} {operation} {right}")
            }
            Expression::Member(expression_raw, selector_raw) => {
                let expression = expression_raw.to_case(Snake);
                if let Some(selector) = selector_raw {
                    format!("{selector}.{expression}")
                } else {
                    expression
                }
            }
            Expression::Mapping(name_raw, indices_raw, selector_raw, insert_maybe) => {
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
                let name_and_selector = if let Some(selector) = selector_raw {
                    format!("{selector}.{name}")
                } else {
                    format!("{name}")
                };
                if let Some(insert_raw) = insert_maybe {
                    let insert = insert_raw.to_string();
                    format!("{name_and_selector}.insert(&{indices}, {insert})")
                } else {
                    format!("{name_and_selector}.get(&{indices}).unwrap()")
                }
            }
            Expression::Subtraction(left, right) => {
                format!("{} - {}", left.to_string(), right.to_string())
            }
            Expression::StructArg(field_name, value) => {
                format!("{} : {}", field_name.to_case(Snake), value.to_string())
            }
            Expression::Ternary(condition_raw, if_true, if_false) => {
                let left = condition_raw.left.to_string();
                let operation = condition_raw.operation.to_string();
                let condition = if let Some(right_raw) = &condition_raw.right {
                    let right = right_raw.to_string();
                    format!("{left} {operation} {right}")
                } else {
                    format!("{operation} {left}")
                };
                format!(
                    "if {} {{{}}} else {{{}}}",
                    condition,
                    if_true.to_string(),
                    if_false.to_string()
                )
            }
            Expression::ZeroAddressInto => String::from("ZERO_ADDRESS.into()"),
        }
    }
}
