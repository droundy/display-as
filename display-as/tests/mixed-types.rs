use display_as::{format_as, URL, HTML};

#[test]
fn mixed_types() {
    assert_eq!(&format_as!(HTML, "hello world " 5 as URL " urls are " 5.0).into_string(),
               "hello world 5 urls are 5");
}

#[test]
fn mixed_formats_in_let() {
    assert_eq!(&format_as!(HTML,
                           let foo = {
                               "hello world " 5 as URL " urls are " 5.0
                           };
                           foo).into_string(),
               "hello world 5 urls are 5");
}
