extern crate display_as_template;

use display_as_template::{display_as_to_string, HTML, Rust};

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
#[test]
fn test_conditionals() {
    let foo = 3.0;
    let bar = 2.0;
    assert_eq!(display_as_to_string!(HTML, r"Game: " if foo > bar { r"foo wins" }),
               r"Game: foo wins");
    assert_eq!(display_as_to_string!(HTML, r"Counting: " for i in 0..5 { i " " }),
               r"Counting: 0 1 2 3 4 ");
}

#[test]
fn test_mixed_formats() {
    let foo = 3e6;
    assert_eq!(display_as_to_string!(HTML, r"Number: " foo r" and " foo as Rust r"!"),
               r"Number: 3×10<sup>6</sup> and 3e6!");
}
