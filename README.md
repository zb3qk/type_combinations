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

## When use this library
You should use `compose_type!` if:
    1. Your data model is utilized in multiple decoupled implementations
    2. Can be generated using clear patterns, coupled to existing data models
    
### Multiple decoupled implementations
Let's say you are building a banking application with different account types

```rust
struct BankAccount {
    balance: i32
}
```

to define the other account types, you have 2 choices: (1) define the `BankAccount` structure defined above as a field of the new account type or (2) use our library to inject those fields into the new account type.

```rust
// (1)
struct CheckingAccount {
    bankAccount: BankAccount
}

// (2)
compose_type! {
    struct BankAccount {
        balance: i32
    }
    struct SavingsAcccount = BankAccount;
}

// (2) Result
struct SavingsAccount {
    balance: i32
}
```

Whether we choose (1) or (2) is based on the specification of our banking app. Let's add a `Transaction` trait. 

```
trait Transaction {
    fn transact(&mut self, other: dyn Transaction);
}
```

If a transaction is implemented differently between `CheckingAccount` and  `SavingsAccount`, using `compose_type!` makes sense since implementation is decoupled from the pattern of data and is instead use case specific.

```
impl CheckingAccount for Transaction {
    fn transact(&mut self, other: dyn Transaction) {
        todo!("Implementation 1")
    }
}


impl SavingsAccount for Transaction {
    fn transact(&mut self, other: dyn Transaction) {
        todo!("Implementation 2")
    }
}
```

Otherwise, if transaction implementations are consistent across Account types, then implementing `transact` on a single type would make more sense:
```
impl BankAccount for Transaction {
    fn transact(&mut self, other: dyn Transaction) {
        todo!("Implementation 1")
    }
}
```

### Coupling with existing data models

Let us say you are creating an API which is intedned to support multiple styles of request: GRPC and JsonRPC. GRPC can guarentee whether a field exists in a given Response because of its encoding format whereas Json cannot. In Json, any field we can expect can either exist, or it cannot but ultimately the expected Response object is the same.

```
struct Response {}
```

We can easily generate a GRPC specific response and a JsonRPC response using `compose_type!`

```
compose_type! {
    struct GrpcResponse = Response;
    struct JsonResponse = Optional(Response);
}
```

Since each field in `Response` is potentially optional for Json, the pattern is clear where each individual field should be wrapped with `Option`. Since the data model for both `GrpcResponse` and `JsonResponse` are logically coupled with the definition of `Response`, `compose_type!` fits this use case quite well.
