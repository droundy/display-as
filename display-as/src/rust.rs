//! Format as rust code

use super::*;

/// Format as Rust.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Rust;
impl Format for Rust {
    fn escape(f: &mut Formatter, s: &str) -> Result<(), Error> {
        (&s as &dyn std::fmt::Debug).fmt(f)
    }
    fn mime() -> mime::Mime {
        return "text/x-rust".parse().unwrap();
    }
    fn this_format() -> Self {
        Rust
    }
}

display_integers_as!(Rust);
display_floats_as!(Rust, "e", "", 1, None);

#[test]
fn escaping() {
    assert_eq!(&format_as!(Rust, ("&")).into_string(), r#""&""#);
}
#[test]
fn floats() {
    assert_eq!(&format_as!(Rust, 3.0).into_string(), "3");
    assert_eq!(&format_as!(Rust, 3e5).into_string(), "3e5");
    assert_eq!(&format_as!(Rust, 3e4).into_string(), "3e4");
    assert_eq!(&format_as!(Rust, 3e3).into_string(), "3e3");
    assert_eq!(&format_as!(Rust, 3e2).into_string(), "300");
}
