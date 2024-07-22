extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// ActionComponent allows you to create Actions directly from your action struct
///
/// See [`bevy_dogoap::prelude::ActionComponent`](../bevy_dogoap/prelude/trait.ActionComponent.html) for full docs
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
            fn new() -> Action {
                Action::new(#snake_case_name)
            }
            fn action_type_name(&self) -> &'static str {
                stringify!(#name)
            }
        }
    };
    gen.into()
}

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
            fn decrease(val: #field_type) -> Mutator {
                Mutator::Decrement(#snake_case_name.to_string(), #field_enum_variant(val))
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
            fn is_less(val: #field_type) -> (String, Compare) {
                (#snake_case_name.to_string(), Compare::LessThanEquals(#field_enum_variant(val)))
            }
        }
    };
    gen.into()
}

/// EnumComponent is specifically for DatumComponent's that use an Enum/EnumDatum
///
/// They're special in the way they'll refuse to be incremented/decremented
/// and compared with "greater" or "less".
///
/// It also handles the `as usize` for you, gives type safety.
///
/// Example:
///
/// ```ignore
/// #[derive(EnumDatum)]
/// struct Location {
///     Home,
///     Outside
/// }
///
/// #[derive(EnumComponent)]
/// struct AtLocation(Location);
///
/// // Used as a Mutator:
/// assert_eq!(
///     AtLocation::set(Location::Home),
///     Mutator::Increment("at_location".to_string(), Datum::Enum(Location::Home as usize))
/// );
///
/// // Used as a Precondition:
/// assert_eq!(
///     AtLocation::is(Location::Outside),
///     ("at_location".to_string(), Compare::Equals(Datum::Enum(Location::Outside as usize)))
/// )
/// ```
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
                    &field.ty
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
                panic!("You cannot call .increase on a Enum!")
            }
            fn decrease(val: #field_type) -> Mutator {
                panic!("You cannot call .increase on a Enum!")
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
                panic!("You cannot call .is_more on a Enum!")
            }
            fn is_less(val: #field_type) -> (String, Compare) {
                panic!("You cannot call .is_less on a Enum!")
            }
        }
    };
    gen.into()
}

/// EnumDatum implements EnumDatum trait so you can use it with an EnumComponent
///
/// See docs for [`EnumComponent`] for example usage
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
