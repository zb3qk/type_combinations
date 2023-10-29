use std::fmt::{Debug, Formatter};
use quote::ToTokens;
use syn::ItemStruct;
use syn::parse::{Parse, ParseStream};
use crate::parser::composite_struct::CompositeStruct;

#[derive(Debug, PartialEq)]
pub struct TypeInput {
    pub items: Vec<InputType>,
}

pub enum InputType {
    CompositeStruct(CompositeStruct),
    Definition(ItemStruct)
}

impl Parse for InputType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(composite) = input.fork().parse::<CompositeStruct>() {
            let _ = input.parse::<CompositeStruct>();
            return Ok(InputType::CompositeStruct(composite));
        } else if let Ok(definition) = input.fork().parse::<ItemStruct>() {
            let _ = input.parse::<ItemStruct>();
            return Ok(InputType::Definition(definition));
        } else {
            Err(input.error("Expected composite struct or type definition"))
        }
    }
}

impl Parse for TypeInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();

        // Continue parsing while there's more content in the input.
        while !input.is_empty() {
            items.push(input.parse::<InputType>()?);
        }

        Ok(TypeInput { items })
    }
}

// implement unit tests
#[cfg(test)]
mod tests {
    use proc_macro2::{Ident, Span};
    use super::*;
    use syn::{parse2, parse_quote};
    use crate::parser::composite_struct::CompositeOperation;
    use crate::parser::composite_struct::utility_operations::UtilityOperation;

    #[test]
    fn test_parse_type_input() {
        let input = parse_quote! {
            struct MyStruct = Required(Ident);
            struct MyStruct2 = Required(Ident);
        };

        let actual = parse2::<TypeInput>(input).unwrap();

        let expected = TypeInput {
            items: vec![
                InputType::CompositeStruct(CompositeStruct::new(
                    Ident::new("MyStruct", Span::call_site()),
                    CompositeOperation::UtilityOp(
                        UtilityOperation::Required(Ident::new("Ident", Span::call_site())))
                )),
                InputType::CompositeStruct(CompositeStruct::new(
                    Ident::new("MyStruct2", Span::call_site()),
                    CompositeOperation::UtilityOp(
                        UtilityOperation::Required(Ident::new("Ident", Span::call_site())))
                )),
            ]
        };
        assert_eq!(actual, expected);
    }
}

impl Debug for InputType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::CompositeStruct(cs) => write!(f, "{:?}", cs),
            InputType::Definition(def) => write!(f, "{:?}", def.to_token_stream()),
        }
    }
}

impl PartialEq for InputType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InputType::CompositeStruct(cs1), InputType::CompositeStruct(cs2)) => cs1 == cs2,
            (InputType::Definition(def1), InputType::Definition(def2)) =>
                def1.to_token_stream().to_string() == def2.to_token_stream().to_string(),
            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}