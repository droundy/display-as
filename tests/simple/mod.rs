#[macro_use]
extern crate display_as_templates;
extern crate display_as;

use display_as::{HTML, As};

struct Simple;

display_as_template!(HTML; impl DisplayAs<HTML> for Simple {}; "template_html.rs");

#[test]
fn use_simple_template() {
    // assert_eq!(&format!("{}", As(HTML, Simple)), "hello world 30000000000\nThis is cool.\n");
}
