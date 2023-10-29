use std::fmt::{Debug, Display, Formatter};
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use crate::parser::composite_struct::helpers::AngleBracketedIdents;

pub enum UtilityOperation {
    Required(Ident),
    Optional(Ident),
}

impl Parse for UtilityOperation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let operator = input.parse::<Ident>()?;
        let params = AngleBracketedIdents::parse(input)?;
        match operator.to_string().as_str() {
            "Required" => {
                let param = validate_one_param(params)?;
                return Ok(UtilityOperation::Required(param))
            },
            "Optional" => {
                let param = validate_one_param(params)?;
                return Ok(UtilityOperation::Optional(param))
            },
            _ => {
                Err(syn::Error::new(operator.span(), "Expected Required or Optional"))
            }
        }
    }
}

fn validate_one_param(params: AngleBracketedIdents) -> syn::Result<Ident> {
    let num_params = params.items.len();
    if num_params != 1 {
        return Err(syn::Error::new(params.span,
                            format!("Expected 1 parameter, but instead found {}", num_params)));
    }
    Ok(params.items.first().unwrap().to_owned())
}

// implement unit tests
#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::parse::Parser;
    use super::*;
    use syn::{parse2, parse_quote};

    #[test]
    fn test_parse_utility_operation() {
        let input = parse_quote! {
            Required<Ident>
        };

        let actual = parse2::<UtilityOperation>(input).unwrap();

        let expected = UtilityOperation::Required(Ident::new("Ident", Span::call_site()));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_utility_operation_with_multiple_params() {
        let input = parse_quote! {
            Required<Ident, Ident>
        };

        let actual = parse2::<UtilityOperation>(input);

        assert!(actual.is_err());
    }

    #[test]
    fn test_parse_utility_operation_with_no_params() {
        let input = parse_quote! {
            Required<>
        };

        let actual = parse2::<UtilityOperation>(input);

        assert!(actual.is_err());
    }

    #[test]
    fn test_parse_utility_operation_with_optional() {
        let input = parse_quote! {
            Optional<Ident>
        };

        let actual = parse2::<UtilityOperation>(input).unwrap();

        let expected = UtilityOperation::Optional(Ident::new("Ident", Span::call_site()));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_utility_operation_with_invalid_operator() {
        let input = parse_quote! {
            Invalid<Ident>
        };

        let actual = parse2::<UtilityOperation>(input);

        assert!(actual.is_err());
    }
}

fn format_util_op(util_op: &UtilityOperation, f: &mut Formatter<'_>) -> std::fmt::Result {
    match util_op {
        UtilityOperation::Required(ident) => {
            write!(f, "Required<{}>", ident)
        },
        UtilityOperation::Optional(ident) => {
            write!(f, "Optional<{}>", ident)
        }
    }
}

impl Debug for UtilityOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_util_op(self, f)
    }
}

impl Display for UtilityOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_util_op(self, f)
    }
}

impl PartialEq for UtilityOperation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (UtilityOperation::Required(ident1), UtilityOperation::Required(ident2)) => {
                ident1 == ident2
            },
            (UtilityOperation::Optional(ident1), UtilityOperation::Optional(ident2)) => {
                ident1 == ident2
            },
            _ => false
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}