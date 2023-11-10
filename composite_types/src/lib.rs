//!  # compose_type!
//! Compose types from other types in Rust similar to
//! Here is a quick example!
//! ```rust
//! # use composite_types::{compose_type};
//! struct FieldType {}
//! compose_type! {
//!     struct Example {
//!         field: Option<FieldType>
//!     }
//!     struct MyStruct = Required(Example);
//!     struct MyStruct2 = Optional(MyStruct);
//!     pub struct MyStruct3 = Required(MyStruct);
//! }
//! struct Example2 {
//!     field: Option<MyStruct>
//! }
//! ```
//! Here are a few points from the above example:
//! 1. You can reference types defined outside of `co
//! 2. You can reference types defined inside of `com
//! invocation.
//! 3. You can expose types outside of the current mo
//! the `pub` keyword.
//! You can find the full feature set below!
//! ## Type Alias
//! You can rename types within the scope of the macr
//! want to rename the type based on context.
//! ```rust
//! # use composite_types::{compose_type};
//! struct FieldType {}
//! compose_type! {
//!     struct Example {
//!         field: Option<FieldType>
//!     }
//!     pub struct MyStruct = Example;
//! }
//! const EXAMPLE: MyStruct = MyStruct {
//!    field: None
//! };
//! ```
//! ## Type Functions
//! ### Optional
//! You can wrap a type in `Option` by using the `Opt
//! ```rust
//! # use composite_types::{compose_type};
//! struct FieldType {}
//! compose_type! {
//!    struct Example {
//!       field: FieldType
//!    }
//!    struct MyStruct = Optional(Example);
//! }
//! const EXAMPLE: MyStruct = MyStruct {
//!   field: None
//! };
//! ```
//! ### Required
//! You can unwrap a type from `Option` by using the
//! ```rust
//! # use composite_types::{compose_type};
//! struct FieldType {}
//! compose_type! {
//!   struct Example {
//!      pub field: Option<FieldType>
//!   }
//!  struct MyStruct = Required(Example);
//! }
//! const EXAMPLE: MyStruct = MyStruct {
//!    field: FieldType {}
//! };
//! ```
//!
mod parser;
mod processor;
mod macro_impl;

use proc_macro::TokenStream;
use crate::macro_impl::composite_type_impl;

#[proc_macro]
pub fn compose_type(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let output = composite_type_impl(input);
    proc_macro::TokenStream::from(output)
}

