//! [Format] as HTML

use super::*;

/// [Format] as HTML.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HTML;
impl Format for HTML {
    fn escape(f: &mut Formatter, s: &str) -> Result<(), Error> {
        escape_with(f, s, |c| match c {
            '<' => Some("&lt;"),
            '>' => Some("&gt;"),
            '&' => Some("&amp;"),
            '"' => Some("&quot;"),
            '\'' => Some("&#x27;"),
            '/' => Some("&#x2f;"),
            _ => None,
        })
    }
    /// The MIME type for HTML is [mime::TEXT_HTML_UTF_8].
    fn mime() -> mime::Mime {
        return mime::TEXT_HTML_UTF_8;
    }
    fn this_format() -> Self {
        HTML
    }
}

/// Conveniently implement [DisplayAs] for integers for a new [Format].
///
/// ```
/// #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// struct Plain;
/// impl display_as::Format for Plain {
///     fn escape(f: &mut ::std::fmt::Formatter, s: &str) -> Result<(), ::std::fmt::Error> {
///         f.write_str(s) // for example I skip escaping...
///     }
///     fn mime() -> mime::Mime { mime::TEXT_PLAIN_UTF_8 }
///     fn this_format() -> Self { Plain }
/// }
/// display_as::display_integers_as!(Plain);
/// assert_eq!(&display_as::format_as!(Plain, 42u8 " and " -7i64).into_string(), "42 and -7");
/// ```
#[macro_export]
macro_rules! display_integers_as {
    (@impl $format:ty, $($type:ty),*) => {
        $(
            impl $crate::DisplayAs<$format> for $type {
                fn fmt(
                    &self,
                    f: &mut ::std::fmt::Formatter,
                ) -> ::std::result::Result<(), ::std::fmt::Error> {
                    ::std::fmt::Display::fmt(self, f)
                }
            }
        )*
    };
    ($format:ty) => {
        $crate::display_integers_as!(
            @impl $format, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
        );
    };
}

display_integers_as!(HTML);

/// Inconveniently implement [DisplayAs] for floats for a new [Format].
///
/// This is inconvenient because we want to enable pretty formatting
/// of both large and small numbers in whatever markup language we are
/// using.  The first argument of the macro is the format that wants
/// implementation of [DisplayAs] for floats.
///
/// For partial documentation of the other files, see
/// [Floating::fmt_with](float/enum.Floating.html#method.fmt_with).
/// However, I think some examples for HTML will most easily define
/// the other arguments.
/// ```
/// #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// struct HTML;
/// use display_as::{Format, format_as};
/// impl Format for HTML {
///    fn escape(f: &mut ::std::fmt::Formatter, mut s: &str) -> Result<(), ::std::fmt::Error> {
///        f.write_str(s) // for example I skip escaping...
///    }
///    fn mime() -> mime::Mime { return mime::TEXT_HTML_UTF_8; }
///    fn this_format() -> Self { HTML }
/// }
/// display_as::display_floats_as!(HTML, "×10<sup>", "</sup>", 3, Some("10<sup>"));
/// fn main() {
///   assert_eq!(&format_as!(HTML, 1e3).into_string(), "1000");
///   assert_eq!(&format_as!(HTML, 3e4).into_string(), "30000");
///   assert_eq!(&format_as!(HTML, 1e5).into_string(), "10<sup>5</sup>");
///   assert_eq!(&format_as!(HTML, 2e5).into_string(), "2×10<sup>5</sup>");
///   assert_eq!(&format_as!(HTML, 1e6).into_string(), "10<sup>6</sup>");
/// }
/// ```
#[macro_export]
macro_rules! display_floats_as {
    ($format:ty, $e:expr, $after_e:expr, $e_cost:expr, $power_ten:expr) => {
        impl $crate::DisplayAs<$format> for f64 {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                $crate::float::Floating::from(*self).fmt_with(f, $e, $after_e, $e_cost, $power_ten)
            }
        }
        impl $crate::DisplayAs<$format> for f32 {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                $crate::float::Floating::from(*self).fmt_with(f, $e, $after_e, $e_cost, $power_ten)
            }
        }
    };
}
display_floats_as!(HTML, "×10<sup>", "</sup>", 3, Some("10<sup>"));

#[test]
fn escaping() {
    assert_eq!(&format_as!(HTML, ("&")).into_string(), "&amp;");
    assert_eq!(
        &format_as!(HTML, ("hello &>this is cool")).into_string(),
        "hello &amp;&gt;this is cool"
    );
    assert_eq!(
        &format_as!(HTML, ("hello &>this is 'cool")).into_string(),
        "hello &amp;&gt;this is &#x27;cool"
    );
}
#[test]
fn floats() {
    assert_eq!(&format_as!(HTML, 3.0).into_string(), "3");
    assert_eq!(&format_as!(HTML, 3e5).into_string(), "3×10<sup>5</sup>");
    assert_eq!(&format_as!(HTML, 1e-6).into_string(), "10<sup>-6</sup>");
    assert_eq!(&format_as!(HTML, 3e4).into_string(), "30000");
}
