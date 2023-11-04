# composite_types

Introducing type definitions in [the style of Typescript](https://www.typescriptlang.org/docs/handbook/2/types-from-types.html) to Rust!
```rust
use proc_macro_def::{compose_type};
struct FieldType {}
mod module { 
    compose_type! {
        struct Example {
            field: Option<FieldType>
        }
        struct MyStruct = Required(Example);
        struct MyStruct2 = Optional(MyStruct);
        pub struct MyStruct3 = Required(MyStruct);
    }

    const EXAMPLE: MyStruct = MyStruct2 {
        field: None
    };
}

struct Example2 {
    field: Option<MyStruct3>
}
```

## Usage
1. Use the `compose_type!` macro
2. Import and define any types outside `compose_type!` as you normally would
3. You can reference these types within `compose_type!`, but cannot compose with them
4. Define structs within `compose_type!` to be used to compose new types
5. Reference your new types outside of `compose_type!` and use them in your project