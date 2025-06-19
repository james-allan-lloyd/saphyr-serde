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
