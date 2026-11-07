//! The format given to the macros may be a path, such as `display_as::HTML`,
//! not just a single identifier.

use display_as::{display_integers_as, format_as, with_template, write_as, DisplayAs, Format};
use std::fmt::{Error, Formatter};

mod formats {
    use super::*;

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Shout;
    impl Format for Shout {
        fn escape(f: &mut Formatter, s: &str) -> Result<(), Error> {
            f.write_str(&s.to_uppercase())
        }
        fn mime() -> mime::Mime {
            mime::TEXT_PLAIN_UTF_8
        }
        fn this_format() -> Self {
            Shout
        }
    }
    display_integers_as!(Shout);
}

struct Foo(isize);

#[with_template("Foo " self.0)]
impl DisplayAs<display_as::HTML> for Foo {}

#[with_template("foo " self.0)]
impl DisplayAs<self::formats::Shout> for Foo {}

#[with_template("Bar " (self.0.to_string() + "<>") as display_as::HTML)]
impl DisplayAs<crate::formats::Shout> for Bar {}
struct Bar(isize);

#[test]
fn format_as_with_path() {
    assert_eq!(
        format_as!(display_as::HTML, "x<" 1 ">" ("<")).into_string(),
        "x<1>&lt;"
    );
    assert_eq!(
        format_as!(formats::Shout, "x " 1 " " ("y")).into_string(),
        "x 1 Y"
    );
}

#[test]
fn format_as_with_path_is_a_formatted_string_of_that_format() {
    let s: display_as::FormattedString<display_as::HTML> = format_as!(display_as::HTML, "hi");
    assert_eq!(s.into_string(), "hi");
}

#[test]
fn format_as_with_leading_colons() {
    assert_eq!(
        format_as!(::display_as::HTML, "x" ("<")).into_string(),
        "x&lt;"
    );
}

#[test]
fn write_as_with_path() {
    let mut s = String::new();
    write_as!(display_as::HTML, s, "x<" 1 ">" ("<")).unwrap();
    assert_eq!(&s, "x<1>&lt;");
    let mut s = String::new();
    write_as!(formats::Shout, &mut s, "x " 1 " " ("y")).unwrap();
    assert_eq!(&s, "x 1 Y");
}

#[test]
fn with_template_with_path() {
    assert_eq!(format_as!(display_as::HTML, Foo(3)).into_string(), "Foo 3");
    assert_eq!(format_as!(formats::Shout, Foo(3)).into_string(), "foo 3");
}

#[test]
fn with_template_with_path_embedding_another_format() {
    assert_eq!(
        format_as!(formats::Shout, Bar(3)).into_string(),
        "Bar 3&lt;&gt;"
    );
}

#[test]
fn let_template_with_path() {
    assert_eq!(
        format_as!(formats::Shout, let x = { "a" ("b") }; x " " x).into_string(),
        "aB aB"
    );
}

#[test]
fn nested_with_the_same_path() {
    assert_eq!(
        format_as!(
            display_as::HTML,
            "a " format_as!(display_as::HTML, "b " format_as!(display_as::HTML, "c " Foo(3)) " " Foo(2)) " " Foo(1)
        )
        .into_string(),
        "a b c Foo 3 Foo 2 Foo 1"
    );
}

#[test]
fn nested_with_different_paths() {
    assert_eq!(
        format_as!(
            formats::Shout,
            "a " format_as!(display_as::HTML, "b<" Foo(2)) as display_as::HTML " " Foo(1)
        )
        .into_string(),
        "a b<Foo 2 foo 1"
    );
}
