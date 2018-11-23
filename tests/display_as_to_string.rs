extern crate display_as_templates;

use display_as_templates::{display_as_to_string, HTML};

#[test]
fn just_string() {
    assert_eq!(display_as_to_string!(HTML, r"Hello world"), r"Hello world");
}
#[test]
fn string_and_integer() {
    assert_eq!(display_as_to_string!(HTML, r"Number " 3 r" is odd"), r"Number 3 is odd");
    assert_eq!(display_as_to_string!(HTML, r"Number " 2+1 r" is odd"), r"Number 3 is odd");
    assert_eq!(display_as_to_string!(HTML, r"Number " 4-1 r" is odd"), r"Number 3 is odd");
    assert_eq!(display_as_to_string!(HTML, r"Number " 8/2-1 r" is odd"), r"Number 3 is odd");
    assert_eq!(display_as_to_string!(HTML, r"Number " 1*3 r" is odd"), r"Number 3 is odd");
}
