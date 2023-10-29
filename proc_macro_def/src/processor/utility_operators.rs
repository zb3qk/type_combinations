use proc_macro2::Ident;
use quote::__private::ext::RepToTokensExt;
use quote::quote;
use crate::parser::composite_struct::utility_operations::UtilityOperation;
use crate::processor::State;
use syn::{Fields, FieldsNamed, ItemStruct, Result, token};
use crate::processor::helpers::generate_not_found_error;

pub fn process_utility_operator(state: &mut State, assignment: &Ident,
                            utility_operation: &UtilityOperation) -> Result<()> {
    match utility_operation {
        UtilityOperation::Required(ident) => {
            let structure  = state.variables.get(ident.to_string().as_str());
            match structure {
                None => return generate_not_found_error(ident),
                Some(structure) => {
                    let structure = process_required(assignment.to_owned(), structure);
                    state.variables.insert(assignment.to_string(), structure);
                }
            }
        },
        UtilityOperation::Optional(ident) => {
            let structure  = state.variables.get(ident.to_string().as_str());
            match structure {
                None => return generate_not_found_error(ident),
                Some(structure) => {
                    let structure = process_optional(assignment.to_owned(), structure);
                    state.variables.insert(assignment.to_string(), structure);
                }
            }
        }
    }
    Ok(())
}

fn process_required(assignment: Ident, structure: &ItemStruct) -> ItemStruct {
    let mut new_struct = structure.clone();
    new_struct.ident = assignment;
    // Wrap fields with Optional
    let fields = structure.fields.iter().map(|field| {
        let mut new_field = field.clone();
        let field_type = &field.ty;
        if let syn::Type::Path(path) = field_type {
            let new_field_type = path.path.segments.first();
            let field_type = path.path.segments.first().unwrap().ident.to_string();
            if field_type != "Optional" {
                new_field.ty = syn::parse2(quote! { Optional<#field_type> }).unwrap();
            }
        }
        new_field
    }).collect();
    new_struct.fields = Fields::Named(FieldsNamed {
        brace_token: token::Brace::default(),
        named: fields
    });
    new_struct
}

fn process_optional(assignment: Ident, structure: &ItemStruct) -> ItemStruct {
    let mut new_struct = structure.clone();
    new_struct.ident = assignment;
    // Wrap fields with Optional
    let fields = structure.fields.iter().map(|field| {
        let mut new_field = field.clone();
        let field_type = &field.ty;
        // If the field is not already an optional, wrap it in an optional
        if let syn::Type::Path(path) = field_type {
            if let Some(ident) = path.path.segments.first() {
                let field_type = ident.ident.to_string();
                if field_type != "Optional" {
                    new_field.ty = syn::parse2(quote! { Optional<#field_type> }).unwrap();
                }
            }
        }
        new_field
    }).collect();
    new_struct.fields = Fields::Named(FieldsNamed {
        brace_token: token::Brace::default(),
        named: fields
    });
    new_struct
}