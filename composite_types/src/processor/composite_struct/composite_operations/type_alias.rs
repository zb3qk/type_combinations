use proc_macro2::Ident;
use crate::processor::errors::generate_not_found_error;
use crate::processor::State;
use syn::{ItemStruct, Result};

pub fn process_type_alias(state: &mut State, alias: &Ident, composite: &Ident) -> Result<ItemStruct> {
    let composite_string = composite.to_string();
    let composite = state.variables.get(composite_string.as_str());
    return match composite {
        None => generate_not_found_error(alias),
        Some(structure) => Ok(structure.clone())
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use super::*;
    use syn::{ItemStruct, parse_quote};

    #[test]
    fn test_set_type_alias() {
        let mut state = State::new();
        let input: Ident = parse_quote! { Test };

        let alias: Ident = parse_quote!(NewStruct);
        let structure: ItemStruct = parse_quote! {
            struct Test {
                field: FieldType
            }
        };

        // Assignment is updated up the call stack at the top level processor
        let expected: ItemStruct = parse_quote! {
            struct Test {
                field: FieldType
            }
        };

        state.variables.insert(input.to_string(), structure);
        let actual = process_type_alias(&mut state, &alias, &input);
        assert!(actual.is_ok());
        let actual = actual.unwrap();

        assert_eq!(
            actual.to_token_stream().to_string(),
            expected.to_token_stream().to_string());
    }

    #[test]
    fn test_set_type_alias_not_found() {
        let mut state = State::new();
        let input = parse_quote! { Test };

        let alias: Ident = parse_quote!(NewStruct);
        let actual = process_type_alias(&mut state, &alias, &input);
        assert!(actual.is_err());
    }
}