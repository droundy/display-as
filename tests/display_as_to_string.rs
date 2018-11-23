extern crate display_as_templates;

use display_as_templates::{display_as_to_string, HTML, Rust};

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
#[test]
fn string_and_float() {
    assert_eq!(display_as_to_string!(HTML, r"Number " 3.0 r" is odd"), r"Number 3 is odd");
    assert_eq!(display_as_to_string!(Rust, r"Number " 1e10 r" is even"),
               r"Number 1e10 is even");
    assert_eq!(display_as_to_string!(Rust, r"Number " 1e2 r" is even"),
               r"Number 100 is even");
    assert_eq!(display_as_to_string!(Rust, r"Number " 1.2345e2 r" is even"),
               r"Number 123.45 is even");
}
