extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(DatumComponent)]
pub fn datum_component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let snake_case_name = to_snake_case(&name.to_string());

    let (_field_type, field_enum_variant) = match &input.data {
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
