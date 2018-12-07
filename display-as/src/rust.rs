//! Format as rust code

use super::*;

/// Format as Rust.
pub struct Rust;
impl Format for Rust {
    fn escape(f: &mut Formatter, s: &str) -> Result<(), Error> {
        (&s as &std::fmt::Debug).fmt(f)
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
    assert_eq!(&format!("{}", As(Rust, "&")), r#""&""#);
}
#[test]
fn floats() {
    assert_eq!(&format!("{}", As(Rust, 3.0)), "3");
    assert_eq!(&format!("{}", As(Rust, 3e5)), "3e5");
    assert_eq!(&format!("{}", As(Rust, 3e4)), "3e4");
    assert_eq!(&format!("{}", As(Rust, 3e3)), "3e3");
    assert_eq!(&format!("{}", As(Rust, 3e2)), "300");
}
