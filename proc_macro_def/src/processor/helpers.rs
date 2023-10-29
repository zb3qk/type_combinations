use proc_macro2::Ident;
use syn::Result;

pub fn generate_not_found_error(ident: &Ident) -> Result<()>{
    return Err(syn::Error::new(ident.span(), format!("Required type {} does not exist", ident.to_string())))
}