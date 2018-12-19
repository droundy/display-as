//! Format as rust code

use super::*;

/// Format as raw UTF8.
///
/// This is one way to output a raw string.
pub struct UTF8;
impl Format for UTF8 {
    fn escape(f: &mut Formatter, s: &str) -> Result<(), Error> {
        f.write_str(s)
    }
    fn mime() -> mime::Mime {
        return mime::TEXT_PLAIN_UTF_8;
    }
    fn this_format() -> Self {
        UTF8
    }
}

display_integers_as!(UTF8);
display_floats_as!(UTF8, "e", "", 1, None);

#[test]
fn escaping() {
    assert_eq!(&format!("{}", "&".display_as(UTF8)), "&");
}
#[test]
fn floats() {
    assert_eq!(&format!("{}", 3.0.display_as(UTF8)), "3");
    assert_eq!(&format!("{}", 3e5.display_as(UTF8)), "3e5");
    assert_eq!(&format!("{}", 3e4.display_as(UTF8)), "3e4");
    assert_eq!(&format!("{}", 3e3.display_as(UTF8)), "3e3");
    assert_eq!(&format!("{}", 3e2.display_as(UTF8)), "300");
}
