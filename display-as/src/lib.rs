#![deny(missing_docs)]

//! This crate defines a trait `DisplayAs` that allows a type to be
//! displayed in a particular format.

use std::fmt::{Display, Formatter, Error};

/// Format is a format that we can use for displaying data.
pub trait Format {
    /// "Escape" the given string so it can be safely displayed in
    /// this format.  The precise meaning of this may vary from format
    /// to format, but the general sense is that this string does not
    /// have any internal formatting, and must be displayed
    /// appropriately.
    fn escape(f: &mut Formatter, s: &str) -> Result<(), Error>;
}

/// This trait is analogous to `Display`, but will display the data in
/// `F` format.
pub trait DisplayAs<F: Format> {
    /// Formats the value using the given formatter.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;
}

/// Choose to `Display` this type using `Format` `F`.
pub struct As<F: Format, T: DisplayAs<F>>(pub F,pub T);

impl<F: Format, T: DisplayAs<F>> Display for As<F,T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.1.fmt(f)
    }
}

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
}
/// Format as rust code.
pub struct Rust;
impl Format for Rust {
    fn escape(f: &mut Formatter, s: &str) -> Result<(), Error> {
        (&s as &std::fmt::Debug).fmt(f)
    }
}
macro_rules! display_as_from_escape {
    ($format:ident) => {
        impl DisplayAs<$format> for String {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                $format::escape(f, self)
            }
        }
        impl DisplayAs<$format> for str {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                $format::escape(f, self)
            }
        }
        impl<'a> DisplayAs<$format> for &'a str {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                $format::escape(f, self)
            }
        }
    }
}
macro_rules! display_as_from_display {
    ($format:ty, $type:ty) => {
        impl DisplayAs<$format> for $type {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                (&self as &Display).fmt(f)
            }
        }
    }
}
macro_rules! display_as_primitives {
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
        display_as_from_display!($format, f64);
        display_as_from_display!($format, f32);
    }
}
display_as_from_escape!(HTML);
display_as_primitives!(HTML);

display_as_from_escape!(Rust);
display_as_primitives!(Rust);

#[cfg(test)]
mod tests {
    use super::{As,HTML};
    #[test]
    fn html_escaping() {
        assert_eq!(&format!("{}", As(HTML,"&")), "&amp;");
        assert_eq!(&format!("{}", As(HTML,"hello &>this is cool")),
                   "hello &amp;&gt;this is cool");
        assert_eq!(&format!("{}", As(HTML,"hello &>this is 'cool")),
                   "hello &amp;&gt;this is &#x27;cool");
    }
}
