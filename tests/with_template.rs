extern crate display_as_template;

use display_as_template::{with_template, DisplayAs, As, HTML};

struct Foo {
    name: String,
    age: usize,
}

#[with_template("Foo: " &self.name " with age " self.age)]
impl DisplayAs<HTML> for Foo {}

#[test]
fn foo() {
    assert_eq!(&format!("{}", As(HTML, Foo { name: "David".to_string(),
                                             age: 45 })),
               "Foo: David with age 45");
}
