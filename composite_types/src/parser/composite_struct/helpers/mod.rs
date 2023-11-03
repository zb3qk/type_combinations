use std::fmt::{Debug, Formatter};
use syn::punctuated::Punctuated;
use syn::{braced, parenthesized, Token};
use proc_macro2::{Ident, Span};
use syn::parse::{Parse, ParseStream, Result as ParseResult};

pub struct AngleBracketedIdents {
    pub items: Punctuated<Ident, Token![,]>,
    pub span: Span
}

impl Parse for AngleBracketedIdents {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let content;
        parenthesized!(content in input);
        let items = Punctuated::parse_terminated(&content)?;
        Ok(AngleBracketedIdents { items, span: input.span() })
    }
}

// implement unit tests
#[cfg(test)]
mod tests {
    use proc_macro2::{Span, TokenStream};
    use syn::parse::Parser;
    use super::*;
    use syn::{parse2, parse_quote};

    #[test]
    fn test_parse_angle_bracketed_idents() {
        let input = parse_quote! {
            (Ident, Ident2)
        };

        let actual = parse2::<AngleBracketedIdents>(input).unwrap();

        let expected = AngleBracketedIdents {
            items: Punctuated::parse_terminated.parse2(parse_quote!(Ident, Ident2)).unwrap(),
            span: Span::call_site()
        };
        assert_eq!(actual, expected);
    }
}


impl Debug for AngleBracketedIdents {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let params = self.items.iter().map(|i| i.to_string()).collect::<Vec<String>>();
        write!(f, "AngleBracketedIdents {{ items: {:?} }}", params)
    }
}

impl PartialEq for AngleBracketedIdents {
    fn eq(&self, other: &Self) -> bool {
        let params_self = self.items.iter().map(|i| i.to_string()).collect::<Vec<String>>();
        let params_other = other.items.iter().map(|i| i.to_string()).collect::<Vec<String>>();
        params_self == params_other
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
