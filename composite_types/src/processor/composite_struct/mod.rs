use proc_macro2::Ident;
use syn::{ItemStruct, parse_quote};
use crate::parser::composite_struct::CompositeStruct;
use crate::processor::composite_struct::composite_operations::process_composite_operation;
use crate::processor::state::State;
use syn::Result;

pub mod composite_operations;

pub fn process_composite_struct(mut state: &mut State, assignment: &Ident,
                                comp: &CompositeStruct) -> Result<ItemStruct> {
    let mut processed_structure = process_composite_operation(&mut state,
                                                              assignment,
                                                              &comp.composite_operation)?;
    if comp.pub_token.is_some() {
        processed_structure.vis = parse_quote! { pub };
    }
    Ok(processed_structure)
}