//! A template may change the variables it uses, as the ones in `write_as!` can.

use display_as::{format_as, write_as, HTML};

#[test]
fn format_as_advances_an_iterator() {
    let mut it = ["a", "b", "c"].iter();
    assert_eq!(
        format_as!(HTML, while let Some(x) = it.next() { x "," }).into_string(),
        "a,b,c,"
    );
    assert_eq!(it.next(), None);
}

#[test]
fn format_as_counts() {
    let mut count = 0;
    assert_eq!(
        format_as!(HTML, for i in 0..3u8 { i "," count += 1; }).into_string(),
        "0,1,2,"
    );
    assert_eq!(count, 3);
}

#[test]
fn write_as_advances_an_iterator() {
    let mut it = ["a", "b"].iter();
    let mut s = String::new();
    write_as!(HTML, s, while let Some(x) = it.next() { x "," }).unwrap();
    assert_eq!(s, "a,b,");
}
