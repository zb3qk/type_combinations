use crate::parser::composite_struct::utility_operations::UtilityOperation;
use crate::processor::State;
use crate::processor::errors::generate_not_found_error;
use syn::{ItemStruct, Result};

use crate::processor::composite_struct::composite_operations::utility_operations::optional::process_optional;
use crate::processor::composite_struct::composite_operations::utility_operations::required::process_required;

mod required;
mod optional;
mod helpers;

pub fn process_utility_operator(state: &mut State,
                                utility_operation: &UtilityOperation) -> Result<ItemStruct> {

    fn process_structure<F>(structure: &ItemStruct, processor: F) -> ItemStruct
        where
            F: FnOnce(&ItemStruct) -> ItemStruct {
        processor(structure)
    }

    return match utility_operation {
        UtilityOperation::Required(ident) | UtilityOperation::Optional(ident) => {
            let structure = state.variables.get(ident.to_string().as_str());
            match structure {
                None => return generate_not_found_error(ident),
                Some(structure) => {
                    let processed_structure = match utility_operation {
                        UtilityOperation::Optional(_) => process_structure(structure, process_optional),
                        UtilityOperation::Required(_) => process_structure(structure, process_required)
                    };
                    Ok(processed_structure)
                }
            }
        }
    }
}


// unit tests
#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_process_utility_operator() {
        let mut state = State::new();
        let input = parse_quote! { Optional(Test) };

        let structure = parse_quote! {
            struct Test {
                field: FieldType
            }
        };
        state.variables.insert("Test".to_string(), structure);

        let expected: ItemStruct = parse_quote! {
            struct TestOptional {
                field: Option<FieldType>
            }
        };

        let actual = process_utility_operator(&mut state, &input).unwrap();
        assert_eq!(
            actual.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_process_utility_operator_with_optional() {
        let mut state = State::new();
        let input = parse_quote! { Required(Test) };

        let structure = parse_quote! {
            struct Test {
                field: Option<FieldType>
            }
        };
        state.variables.insert("Test".to_string(), structure);

        let actual = process_utility_operator(&mut state, &input).unwrap();

        let expected: ItemStruct = parse_quote! {
            struct TestRequired {
                field: FieldType
            }
        };
        assert_eq!(actual.to_token_stream().to_string(), expected.to_token_stream().to_string());
    }
}