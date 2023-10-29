use std::collections::BTreeMap;
use quote::ToTokens;
use syn::ItemStruct;
use std::fmt::{Debug, Formatter};


/// Storage for the State of the processor.
pub struct State {
    /// Stores the state of exposed data structures and intermediate data structures.
    pub variables: BTreeMap<String, ItemStruct>
}

impl State {
    pub fn new() -> Self {
        State {
            variables: BTreeMap::new()
        }
    }

    /// Expands the state into a TokenStream representing the macro expanded tokens.
    pub fn expand(&self) -> proc_macro2::TokenStream {
        let mut output = proc_macro2::TokenStream::new();
        for item in self.variables.iter() {
            let (_, value) = item;
            output.extend(value.to_token_stream());
        }
        output
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_expand() {
        let mut state = State::new();
        let structure: ItemStruct = parse_quote! {
            struct Test {
                field: FieldType
            }
        };
        state.variables.insert("Test".to_string(), structure);

        let expected: proc_macro2::TokenStream = parse_quote! {
            struct Test {
                field: FieldType
            }
        };

        let actual = state.expand();
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn test_expand_multiple() {
        let mut state = State::new();
        let structure: ItemStruct = parse_quote! {
            struct Test {
                field: FieldType
            }
        };
        state.variables.insert("Test".to_string(), structure);

        let structure: ItemStruct = parse_quote! {
            struct Test2 {
                field: FieldType
            }
        };
        state.variables.insert("Test2".to_string(), structure);

        let structure: ItemStruct = parse_quote! {
            struct Test3 {
                field: Test2
            }
        };
        state.variables.insert("Test3".to_string(), structure);

        let expected: proc_macro2::TokenStream = parse_quote! {
            struct Test {
                field: FieldType
            }

            struct Test2 {
                field: FieldType
            }

            struct Test3 {
                field: Test2
            }
        };

        let actual = state.expand();
        assert_eq!(actual.to_string(), expected.to_string());
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