mod errors;
mod state;
mod composite_struct;

use std::collections::BTreeMap;
use proc_macro2::Ident;
use quote::ToTokens;
use crate::parser::type_input::{InputType, TypeInput};
use syn::{ItemStruct, Result};

use crate::processor::composite_struct::process_composite_struct;
use crate::processor::state::State;

pub fn process_input<'a>(input: TypeInput) -> Result<State> {
    let mut state = State {
        variables: BTreeMap::new()
    };
    for item in input.items.iter() {
        let assignment_ident = get_item_name(item);
        validate_no_conflicts(&state, assignment_ident).unwrap();
        // Run different processing logic for each operation
        let new_structure: ItemStruct = match item {
            InputType::CompositeStruct(comp) => {
                process_composite_struct(&mut state, assignment_ident, comp)?
            },
            InputType::Definition(def) => def.to_owned()
        };
        apply_structure(&mut state, assignment_ident, new_structure);
    }
    Ok(state)
}

fn apply_structure(state: &mut State, assignment: &Ident, mut structure: ItemStruct) {
    structure.ident = assignment.clone();
    state.variables.insert(assignment.to_string(), structure.clone());
}

fn get_item_name(item: &InputType) -> &Ident {
    match item {
        InputType::CompositeStruct(comp) => &comp.name,
        InputType::Definition(def) => &def.ident
    }
}

fn validate_no_conflicts(state: &State, key: &Ident) -> Result<()> {
    if state.variables.contains_key(key.to_string().as_str()) {
        return Err(syn::Error::new(key.span(),"This identifier is already in use"))
    }
    Ok(())
}

// unit tests
#[cfg(test)]
mod tests {
    use maplit::btreemap;
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_process_input() {
        let input = parse_quote! {
            struct MyStruct {
                field1: Option<Ident>
            }
            struct MyStruct1 = Required(MyStruct);
            struct MyStruct2 = Required(MyStruct);
        };

        let actual = process_input(input).unwrap();

        let expected = State {
            variables: btreemap! {
                "MyStruct".to_string() => parse_quote! {
                    struct MyStruct {
                        field1: Option<Ident>
                    }
                },
                "MyStruct1".to_string() => parse_quote! {
                    struct MyStruct1 {
                        field1: Ident
                    }
                },
                "MyStruct2".to_string() => parse_quote! {
                    struct MyStruct2 {
                        field1: Ident
                    }
                }
            }
        };
        assert_eq!(actual, expected);
    }
}