use proc_macro2::Ident;
use crate::parser::composite_struct::CompositeOperation;
use crate::processor::errors::generate_not_found_error;
use crate::processor::State;
use syn::{ItemStruct, Result};
use crate::processor::composite_struct::composite_operations::type_alias::process_type_alias;
use crate::processor::composite_struct::composite_operations::utility_operations::process_utility_operator;

mod utility_operations;
mod type_alias;

pub fn process_composite_operation(mut state: &mut State, assignment_ident: &Ident,
                               composite_operation: &CompositeOperation) -> Result<ItemStruct>{
    match composite_operation {
        CompositeOperation::TypeAlias(original) => {
            process_type_alias(&mut state, assignment_ident, &original)
        },
        CompositeOperation::UtilityOp(uo) => {
            process_utility_operator(&mut state, &uo)
        }
    }
}