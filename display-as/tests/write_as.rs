extern crate display_as;

use display_as::{write_as, HTML};

#[test]
fn write_to_string() {
    let mut s = String::new();
    write_as!(HTML, s, r"Hello world").unwrap();
    assert_eq!(&s, r"Hello world");
}

#[test]
fn write_to_mut_ref_string() {
    let mut s = String::new();
    write_as!(HTML, &mut s, r"Hello world").unwrap();
    assert_eq!(&s, r"Hello world");
}

#[test]
fn write_integer() {
    let mut s = String::new();
    write_as!(HTML, s, 137).unwrap();
    assert_eq!(&s, r"137");
}

#[test]
fn write_nice_loop() {
    let data = ["hello", "world"];
    let mut s = String::new();
    write_as!(HTML, s, for d in data.iter() {
        " " d
    })
    .unwrap();
    assert_eq!(&s, r" hello world");
}

#[test]
fn write_nice_loop_strings() {
    let data = ["hello".to_string(), "world".to_string()];
    let mut s = String::new();
    write_as!(HTML, s, for d in data.iter() {
        " " d
    })
    .unwrap();
    assert_eq!(&s, r" hello world");
}
