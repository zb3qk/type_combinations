use proc_macro2::Ident;
use quote::quote;
use syn::{Fields, FieldsNamed, ItemStruct, token};

pub fn process_optional(structure: &ItemStruct) -> ItemStruct {
    let mut new_struct = structure.clone();
    // Wrap fields with Optional

    let fields = structure.fields.iter().map(|field| {
        new_struct.ident = Ident::new(&format!("{}Optional", structure.ident), structure.ident.span());
        let mut new_field = field.clone();
        let field_type = &field.ty;
        // If the field is not already an optional, wrap it in an optional
        if let syn::Type::Path(type_path) = field_type {
            if let Some(ident) = type_path.path.segments.first() {
                let field_type = ident.ident.to_string();
                if field_type != "Option" {
                    new_field.ty = syn::parse2(quote! { Option<#type_path> }).unwrap();
                }
            }
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
    fn test_process_optional() {
        let input: ItemStruct = parse_quote! {
            struct Test {
               field: FieldType
            }
        };

        let expected: ItemStruct = parse_quote! {
            struct TestOptional {
               field: Option<FieldType>
            }
        };

        let actual = process_optional(&input);
        assert_eq!(
            actual.to_token_stream().to_string(),
            expected.to_token_stream().to_string());
    }

    #[test]
    fn test_process_optional_with_optional() {
        let input: ItemStruct = parse_quote! {
            struct Test {
               field: Option<FieldType>
            }
        };

        let expected: ItemStruct = parse_quote! {
            struct TestOptional {
               field: Option<FieldType>
            }
        };

        let actual = process_optional(&input);
        assert_eq!(
            actual.to_token_stream().to_string(),
            expected.to_token_stream().to_string());
    }

    #[test]
    fn test_process_optional_complicated() {
        let input: ItemStruct = parse_quote! {
            struct Test {
               field: FieldType::Nested<'a, AnotherFieldType>
            }
        };

        let expected: ItemStruct = parse_quote! {
            struct TestOptional {
               field: Option<FieldType::Nested<'a, AnotherFieldType>>
            }
        };

        let actual = process_optional(&input);
        assert_eq!(
            actual.to_token_stream().to_string(),
            expected.to_token_stream().to_string());
    }
}