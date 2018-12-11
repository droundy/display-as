use display_as::{format_as, DisplayAs, URL, HTML};

#[test]
fn mixed_types() {
    assert_eq!(&format_as!(HTML, "hello world " 5 as URL " urls are " 5.0),
               "hello world 5 urls are 5");
}
