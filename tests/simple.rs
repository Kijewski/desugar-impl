use std::fmt::{Debug, Result, Write};

use desugar_impl::desugar_impl;

#[desugar_impl]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct ExampleStruct {
    a: impl Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
    b: impl Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
    c: impl Clone + Debug + PartialEq + PartialOrd + Eq + Ord,
}

#[test]
fn compile_test() -> Result {
    let a = ExampleStruct {
        a: &1,
        b: "hello",
        c: true,
    };
    let b = ExampleStruct {
        a: &3,
        b: "world",
        c: false,
    };
    let c = ExampleStruct {
        a: &1,
        b: ":)",
        c: true,
    };

    let mut dbg_str = String::new();
    write!(dbg_str, "{:?}", &a)?;
    assert_eq!(dbg_str, "ExampleStruct { a: 1, b: \"hello\", c: true }");

    let mut dbg_str = String::new();
    write!(dbg_str, "{:?}", &b)?;
    assert_eq!(dbg_str, "ExampleStruct { a: 3, b: \"world\", c: false }");

    let mut dbg_str = String::new();
    write!(dbg_str, "{:?}", &c)?;
    assert_eq!(dbg_str, "ExampleStruct { a: 1, b: \":)\", c: true }");

    let mut abc = vec![a.clone(), b.clone(), c.clone()];
    abc.sort();
    assert_eq!(abc, vec![c, a, b]);

    Ok(())
}
