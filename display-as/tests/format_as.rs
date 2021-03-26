extern crate display_as;

use display_as::{format_as, with_template, DisplayAs, Rust, HTML};

#[test]
fn just_string() {
    assert_eq!(
        format_as!(HTML, r"Hello world").into_string(),
        r"Hello world"
    );
}
#[test]
fn string_and_integer() {
    assert_eq!(
        format_as!(HTML, r"Number " 3 r" is odd").into_string(),
        r"Number 3 is odd"
    );
    assert_eq!(
        format_as!(HTML, r"Number " 2+1 r" is odd").into_string(),
        r"Number 3 is odd"
    );
    assert_eq!(
        format_as!(HTML, r"Number " 4-1 r" is odd").into_string(),
        r"Number 3 is odd"
    );
    assert_eq!(
        format_as!(HTML, r"Number " 8/2-1 r" is odd").into_string(),
        r"Number 3 is odd"
    );
    assert_eq!(
        format_as!(HTML, r"Number " 1*3 r" is odd").into_string(),
        r"Number 3 is odd"
    );
}
#[test]
fn integer_reference() {
    assert_eq!(
        format_as!(HTML, r"Number " &3u64 r" is odd").into_string(),
        r"Number 3 is odd"
    );
    assert_eq!(
        format_as!(HTML, r"Number " 3u64 r" is odd").into_string(),
        r"Number 3 is odd"
    );
    assert_eq!(
        format_as!(HTML, r"Number " &&3u64 r" is odd").into_string(),
        r"Number 3 is odd"
    );
}
#[test]
fn string_and_float() {
    assert_eq!(
        format_as!(HTML, r"Number " 3.0 r" is odd").into_string(),
        r"Number 3 is odd"
    );
    assert_eq!(
        format_as!(Rust, r"Number " 1e10 r" is even").into_string(),
        r"Number 1e10 is even"
    );
    assert_eq!(
        format_as!(Rust, r"Number " 1e2 r" is even").into_string(),
        r"Number 100 is even"
    );
    assert_eq!(
        format_as!(Rust, r"Number " 1.2345e2 r" is even").into_string(),
        r"Number 123.45 is even"
    );
}
#[test]
fn test_conditionals() {
    let foo = 3.0;
    let bar = 2.0;
    assert_eq!(
        format_as!(HTML, r"Game: " if foo > bar { r"foo wins" }).into_string(),
        r"Game: foo wins"
    );
    assert_eq!(
        format_as!(HTML, r"Counting: " for i in 0..5 { i " " }).into_string(),
        r"Counting: 0 1 2 3 4 "
    );
}

#[test]
fn test_mixed_formats() {
    let foo = 3e6;
    assert_eq!(
        format_as!(HTML, r"Number: " foo r" and " foo as Rust r"!").into_string(),
        r"Number: 3×10<sup>6</sup> and 3e6!"
    );
}

#[test]
fn test_let() {
    assert_eq!(
        format_as!(HTML, let foo = {
        for i in 0..3 {
            "counting " i " "
        }
    };
                                  foo).into_string(),
        r"counting 0 counting 1 counting 2 "
    );
}
#[test]
fn test_let_again() {
    struct Foo(isize);
    #[with_template("Foo " self.0)]
    impl DisplayAs<HTML> for Foo {}
    let foos = vec![Foo(1), Foo(2)];
    assert_eq!(
        format_as!(HTML,
                          let foo = {
                              "I am "
                                  for i in foos.iter() {
                                      "counting " i " "
                                  }
                          };
                          foo "and I am done!").into_string(),
        r"I am counting Foo 1 counting Foo 2 and I am done!"
    );
}

#[test]
fn nested_format_as() {
    struct Foo(isize);
    #[with_template("Foo " self.0)]
    impl DisplayAs<HTML> for Foo {}
    format_as!(HTML, "testing" format_as!(HTML, "hello" Foo(2)) " and " Foo(1));
}