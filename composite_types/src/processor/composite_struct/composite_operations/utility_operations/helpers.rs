use proc_macro2::Ident;
use syn::{AngleBracketedGenericArguments, Field, GenericArgument, ItemStruct, PathArguments, PathSegment, Result, Type};

extern crate proc_macro2;

pub fn generate_not_found_error(ident: &Ident) -> Result<()>{
    return Err(syn::Error::new(ident.span(), format!("Required type {} does not exist", ident.to_string())))
}

pub fn get_generics(field_type: &Type) -> Option<AngleBracketedGenericArguments> {
    if let syn::Type::Path(path) = field_type {
        if let PathArguments::AngleBracketed(arguments) = path.path.segments.first()?.to_owned().arguments {
            return Some(arguments)
        }
    }
    return None
}

fn is_generic_type_arg(arg: &GenericArgument) -> bool {
    return if let syn::GenericArgument::Type(syn::Type::Path(type_path)) = arg {
        true
    } else {
        false
    }
}

fn update_field_type(mut field: Field, top_level_type: Ident, generic_args: AngleBracketedGenericArguments) -> Field {
    match &mut field.ty {
        syn::Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last_mut() {
                segment.ident = top_level_type;
                segment.arguments = PathArguments::AngleBracketed(generic_args);
            }
        }
        _ => {}
    }
    field
}

/// Wraps the current top level type as a generic argument, and sets the new field type
/// as the passed in top level type. For example, if the field type is `FieldType<NestedFieldType, ...>`
/// and the top level type is `NewFieldType`, the new field type will be `NewFieldType<FieldType<NestedFieldType, ...>>`
///
/// # Arguments
///
/// * `field`: Field that is being operated on
/// * `top_level_type`: New type which will process
///
/// returns: Option<Field> If None, the operation failed to complete.

pub fn wrap_field_as_generic_arg(field: Field, top_level_type: Ident) -> Option<Field> {
    let mut field = field.clone();
    let new_generic_args = syn::punctuated::Punctuated::from_iter(vec![GenericArgument::Type(field.ty.clone())]);
    let new_generic_args = AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: syn::token::Lt::default(),
        args: new_generic_args,
        gt_token: syn::token::Gt::default()
    };
    field.ty = Type::Path(syn::TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: None,
            segments: syn::punctuated::Punctuated::from_iter(vec![PathSegment {
                ident: top_level_type,
                arguments: PathArguments::AngleBracketed(new_generic_args)
            }])
        }
    });
    Some(field)
}

pub fn assign_struct_name(mut structure: ItemStruct, new_name: Ident) -> ItemStruct {
    structure.ident = new_name;
    structure
}

pub fn get_first_generic_type_arg(field: &Field) -> Option<Type> {
    let field_type = &field.ty;
    let arguments = get_generics(field_type)?;
    // Search for first valid generic argument
    let search_result =  arguments.args.iter().enumerate().find(|(i, arg)| {
        return is_generic_type_arg(arg)
    });
    let (position, first_generic_type_arg) = search_result?;
    let first_generic_type_arg = match first_generic_type_arg {
        syn::GenericArgument::Type(syn::Type::Path(type_path)) => type_path.to_owned(),
        _ => return None
    };
    Some(syn::Type::Path(first_generic_type_arg))
}

/// Promotes a generic argument to the top level of the type, overriding the existing
/// top level type. This is useful for converting a type like `FieldType<NestedFieldType, ...>`
/// to `NestedFieldType<...>`. This removes any other generic arguments from the type.
///
/// # Arguments
///
/// * `field`: A field from an ItemStruct with GenericArguments.
///
/// returns: Option<Field> If None, the operation failed to complete.
pub fn promote_first_generic_argument(field: Field) -> Option<Field> {
    let mut field = field.clone();
    let field_type = &field.ty;
    let mut arguments = get_generics(field_type)?;

    // Search for first valid generic argument
    let search_result =  arguments.args.iter().enumerate().find(|(i, arg)| {
        return is_generic_type_arg(arg)
    });
    let (position, first_generic_argument) = search_result?;
    let first_generic_argument = match first_generic_argument {
        syn::GenericArgument::Type(syn::Type::Path(type_path)) => type_path.to_owned(),
        _ => return None
    };
    field.ty = syn::Type::Path(first_generic_argument);
    Some(field)
}

// write unit tests
#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use super::*;
    use syn::{ItemStruct, parse_quote};

    #[test]
    fn test_promote_first_generic_argument() {
        let input: ItemStruct = parse_quote! {
            struct Test {
               field: FieldType<NestedFieldType>
            }
        };
        let input_field = input.fields.iter().next().unwrap();

        let expected: ItemStruct = parse_quote! {
            struct Test {
               field: NestedFieldType
            }
        };
        let expected_field = expected.fields.iter().next().unwrap();

        let promoted_field = promote_first_generic_argument(input_field.to_owned()).unwrap();
        assert_eq!(
            promoted_field.ty.to_token_stream().to_string(),
            expected_field.ty.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_promote_first_generic_argument_complicated() {
        let input: ItemStruct = parse_quote! {
            struct Test {
               field: FieldType<'a, NestedFieldType<Booper, Dooper::Looper>, AnotherField>
            }
        };
        let input_field = input.fields.iter().next().unwrap();

        let expected: ItemStruct = parse_quote! {
            struct Test {
               field: NestedFieldType<Booper, Dooper::Looper>
            }
        };
        let expected_field = expected.fields.iter().next().unwrap();
        let promoted_field = promote_first_generic_argument(input_field.to_owned()).unwrap();
        assert_eq!(
            promoted_field.ty.to_token_stream().to_string(),
            expected_field.ty.to_token_stream().to_string());
    }

    #[test]
    fn test_demote_field_type_and_wrap_complicated() {
        let input: ItemStruct = parse_quote! {
            struct Test {
               field: FieldType<'a, NestedFieldType, Goober::Nested>
            }
        };
        let input_field = input.fields.iter().next().unwrap();

        let new_field_type: Ident = parse_quote!(NewFieldType);
        let expected: ItemStruct = parse_quote! {
            struct Test {
               field: NewFieldType<FieldType<'a, NestedFieldType, Goober::Nested>>
            }
        };
        let expected_field = expected.fields.iter().next().unwrap();

        let promoted_field = wrap_field_as_generic_arg(input_field.to_owned(), new_field_type).unwrap();
        assert_eq!(
            promoted_field.ty.to_token_stream().to_string(),
            expected_field.ty.to_token_stream().to_string());
    }

    #[test]
    fn test_demote_field_type_and_wrap() {
        let input: ItemStruct = parse_quote! {
            struct Test {
               field: FieldType<NestedFieldType>
            }
        };
        let input_field = input.fields.iter().next().unwrap();

        let new_field_type: Ident = parse_quote!(NewFieldType);
        let expected: ItemStruct = parse_quote! {
            struct Test {
               field: NewFieldType<FieldType<NestedFieldType>>
            }
        };
        let expected_field = expected.fields.iter().next().unwrap();

        let promoted_field = wrap_field_as_generic_arg(input_field.to_owned(), new_field_type).unwrap();
        assert_eq!(
            promoted_field.ty.to_token_stream().to_string(),
            expected_field.ty.to_token_stream().to_string());
    }
}