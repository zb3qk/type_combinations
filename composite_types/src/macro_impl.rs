use crate::parser::type_input::TypeInput;
use crate::processor::process_input;

pub fn composite_type_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match composite_type_processing_pipeline(input) {
        Ok(output) => output,
        Err(e) => e.to_compile_error()
    }
}

fn composite_type_processing_pipeline(input: proc_macro2::TokenStream)
                                      -> syn::Result<proc_macro2::TokenStream> {
    let input: TypeInput = syn::parse2(input)?;
    let state = process_input(input)?;
    let output = state.expand();
    Ok(output)
}

// unit tests
#[cfg(test)]
mod tests {
    use quote::quote;
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_composite_type() {
        let input = quote! {
            struct Example {
                field: Option<FieldType>
            }
            struct MyStruct = Required(Example);
            struct MyStruct2 = Required(MyStruct);
            struct MyStruct3 = Optional(MyStruct);
        };

        let expected = quote! {
            struct Example {
                field: Option<FieldType>
            }
            struct MyStruct {
                field: FieldType
            }
            struct MyStruct2 {
                field: FieldType
            }
            struct MyStruct3 {
                field: Option<FieldType>
            }
        };

        let actual = composite_type_impl(input.into());
        assert_eq!(actual.to_string(), expected.to_string());
    }
}