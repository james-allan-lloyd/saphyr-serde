use std::collections::HashMap;

use serde::Serialize;

use crate::ser::to_string;

#[test]
fn test_struct() {
    #[derive(Serialize)]
    struct Test {
        int: u32,
        seq: Vec<&'static str>,
    }

    let test = Test {
        int: 1,
        seq: vec!["a", "b"],
    };
    let expected = r###"int: 1
seq:
  - a
  - b
"###;
    assert_eq!(to_string(&test).unwrap(), expected);
}

#[test]
fn test_enum() {
    #[derive(Serialize)]
    enum E {
        Unit,
        Newtype(u32),
        Tuple(u32, u32),
        Struct { a: u32 },
    }

    let u = E::Unit;
    let expected = "Unit\n";
    assert_eq!(to_string(&u).unwrap(), expected);

    let n = E::Newtype(1);
    let expected = "Newtype: 1\n";
    assert_eq!(to_string(&n).unwrap(), expected);

    let t = E::Tuple(1, 2);
    let expected = r###"Tuple:
  - 1
  - 2
"###;
    assert_eq!(to_string(&t).unwrap(), expected);

    let s = E::Struct { a: 1 };
    let expected = r#"Struct:
  a: 1
"#;
    assert_eq!(to_string(&s).unwrap(), expected);
}

#[test]
fn test_outdenting() {
    #[derive(Serialize)]
    enum E {
        Unit,
        Newtype(u32),
        Tuple(u32, u32),
        Struct { a: u32 },
    }

    let u = E::Unit;
    let n = E::Newtype(1);
    let t = E::Tuple(1, 2);
    let s = E::Struct { a: 1 };

    let v = vec![
        u,
        n,
        t,
        s,
        E::Newtype(2),
        E::Tuple(3, 4),
        E::Struct { a: 2 },
    ];

    let expected = r#"- Unit
- Newtype: 1
- Tuple:
    - 1
    - 2
- Struct:
    a: 1
- Newtype: 2
- Tuple:
    - 3
    - 4
- Struct:
    a: 2
"#;
    assert_eq!(to_string(&v).unwrap(), expected);
}

#[test]
fn it_writes_multiline_strings() {
    todo!();
}

#[test]
fn it_serializes_other_types() {
    #[derive(Debug, PartialEq, Serialize)]
    pub struct NewTypeStruct(u32);

    #[derive(Serialize, PartialEq, Eq, Debug)]
    struct Address {
        street: String,
        state: String,
    }

    #[derive(Serialize, Debug)]
    struct S {
        b: bool,
        o_none: Option<String>,
        o_some: Option<String>,
        nested: Address,
        sbyte: i8,
        ubyte: u8,
        sshort: i16,
        ushort: u16,
        slong: i32,
        ulong: u32,
        slonglong: i64,
        ulonglong: u64,
        tuple: (i32, String),
        newtype: NewTypeStruct,
        map: HashMap<String, String>,
    }

    let mut map = HashMap::new();
    map.insert(String::from("foo"), String::from("bar"));
    map.insert(String::from("baz"), String::from("duh"));

    let s = S {
        b: true,
        o_none: None,
        o_some: Some(String::from("Some string")),
        nested: Address {
            street: String::from("Main Street"),
            state: String::from("New Jersey"),
        },
        sbyte: -1,
        ubyte: 2,
        sshort: -3,
        ushort: 4,
        slong: -5,
        ulong: 6,
        slonglong: -7,
        ulonglong: 8,
        tuple: (9, String::from("that's a tuple")),
        newtype: NewTypeStruct(10),
        map,
    };

    let expected = r#"b: true
o_none: null
o_some: Some string
nested:
  street: Main Street
  state: New Jersey
sbyte: -1
ubyte: 2
sshort: -3
ushort: 4
slong: -5
ulong: 6
slonglong: -7
ulonglong: 8
tuple:
  - 9
  - that's a tuple
newtype: 10
map:
  foo: bar
  baz: duh
"#;

    assert_eq!(to_string(&s).unwrap(), expected);
}
