//! Format as HTML

use super::*;

/// Format as HTML.
pub struct HTML;
impl Format for HTML {
    fn escape(f: &mut Formatter, mut s: &str) -> Result<(), Error> {
        let badstuff = "<>&\"'/";
        while let Some(idx) = s.find(|c| badstuff.contains(c)) {
            let (first, rest) = s.split_at(idx);
            let (badchar, tail) = rest.split_at(1);
            f.write_str(first)?;
            f.write_str(match badchar {
                "<" => "&lt;",
                ">" => "&gt;",
                "&" => "&amp;",
                "\"" => "&quot;",
                "'" => "&#x27;",
                "/" => "&#x2f;",
                _ => unreachable!(),
            })?;
            s = tail;
        }
        f.write_str(s)
    }
    fn mime() -> mime::Mime { return mime::TEXT_HTML_UTF_8; }
    fn this_format() -> Self { HTML }
}

#[macro_export]
macro_rules! display_as_from_display {
    ($format:ty, $type:ty) => {
        impl DisplayAs<$format> for $type {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                (&self as &Display).fmt(f)
            }
        }
    }
}

#[macro_export]
macro_rules! display_integers_as {
    ($format:ty) => {
        display_as_from_display!($format, i8);
        display_as_from_display!($format, u8);
        display_as_from_display!($format, i16);
        display_as_from_display!($format, u16);
        display_as_from_display!($format, i32);
        display_as_from_display!($format, u32);
        display_as_from_display!($format, i64);
        display_as_from_display!($format, u64);
        display_as_from_display!($format, i128);
        display_as_from_display!($format, u128);
        display_as_from_display!($format, isize);
        display_as_from_display!($format, usize);
    }
}

display_integers_as!(HTML);

#[macro_export]
macro_rules! display_floats_as {
    ($format:ty, $e:expr, $after_e:expr, $e_cost:expr) => {
        impl DisplayAs<$format> for f64 {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                super::float::Floating::from(*self)
                    .fmt_with(f, $e, $after_e, $e_cost)
            }
        }
        impl DisplayAs<$format> for f32 {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                super::float::Floating::from(*self)
                    .fmt_with(f, $e, $after_e, $e_cost)
            }
        }
    }
}
display_floats_as!(HTML, "×10<sup>", "</sup>", 3);

#[test]
fn escaping() {
    assert_eq!(&format!("{}", As(HTML,"&")), "&amp;");
    assert_eq!(&format!("{}", As(HTML,"hello &>this is cool")),
               "hello &amp;&gt;this is cool");
    assert_eq!(&format!("{}", As(HTML,"hello &>this is 'cool")),
               "hello &amp;&gt;this is &#x27;cool");
}
#[test]
fn floats() {
    assert_eq!(&format!("{}", As(HTML, 3.0)), "3");
    assert_eq!(&format!("{}", As(HTML, 3e5)), "3×10<sup>5</sup>");
    assert_eq!(&format!("{}", As(HTML, 3e4)), "30000");
}
