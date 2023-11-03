use proc_macro_def::{compose_type};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

struct FieldType {}

compose_type! {
    struct Example {
        field: Option<FieldType>
    }
    struct MyStruct = Required(Example);
    struct MyStruct2 = Optional(MyStruct);
    struct MyStruct3 = Required(MyStruct);
}

const EXAMPLE: MyStruct = MyStruct {
    field: FieldType {}
};

struct Example2 {
    field: Option<MyStruct>
}
