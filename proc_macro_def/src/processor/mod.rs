mod utility_operators;
mod helpers;

use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use proc_macro2::Ident;
use quote::ToTokens;
use crate::parser::composite_struct::CompositeOperation;
use crate::parser::type_input::{InputType, TypeInput};
use syn::{ItemStruct, Result};
use crate::processor::helpers::generate_not_found_error;
use crate::processor::utility_operators::process_utility_operator;

pub struct State {
    /// Used to
    pub variables: BTreeMap<String, ItemStruct>
}

pub fn process_input<'a>(input: TypeInput) -> Result<State> {
    let mut state = State {
        variables: BTreeMap::new()
    };
    for item in input.items.iter() {
        let assignment_ident = get_item_name(item);
        validate_no_conflicts(&state, assignment_ident).unwrap();
        // Run different processing logic for each operation
        match item {
            InputType::CompositeStruct(comp) => {
                process_composite_operation(&mut state, assignment_ident, &comp.composite_operation)?
            },
            InputType::Definition(def) => {
                state.variables.insert(assignment_ident.to_string(), def.to_owned());
            }
        };
    }
    Ok(state)
}

fn get_item_name(item: &InputType) -> &Ident {
    match item {
        InputType::CompositeStruct(comp) => &comp.name,
        InputType::Definition(def) => &def.ident
    }
}

fn process_composite_operation(mut state: &mut State, assignment_ident: &Ident,
                               composite_operation: &CompositeOperation) -> Result<()>{
    match composite_operation {
        CompositeOperation::TypeAlias(original) => {
            set_type_alias(&mut state, assignment_ident, &original)?
        },
        CompositeOperation::UtilityOp(uo) => {
            process_utility_operator(&mut state, assignment_ident, &uo)?;
        }
    };
    Ok(())
}

fn set_type_alias(state: &mut State, alias: &Ident, composite: &Ident) -> Result<()> {
    let alias_string = alias.to_string();
    let composite_string = composite.to_string();
    let composite = state.variables.get(composite_string.as_str());
    match composite {
        None => return generate_not_found_error(alias),
        Some(_struct) => {
            state.variables.insert(alias_string, _struct.to_owned());
        }
    }
    return Ok(())
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
                field1: Optional<Ident>
            }
            struct MyStruct1 = Required(Ident);
            struct MyStruct2 = Required(Ident);
        };

        let actual = process_input(input).unwrap();

        let expected = State {
            variables: btreemap! {
                "MyStruct".to_string() => parse_quote! {
                    struct MyStruct {
                        field1: Optional<Ident>
                    }
                },
                "MyStruct1".to_string() => parse_quote! {
                    struct MyStruct {
                        field1: Optional<Ident>
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

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for item in self.variables.iter() {
            let (key, value) = item;
            writeln!(f, "{}: {}", key, value.to_token_stream().to_string())?;
        }
        Ok(())
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.variables.iter().all(|(key, value)| {
            let other_value = other.variables.get(key);
            match other_value {
                None => false,
                Some(other_value) =>
                    value.to_token_stream().to_string() == other_value.to_token_stream().to_string()
            }
        })
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}