// macros/src/lib.rs
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(DatumComponent)]
pub fn datum_component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let snake_case_name = to_snake_case(&name.to_string());

    let (field_type, field_enum_variant) = match &input.data {
        Data::Struct(data_struct) => {
            if let Fields::Unnamed(fields) = &data_struct.fields {
                if let Some(field) = fields.unnamed.first() {
                    let ty = &field.ty;
                    let variant = match ty.to_token_stream().to_string().as_str() {
                        "bool" => quote! { Datum::Bool },
                        "f64" => quote! { Datum::F64 },
                        "usize" => quote! { Datum::Enum },
                        "i64" => quote! { Datum::I64 },
                        _ => panic!("Unsupported type for DatumComponent"),
                    };
                    (ty, variant)
                } else {
                    panic!("Expected a tuple struct with one Datum")
                }
            } else {
                panic!("Expected a tuple struct")
            }
        }
        _ => panic!("Expected a struct"),
    };

    let gen = quote! {

        impl DatumComponent for #name {
            fn field_key(&self) -> String {
                #snake_case_name.to_owned()
            }

            fn field_value(&self) -> Datum {
                #field_enum_variant(self.0)
            }

            fn set_value(&mut self, new_val: Datum) {
                self.0 = match new_val {
                    #field_enum_variant(val) => val,
                    _ => panic!("Type mismatch when setting value"),
                }
            }

            fn insert(&self, commands: &mut Commands, entity_to_insert_to: Entity) {
                commands.entity(entity_to_insert_to).insert(self.clone());
            }
        }

        impl #name {
            pub fn key() -> String {
                #snake_case_name.to_owned()
            }
        }

        impl MutatorTrait<#field_type> for #name {
            fn set(val: #field_type) -> Mutator {
                Mutator::Set(#snake_case_name.to_string(), #field_enum_variant(val))
            }
            fn increase(val: #field_type) -> Mutator {
                Mutator::Increment(#snake_case_name.to_string(), #field_enum_variant(val))
            }
        }

        impl Precondition<#field_type> for #name {
            fn is(val: #field_type) -> (String, Compare) {
                (#snake_case_name.to_string(), Compare::Equals(#field_enum_variant(val)))
            }
            fn is_not(val: #field_type) -> (String, Compare) {
                (#snake_case_name.to_string(), Compare::NotEquals(#field_enum_variant(val)))
            }
            fn is_more(val: #field_type) -> (String, Compare) {
                (#snake_case_name.to_string(), Compare::GreaterThanEquals(#field_enum_variant(val)))
            }
        }

        // impl<T> MutatorTrait<T> for #name
        //     where
        //         T: EnumDatum
        //     {
        //         fn set(val: T) -> Mutator {
        //             Mutator::Set(#snake_case_name.to_string(), val.datum())
        //         }
        //     }


        // impl MutatorTrait<#field_type> for #name {
        //     fn set(val: #field_type) -> Mutator {
        //         Mutator::Set(#snake_case_name.to_string(), Datum::Enum(val as usize))
        //     }
        // }
    };
    gen.into()
}

#[proc_macro_derive(EnumComponent)]
pub fn enum_component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let snake_case_name = to_snake_case(&name.to_string());

    let field_enum_variant = quote! { Datum::Enum };
    let field_type = match &input.data {
        Data::Struct(data_struct) => {
            if let Fields::Unnamed(fields) = &data_struct.fields {
                if let Some(field) = fields.unnamed.first() {
                    let ty = &field.ty;
                    ty
                    // let variant = match ty.to_token_stream().to_string().as_str() {
                    //     "bool" => quote! { Datum::Bool },
                    //     "f64" => quote! { Datum::F64 },
                    //     "usize" => quote! { Datum::Enum },
                    //     "i64" => quote! { Datum::I64 },
                    //     _ => panic!("Unsupported type for DatumComponent"),
                    // };
                    // (ty, variant)
                } else {
                    panic!("Expected a tuple struct with one Datum")
                }
            } else {
                panic!("Expected a tuple struct")
            }
        }
        _ => panic!("Expected a struct"),
    };

    let gen = quote! {

        impl DatumComponent for #name {
            fn field_key(&self) -> String {
                #snake_case_name.to_owned()
            }

            fn field_value(&self) -> Datum {
                #field_enum_variant(self.0 as usize)
            }

            // This is where we're stuck, TODO
            // TODO asd
            // This needs to actually set the value :/
            // But we want and it expects, to be a Location
            // But the trait requires it to be a Datum
            // Could we turn a Datum::Enum into a Location somehow?
            // Maybe with macros or whatever
            fn set_value(&mut self, new_val: Datum) { // This is wrong, should be Datum
                                                            // according to trait, but is a
                                                            // Location
                                                            //
                // self.0 = new_val
                // self.0 = match new_val {
                //     #field_enum_variant(val) => val,
                //     _ => panic!("Type mismatch when setting value"),
                // }
            }

            fn insert(&self, commands: &mut Commands, entity_to_insert_to: Entity) {
                commands.entity(entity_to_insert_to).insert(self.clone());
            }
        }

        impl #name {
            pub fn key() -> String {
                #snake_case_name.to_owned()
            }
        }

        impl MutatorTrait<#field_type> for #name {
            fn set(val: #field_type) -> Mutator {
                Mutator::Set(#snake_case_name.to_string(), #field_enum_variant(val as usize))
            }
            fn increase(val: #field_type) -> Mutator {
                panic!("Invalid for enums")
            }
        }

        impl Precondition<#field_type> for #name {
            fn is(val: #field_type) -> (String, Compare) {
                (#snake_case_name.to_string(), Compare::Equals(#field_enum_variant(val as usize)))
            }
            fn is_not(val: #field_type) -> (String, Compare) {
                (#snake_case_name.to_string(), Compare::NotEquals(#field_enum_variant(val as usize)))
            }
            fn is_more(val: #field_type) -> (String, Compare) {
                (#snake_case_name.to_string(), Compare::GreaterThanEquals(#field_enum_variant(val as usize)))
            }
        }

        // impl<T> MutatorTrait<T> for #name
        //     where
        //         T: EnumDatum
        //     {
        //         fn set(val: T) -> Mutator {
        //             Mutator::Set(#snake_case_name.to_string(), val.datum())
        //         }
        //     }


        // impl MutatorTrait<#field_type> for #name {
        //     fn set(val: #field_type) -> Mutator {
        //         Mutator::Set(#snake_case_name.to_string(), Datum::Enum(val as usize))
        //     }
        // }
    };
    gen.into()
}

#[proc_macro_derive(ActionComponent)]
pub fn action_component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let snake_case_name = to_snake_case(&name.to_string());

    let gen = quote! {
        impl ActionComponent for #name {
            fn key() -> String {
                #snake_case_name.to_owned()
            }
        }
        impl ActionBuilder for #name {
            fn new() -> Action {
                Action::new(#snake_case_name)
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(EnumDatum)]
pub fn enum_datum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let gen = quote! {
        impl EnumDatum for #name {
            fn datum(self) -> Datum {
                Datum::Enum(self as usize)
            }
        }
    };

    gen.into()
}

fn to_snake_case(s: &str) -> String {
    let mut chars = s.chars().peekable();
    let mut snake_case = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_uppercase() {
            if !snake_case.is_empty() {
                snake_case.push('_');
            }
            snake_case.extend(c.to_lowercase());
        } else {
            snake_case.push(c);
        }
        chars.next();
    }
    snake_case
}
