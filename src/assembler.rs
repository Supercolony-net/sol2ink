use std::{
    collections::HashSet,
    str::FromStr,
};

use crate::structures::*;
use convert_case::{
    Case::{
        Pascal,
        Snake,
    },
    Casing,
};
use proc_macro2::{
    Ident,
    TokenStream,
};
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
    let storage = assemble_storage(&contract.name, &contract.fields);
    let constructor = assemble_constructor(contract.constructor, &contract.fields);
    let constants = assemble_constants(contract.fields);
    let functions = assemble_functions(contract.functions);
    let comments = assemble_contract_doc(contract.contract_doc);
    let modifiers = assemble_modifiers(contract.modifiers, &contract_name);

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

            #constants
            #modifiers
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
    let interface_name = TokenStream::from_str(&interface.name).unwrap();
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

fn assemble_contract_doc(comments: Vec<String>) -> TokenStream {
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
    let output_vec = Vec::from_iter(imports);

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
        let enum_name = TokenStream::from_str(&enumeration.name.to_case(Pascal)).unwrap();
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
            let value_name = TokenStream::from_str(&value.to_case(Pascal)).unwrap();

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

    for event in events.iter() {
        let event_name = TokenStream::from_str(&event.name).unwrap();
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
fn assemble_storage(contract_name: &String, fields: &[ContractField]) -> TokenStream {
    let mut output = TokenStream::new();
    let contract_name = format_ident!("{}", contract_name);
    let mut storage_fields = TokenStream::new();

    // assemble storage fields
    for field in fields.iter().filter(|field| !field.constant) {
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

/// Assembles constant fields of the contract
fn assemble_constants(fields: Vec<ContractField>) -> TokenStream {
    let mut output = TokenStream::new();

    // assemble storage fields
    for field in fields.iter().filter(|field| field.constant) {
        let field_name = format_ident!("{}", field.name.to_case(Snake));
        let field_type = TokenStream::from_str(&field.field_type).unwrap();
        let initial_value = TokenStream::from_str(&field.initial_value.clone().unwrap()).unwrap();

        for comment in field.comments.iter() {
            output.extend(quote! {
                #[doc = #comment]
            });
        }
        output.extend(quote! {
            pub const #field_name: #field_type = #initial_value;
        });
    }

    output.extend(quote! {
        _blank_!();
    });

    output
}

/// Assembles ink! structs from the vec of parsed Struct structs and return them as a vec of Strings
fn assemble_structs(structs: Vec<Struct>) -> TokenStream {
    let mut output = TokenStream::new();

    for structure in structs.iter() {
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
fn assemble_constructor(constructor: Function, fields: &[ContractField]) -> TokenStream {
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

    for field in fields
        .iter()
        .filter(|field| field.initial_value.is_some() && !field.constant)
    {
        let field_name = format_ident!("{}", field.name.to_case(Snake));
        let intial_value =
            TokenStream::from_str(field.initial_value.clone().unwrap().as_str()).unwrap();

        body.extend(quote! {
            self.#field_name = #intial_value;
        });
    }

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
        let mut function_modifiers = TokenStream::new();
        let statements = &function.body;

        // assemble comments
        for comment in function.header.comments.iter() {
            comments.extend(quote! {
                #[doc = #comment]
            });
        }

        for function_modifier in function.header.modifiers.iter() {
            function_modifiers.extend(quote! {
                #[modifiers(#function_modifier)]
            });
        }

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
        if !function.header.return_params.is_empty() {
            let mut params = TokenStream::new();

            for i in 0..function.header.return_params.len() {
                let param = &function.header.return_params[i];
                let param_type = TokenStream::from_str(&param.param_type).unwrap();

                if i > 0 {
                    params.extend(quote! {,});
                }

                params.extend(quote! {
                    #param_type
                });

                if param.name != "_" {
                    let param_name = TokenStream::from_str(&param.name.to_case(Snake)).unwrap();
                    body.extend(quote! {
                        let mut #param_name = Default::default();
                    })
                }
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
        } else if function.header.return_params[0].name != "_" {
            let out = TokenStream::from_str(
                &function
                    .header
                    .return_params
                    .iter()
                    .map(|param| param.name.clone())
                    .collect::<Vec<String>>()
                    .join(","),
            )
            .unwrap();
            body.extend(
                if function.header.return_params.len() > 1 {
                    quote! {
                        Ok((#out))
                    }
                } else {
                    quote! {
                        Ok(#out)
                    }
                },
            );
        }

        output.extend(quote! {
            #comments
            #message
            #function_modifiers
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

/// Assembles ink! functions from the vec of parsed Function structs and return them as a vec of Strings
fn assemble_modifiers(modifiers: Vec<Modifier>, contract_name: &Ident) -> TokenStream {
    let mut output = TokenStream::new();

    for modifier in modifiers.iter() {
        let modifier_name = format_ident!("{}", modifier.header.name.to_case(Snake));
        let mut body = TokenStream::new();
        let mut comments = TokenStream::new();
        let mut params = TokenStream::new();

        // assemble comments
        for comment in modifier.comments.iter() {
            comments.extend(quote! {
                #[doc = #comment]
            });
        }
        let statements = &modifier.statements;

        // assemble params
        for param in modifier.header.params.iter() {
            let param_name = format_ident!("{}", param.name.to_case(Snake));
            let param_type = TokenStream::from_str(&param.param_type).unwrap();

            params.extend(quote! {
                , #param_name: #param_type
            });
        }

        // body
        body.extend(quote! {
            #(#statements)*
        });

        output.extend(quote! {
            #comments
            #[doc = "The type of `T` should be the trait which implements the storage"]
            #[doc = "This will be implemented in Sol2Ink in upcoming version"]
            #[modifier_definition]
            pub fn #modifier_name<T, F, R>(instance: &mut T, body: F #params) -> Result<R, Error>
            where
                T: #contract_name,
                F: FnOnce(&mut T) -> Result<R, Error>
            {
                #body
            }
        });

        output.extend(quote! {
            _blank_!();
        });
    }

    output
}

/// Assembles ink! trait function headers from the vec of parsed FunctionHeader structs and return them as a vec of Strings
fn assemble_function_headers(function_headers: Vec<FunctionHeader>) -> TokenStream {
    let mut output = TokenStream::new();

    for header in function_headers.iter() {
        let mut function_comments = TokenStream::new();
        let mut message = TokenStream::new();
        let mut function_name = TokenStream::new();
        let mut view = TokenStream::new();
        let mut params = TokenStream::new();
        let mut return_params = TokenStream::new();

        // assemble comments
        for comment in header.comments.iter() {
            function_comments.extend(quote! {
                #[doc = #comment]
            });
        }

        // assemble message
        if header.external {
            if header.payable {
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
        function_name
            .extend(TokenStream::from_str(&format!("fn {}", header.name.to_case(Snake))).unwrap());

        // assemble view
        view.extend(
            TokenStream::from_str(match header.view {
                true => "&self",
                false => "&mut self",
            })
            .unwrap(),
        );

        // assemble params
        for param in header.params.iter() {
            let param_name = format_ident!("{}", param.name.to_case(Snake));
            let param_type = TokenStream::from_str(&param.param_type).unwrap();

            params.extend(quote! {
                , #param_name: #param_type
            });
        }

        // assemble return params
        if !header.return_params.is_empty() {
            let mut params = TokenStream::new();
            for i in 0..header.return_params.len() {
                let param_type =
                    TokenStream::from_str(&header.return_params[i].param_type).unwrap();

                if i > 0 {
                    params.extend(quote! {,});
                }
                params.extend(quote! {
                    #param_type
                });
            }

            if header.return_params.len() > 1 {
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

        output.extend(quote! {
            _blank_!();
        });
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

impl ToTokens for Operation {
    fn to_tokens(&self, stream: &mut TokenStream) {
        stream.extend(match self {
            Operation::Add => quote!(+),
            Operation::AddAssign => quote!(+=),
            Operation::AndAssign => quote!(&=),
            Operation::Assign => quote!(=),
            Operation::BitwiseAnd => quote!(&),
            Operation::BitwiseOr => quote!(|),
            Operation::Div => quote!(/),
            Operation::DivAssign => quote!(/=),
            Operation::Equal => quote!(==),
            Operation::GreaterThanEqual => quote!(>=),
            Operation::GreaterThan => quote!(>),
            Operation::LessThanEqual => quote!(<=),
            Operation::LessThan => quote!(<),
            Operation::LogicalAnd => quote!(&&),
            Operation::LogicalOr => quote!(||),
            Operation::Modulo => quote!(%),
            Operation::Mul => quote!(*),
            Operation::MulAssign => quote!(*=),
            Operation::Not => quote!(!),
            Operation::NotEqual => quote!(!=),
            Operation::OrAssign => quote!(|=),
            Operation::Pow => quote!(),
            Operation::ShiftLeft => quote!(<<),
            Operation::ShiftRight => quote!(>>),
            Operation::Subtract => quote!(-),
            Operation::SubtractAssign => quote!(-=),
            _ => quote!(),
        })
    }
}

impl ToTokens for Statement {
    fn to_tokens(&self, stream: &mut TokenStream) {
        match self {
            Statement::AssemblyEnd => {}
            Statement::Assign(left, right, operation_raw) => {
                let operation = TokenStream::from_str(&operation_raw.to_string()).unwrap();
                stream.extend(quote! {
                    #left #operation #right;
                })
            }
            Statement::Assembly(statements) => {
                stream.extend(quote! {
                        #(#statements)*
                })
            }
            Statement::Catch(statements) => {
                stream.extend(quote! {
                    else if false {
                        #(#statements)*
                        _comment_!("<<< Please handle try/catch blocks manually");
                    }
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
                if let Some(initial_value) = &initial_value_maybe {
                    stream.extend(quote!(let #var_name : #var_type = #initial_value;));
                } else {
                    stream.extend(quote!(let #var_name : #var_type;));
                }
            }
            Statement::Group(statements) => {
                stream.extend(quote! {
                        #(#statements)*
                })
            }
            Statement::Else(statements) => {
                stream.extend(quote! {
                    else {
                        #(#statements)*
                    }
                })
            }
            Statement::ElseIf(condition_raw, statements) => {
                let left = &condition_raw.left;
                let operation = condition_raw.operation;
                let condition = if let Some(right) = &condition_raw.right {
                    quote!(#left #operation #right)
                } else {
                    quote!(#operation #left)
                };
                stream.extend(quote! {
                    else if #condition {
                        #(#statements)*
                    }
                })
            }
            Statement::Emit(event_name_raw, args) => {
                let event_name = TokenStream::from_str(event_name_raw).unwrap();
                stream.extend(quote! {
                    self.env().emit_event(#event_name {
                        #(#args,)*
                    });
                })
            }
            Statement::FunctionCall(expression) => {
                stream.extend(quote! {
                    #expression;
                })
            }
            Statement::If(condition_raw, statements) => {
                let left = &condition_raw.left;
                let operation = condition_raw.operation;
                let condition = if let Some(right) = &condition_raw.right {
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
            Statement::ModifierBody => {
                stream.extend(quote! {
                    body(instance)
                })
            }
            Statement::Raw(_) => {}
            Statement::Require(condition_raw, error_raw) => {
                let left = &condition_raw.left;
                let operation = condition_raw.operation;
                let error = TokenStream::from_str(error_raw).unwrap();
                let condition = if let Some(right) = &condition_raw.right {
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
            Statement::Return(output) => {
                stream.extend(quote! {
                    return Ok(#output)
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
            Statement::While(assign, condition, modification, statements) => {
                stream.extend(quote! {
                    #assign
                    while #condition {
                        #(#statements)*
                        #modification
                    }
                })
            }
            Statement::WhileEnd => {}
        }
    }
}

impl ToTokens for Expression {
    fn to_tokens(&self, stream: &mut TokenStream) {
        stream.extend(match self {
            Expression::Arithmetic(left, right, operation) => {
                if operation == &Operation::Pow {
                    quote!(#left.pow(#right as u32))
                } else {
                    quote!(#left #operation #right)
                }
            }
            Expression::Cast(unique_cast, cast_type_raw, expression) => {
                let cast_type = TokenStream::from_str(cast_type_raw).unwrap();
                if *unique_cast {
                    quote!(#cast_type(#expression))
                } else {
                    quote!((#expression as #cast_type))
                }
            }
            Expression::Condition(condition_raw) => {
                let left = &condition_raw.left;
                let operation = condition_raw.operation;
                if let Some(right_raw) = &condition_raw.right {
                    let right = right_raw;
                    quote!(#left #operation #right)
                } else {
                    quote!(#operation #left)
                }
            }
            Expression::Enclosed(expression) => {
                quote!((#expression))
            }
            Expression::EnvCaller(selector_raw) => {
                let selector =
                    TokenStream::from_str(&selector_raw.clone().unwrap_or_default()).unwrap();
                quote!(#selector.env().caller())
            }
            Expression::FunctionCall(function_name_raw, args_raw, selector_maybe, external) => {
                let mut function_call = TokenStream::new();
                if let Some(selector_raw) = selector_maybe {
                    let selector = TokenStream::from_str(selector_raw).unwrap();
                    function_call.extend(quote!(#selector.))
                }
                let function_name = format_ident!(
                    "{}{}",
                    if *external { "" } else { "_" },
                    function_name_raw.to_case(Snake)
                );
                let mut args = TokenStream::new();
                for arg in args_raw {
                    args.extend(quote!(#arg, ));
                }
                quote! {
                    #function_call #function_name(#args)?
                }
            }
            Expression::IsZero(expression) => {
                quote!(#expression.is_zero())
            }
            Expression::Literal(content) => TokenStream::from_str(content).unwrap(),
            Expression::Logical(left, operation, right) => {
                quote!(#left #operation #right)
            }
            Expression::Member(expression_raw, selector_raw) => {
                let expression = TokenStream::from_str(&expression_raw.to_case(Snake))
                    .unwrap_or_else(|_| panic!("{expression_raw}"));
                if let Some(selector_raw) = selector_raw {
                    let selector = format_ident!("{}", selector_raw);
                    quote!(#selector.#expression)
                } else {
                    quote!(#expression)
                }
            }
            Expression::Mapping(name_raw, indices_raw, selector_raw, insert_maybe) => {
                let indices = if indices_raw.len() > 1 {
                    let mut inner = TokenStream::new();
                    for i in 0..indices_raw.len() {
                        let expression = indices_raw.get(i).unwrap();
                        if i > 0 {
                            inner.extend(quote!(,));
                        }
                        inner.extend(quote!(#expression));
                    }
                    quote!((#inner))
                } else {
                    let expression = indices_raw.get(0).unwrap();
                    quote!(#expression)
                };
                let expression = format_ident!("{}", name_raw.to_case(Snake));
                let name_and_selector = if let Some(selector_raw) = selector_raw {
                    let selector = format_ident!("{}", selector_raw);
                    quote!(#selector.#expression)
                } else {
                    quote!(#expression)
                };
                if let Some(insert) = insert_maybe {
                    quote!(#name_and_selector.insert(&#indices, #insert))
                } else {
                    quote!(#name_and_selector.get(&#indices).unwrap())
                }
            }
            Expression::Modifier(modifier_raw) => {
                let modifier = TokenStream::from_str(modifier_raw).unwrap();
                quote!(#modifier)
            }
            Expression::NewArray(array_type_raw, array_size) => {
                let array_type = TokenStream::from_str(array_type_raw).unwrap();
                quote!(vec![#array_type::default(); #array_size])
            }
            Expression::StructArg(field_name_raw, value) => {
                let field_name = TokenStream::from_str(&field_name_raw.to_case(Snake)).unwrap();
                quote!(#field_name : #value)
            }
            Expression::StructInit(struct_name_raw, struct_args_raw) => {
                let mut struct_args = TokenStream::new();
                for i in 0..struct_args_raw.len() {
                    let expression = struct_args_raw.get(i).unwrap();
                    if i > 0 {
                        struct_args.extend(quote!(,));
                    }
                    struct_args.extend(quote!(#expression));
                }
                let struct_name = TokenStream::from_str(&struct_name_raw.to_case(Pascal)).unwrap();
                quote!(#struct_name {#struct_args})
            }
            Expression::Ternary(condition_raw, if_true, if_false) => {
                let left = &condition_raw.left;
                let operation = condition_raw.operation;
                if let Some(right) = &condition_raw.right {
                    quote! {
                        if #left #operation #right {
                            #if_true
                        } else {
                            #if_false
                        }
                    }
                } else {
                    quote! {
                        if #operation #left {
                            #if_true
                        } else {
                            #if_false
                        }
                    }
                }
            }
            Expression::WithSelector(left, right) => {
                quote!(#left.#right)
            }
            Expression::ZeroAddressInto => quote!(ZERO_ADDRESS.into()),
        })
    }
}
