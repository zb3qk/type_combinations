mod parser;
mod processor;
mod macro_impl;

use quote::quote;
use proc_macro::TokenStream;
use crate::macro_impl::composite_type_impl;
use crate::parser::type_input::TypeInput;
use crate::processor::process_input;

/// # compose_type!
/// Compose types from other types in Rust similar to [Typescript](https://www.typescriptlang.org/docs/handbook/2/types-from-types.html)!
/// Here is a quick example!
/// ```rust
/// # use proc_macro_def::{compose_type};
/// struct FieldType {}
///
/// compose_type! {
///
///     struct Example {
///         field: Option<FieldType>
///     }
///     struct MyStruct = Required(Example);
///     struct MyStruct2 = Optional(MyStruct);
///     pub struct MyStruct3 = Required(MyStruct);
/// }
///
/// struct Example2 {
///     field: Option<MyStruct>
/// }
///
/// ```
/// Here are a few points from the above example:
/// 1. You can reference types defined outside of `compose_type` like `FieldType`.
/// 2. You can reference types defined inside of `compose_type` like `MyStruct` outside of the macro
/// invocation.
/// 3. You can expose types outside of the current module, defined inside of `compose_type` by using
/// the `pub` keyword.
///
/// You can find the full feature set below!
///
/// ## Type Alias
/// You can rename types within the scope of the macro invocation. This can be useful when you
/// want to rename the type based on context.
/// ```rust
/// # use proc_macro_def::{compose_type};
/// compose_type! {
///
///     struct Example {
///         field: Option<FieldType>
///     }
///     pub struct MyStruct = Example;
/// }
///
/// const EXAMPLE: MyStruct = MyStruct {
///    field: None
/// };
/// ```
///
/// ## Type Functions
/// ### Optional
/// You can wrap a type in `Option` by using the `Optional` type function.
/// ```rust
/// # use proc_macro_def::{compose_type};
/// struct FieldType {}
/// compose_type! {
///    struct Example {
///       field: FieldType
///    }
///
///    struct MyStruct = Optional(Example);
/// }
///
/// const EXAMPLE: MyStruct = MyStruct {
///   field: None
/// };
///
/// ```
/// ### Required
/// You can unwrap a type from `Option` by using the `Required` type function.
/// ```rust
/// # use proc_macro_def::{compose_type};
/// struct FieldType {}
/// compose_type! {
///   struct Example {
///      pub field: Option<FieldType>
///   }
///  struct MyStruct = Required(Example);
/// }
///
/// const EXAMPLE: MyStruct = MyStruct {
///    field: FieldType {}
/// };
/// ```
#[proc_macro]
pub fn compose_type(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let output = composite_type_impl(input);
    proc_macro::TokenStream::from(output)
}

