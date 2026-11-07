//! A `Format` defined outside of display-as can use display-as's macros to
//! support integers and floats.

use display_as::{display_floats_as, display_integers_as, format_as, Format};
use std::fmt::{Error, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Shout;
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
display_floats_as!(Shout, "e", "", 1, None);

#[test]
fn integers() {
    assert_eq!(
        &format_as!(Shout, 255u8 " " -12i64 " " 7usize " " 1u128 " " 3i8).into_string(),
        "255 -12 7 1 3"
    );
}

#[test]
fn floats() {
    assert_eq!(&format_as!(Shout, 2.5 " " 1e9f32).into_string(), "2.5 1e9");
}

#[test]
fn strings_are_escaped() {
    assert_eq!(&format_as!(Shout, ("hi") " " 3u8).into_string(), "HI 3");
}
