use proc_macro2::Ident;
use quote::quote;
use syn::{Fields, FieldsNamed, ItemStruct, parse_quote, token};
use crate::processor::composite_struct::composite_operations::utility_operations::helpers::get_first_generic_type_arg;

pub fn process_required(structure: &ItemStruct) -> ItemStruct {
    let mut new_struct = structure.clone();
    new_struct.ident = Ident::new(&format!("{}Required", structure.ident), structure.ident.span());
    // Wrap fields with Optional
    let fields = structure.fields.iter().map(|field| {
        let mut new_field = field.clone();
        let new_type = get_first_generic_type_arg(&new_field);
        if let Some(new_type) = new_type {
            new_field.ty = parse_quote! {
                #new_type
            };
        }
        new_field
    }).collect();
    new_struct.fields = Fields::Named(FieldsNamed {
        brace_token: token::Brace::default(),
        named: fields
    });
    new_struct
}

// unit tests
#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use super::*;
    use syn::{parse_quote, parse_str};

    #[test]
    fn test_process_required() {
        let input: ItemStruct = parse_quote! {
            struct Test {
               field: FieldType
            }
        };

        let expected: ItemStruct = parse_quote! {
            struct Test {
               field: FieldType
            }
        };

        let actual = process_required(&input);
        assert_eq!(
            actual.to_token_stream().to_string(),
            expected.to_token_stream().to_string());
    }

    #[test]
    fn test_process_required_with_optional() {
        let input: ItemStruct = parse_quote! {
            struct Test {
               field: Option<FieldType>
            }
        };

        let expected: ItemStruct = parse_quote! {
            struct TestRequired {
               field: FieldType
            }
        };

        let actual = process_required( &input);
        assert_eq!(
            actual.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_process_required_non_optional() {
        let input: ItemStruct = parse_quote! {
            struct Test {
               field: FieldType<Booper>
            }
        };

        let expected: ItemStruct = parse_quote! {
            struct Test {
               field: FieldType<Booper>
            }
        };

        let actual = process_required(&input);
        assert_eq!(
            actual.to_token_stream().to_string(),
            expected.to_token_stream().to_string());
    }
}