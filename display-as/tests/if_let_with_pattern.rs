extern crate display_as;

use display_as::{format_as, HTML};

#[test]
fn test_if_let() {
    struct Foo { x: usize }
    let foo = Foo { x: 37 };
    assert_eq!(
        format_as!(HTML, if let Foo { x } = foo {
            x
        }).into_string(),
        r"37"
    );
}

#[test]
fn test_let_with_braces_match() {
    struct Foo { x: usize }
    let foo = Foo { x: 37 };
    assert_eq!(
        format_as!(HTML, {
            let Foo { x } = foo;
            x
        }).into_string(),
        r"37"
    );
}
