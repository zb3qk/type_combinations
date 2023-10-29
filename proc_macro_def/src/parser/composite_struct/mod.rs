pub mod utility_operations;
mod helpers;

use std::fmt::{Debug, Formatter};
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{ItemStruct, Token};
use crate::parser::composite_struct::utility_operations::UtilityOperation;

/// Macro specific syntax to represent type composition operations.
pub struct CompositeStruct {
    struct_token: Token![struct],
    pub name: Ident,
    assignment_token: Token![=],
    pub composite_operation: CompositeOperation,
    semi_colon: Option<Token![;]>
}

impl Parse for CompositeStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(CompositeStruct {
            struct_token: input.parse()?,
            name: input.parse()?,
            assignment_token: input.parse()?,
            composite_operation: input.parse()?,
            semi_colon: input.parse().ok(),
        })
    }
}

pub enum CompositeOperation {
    TypeAlias(Ident),
    UtilityOp(UtilityOperation),
}

impl Parse for CompositeOperation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        return if let Ok(uo) = input.fork().parse::<UtilityOperation>() {
            input.parse::<UtilityOperation>().ok();
            Ok(CompositeOperation::UtilityOp(uo))
        } else if input.peek(syn::Ident) {
            let alias = input.parse::<Ident>()?;
            Ok(CompositeOperation::TypeAlias(alias))
        } else {
            Err(input.error("Expected type alias or utility operation"))
        };
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use super::*;
    use syn::{parse2, parse_quote};


    #[test]
    fn test_parse_composite_struct() {
        let input = parse_quote! {
            struct MyStruct = Required(Ident) ;
        };

        let actual = parse2::<CompositeStruct>(input).unwrap();

        let expected = CompositeStruct {
            struct_token: Token![struct](Span::call_site()),
            name: Ident::new("MyStruct", Span::call_site()),
            assignment_token: Token![=](Span::call_site()),
            composite_operation: CompositeOperation::UtilityOp(UtilityOperation::Required(Ident::new("Ident", Span::call_site()))),
            semi_colon: Some(Token![;](Span::call_site()))
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_composite_struct_with_type_alias() {
        let input = parse_quote! {
            struct MyStruct = MyType
        };

        let actual = parse2::<CompositeStruct>(input).unwrap();

        let expected = CompositeStruct {
            struct_token: Token![struct](Span::call_site()),
            name: Ident::new("MyStruct", Span::call_site()),
            assignment_token: Token![=](Span::call_site()),
            composite_operation: CompositeOperation::TypeAlias(Ident::new("MyType", Span::call_site())),
            semi_colon: Some(Token![;](Span::call_site()))
        };
        assert_eq!(actual, expected);
    }
}


impl PartialEq for CompositeOperation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CompositeOperation::TypeAlias(id1), CompositeOperation::TypeAlias(id2)) => id1 == id2,
            (CompositeOperation::UtilityOp(uo1), CompositeOperation::UtilityOp(uo2)) => uo1 == uo2,
            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Debug for CompositeStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.composite_operation {
            CompositeOperation::TypeAlias(id) => {
                write!(f, "struct {} = {}", self.name, id)
            },
            CompositeOperation::UtilityOp(uo) => {
                write!(f, "struct {} = {}", self.name, uo)
            }
        }
    }
}


impl PartialEq for CompositeStruct {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.composite_operation == other.composite_operation
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Debug for CompositeOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            CompositeOperation::TypeAlias(id) => {
                write!(f, "TypeAlias {}", id)
            },
            CompositeOperation::UtilityOp(uo) => {
                write!(f, "Utility {}", uo)
            }
        }
    }
}

impl CompositeStruct {
    pub fn new(name: Ident, composite_operation: CompositeOperation) -> Self {
        CompositeStruct {
            struct_token: Token![struct](Span::call_site()),
            name,
            assignment_token: Token![=](Span::call_site()),
            composite_operation,
            semi_colon: Some(Token![;](Span::call_site())),
        }
    }
}