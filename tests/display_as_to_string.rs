extern crate display_as_templates;

use display_as_templates::{display_as_to_string, HTML};

#[test]
fn just_string() {
    assert_eq!(display_as_to_string!(HTML, r"Hello world"), r"Hello world");
}
