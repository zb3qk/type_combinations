use proc_macro2::Ident;
use syn::{ItemStruct, Result};

pub fn generate_not_found_error(ident: &Ident) -> Result<ItemStruct> {
    let ident_string = ident.to_string();
    let error_message = format!(r#"Type `{0}` does not exist in the scope of the macro.
        Please ensure that the type is defined before it is used. For example:
         `struct {0} = ...` or `struct {0} {{ ... }}` within the scope of compose_type!"#, ident_string);
    return Err(syn::Error::new(ident.span(), error_message.as_str()))
}