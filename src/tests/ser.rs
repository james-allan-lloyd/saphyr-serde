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
