#![deny(missing_docs)]

//! This crate defines a trait `DisplayAs` that allows a type to be
//! displayed in a particular format.

//! The `rouille` feature flag enables conversion of any `As<F,T>`
//! type into a `rouille::Response`.  Note that it is necessary to be
//! explicit about the format because a given type `T` may be
//! displayed in multiple different formats.

extern crate mime;

#[cfg(feature = "rouille")]
extern crate rouille;

use std::fmt::{Display, Formatter, Error};

#[macro_use]
mod html;
mod latex;
mod rust;

mod float;

pub use html::{HTML};
pub use latex::{LaTeX};
pub use rust::{Rust};

/// Format is a format that we can use for displaying data.
pub trait Format {
    /// "Escape" the given string so it can be safely displayed in
    /// this format.  The precise meaning of this may vary from format
    /// to format, but the general sense is that this string does not
    /// have any internal formatting, and must be displayed
    /// appropriately.
    fn escape(f: &mut Formatter, s: &str) -> Result<(), Error>;
    /// The mime type of this format.
    fn mime() -> mime::Mime;
    /// Return an actual `Format` for use in `As` below.
    fn this_format() -> Self;
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

#[cfg(feature = "rouille")]
impl<F: Format, T: DisplayAs<F>> Into<rouille::Response> for As<F,T> {
    fn into(self) -> rouille::Response {
        let s = format!("{}", &self);
        rouille::Response::from_data(F::mime().as_ref().to_string(), s)
    }
}

impl<F: Format> DisplayAs<F> for String {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        F::escape(f, self)
    }
}
impl<F: Format> DisplayAs<F> for str {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        F::escape(f, self)
    }
}
impl<'a, F: Format> DisplayAs<F> for &'a str {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        F::escape(f, self)
    }
}

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
