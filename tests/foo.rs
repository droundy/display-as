#[macro_use]
extern crate display_as_templates;
extern crate display_as;

use display_as::{HTML, As};

struct Foo;

display_as_with_template!(HTML, Foo, "foo.html");

#[test]
fn use_foo_template() {
    assert_eq!(&format!("{}", As(HTML, Foo)), "hello world 30000000000\nThis is cool.\n");
}
