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
    fn escape(&str) -> String;
}

/// This trait is analogous to `Display`, but will display the data in
/// `F` format.
pub trait DisplayAs<F: Format> {
    /// Formats the value using the given formatter.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;
}

/// Choose to `Display` this type using `Format` `F`.
pub struct As<F: Format, T: DisplayAs<F>>(F,T);

impl<F: Format, T: DisplayAs<F>> Display for As<F,T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.1.fmt(f)
    }
}

/// Format as HTML.
pub struct HTML;
impl Format for HTML {
    fn escape(s: &str) -> String {
        s.to_string()
    }
}
impl DisplayAs<HTML> for String {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        (&HTML::escape(&self) as &Display).fmt(f)
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
    }
}
display_as_from_display!(HTML, str);
display_as_primitives!(HTML);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
