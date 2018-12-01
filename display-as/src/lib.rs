#![deny(missing_docs)]

//! This template crate uses and defines a `DisplayAs` trait, which
//! allows a type to be displayed in a particular format.
//!
//! # Overview
//!
//! This crate defines three things that you need be aware of in order
//! to use it: the `Format` trait, which defines a markup language or
//! other format, the `DisplayAs` trait which is implemented for any
//! type that can be converted into some `Format`, and finally the
//! template language and macros which allow you to conveniently
//! implement `DisplayAs` for your own types.  I will describe each of
//! these concepts in order.  (**FIXME** I should also have a
//! quick-start...)
//!
//! ## `Format`
//!
//! There are a number of predefined Formats (and I can easily add
//! more if there are user requests), so the focus here will be on
//! using these Formats, rather than on defining your own (which also
//! isn't too hard).  A format is a zero-size type that has a rule for
//! escaping strings and an associated MIME type.  The builtin formats
//! include `HTML`, `LaTeX`, and `Math` (which is math-mode LaTeX).
//!
//! ## `DisplayAs<F>`
//!
//! The `DisplayAs<F: Format>` trait is entirely analogous to the `Display` trait
//! in the standard library, except that it is parametrized by a
//! `Format` so you can have different representations for the same
//! type in different formats.  This also makes it harder to
//! accidentally include the wrong representation in your output.
//!
//! Most of the primitive types already have `DisplayAs` implemented
//! for the included Formats.  If you encounter a type that you wish
//! had `DisplayAs` implemented for a given format, just let me know.
//! You can manually implement `DisplayAs` for any of your own types
//! (it's not worse than implementing `Display`) but that isn't how
//! you are intended to do things (except perhaps in very simple
//! cases, like a wrapper around an integer).  Instead you will want
//! to use a template to implement `DisplayAs` for your own types.
//!
//! ## Templates!
//!
//! There are two template macros that you can use.  If you just want
//! to get a string out of one or more `DisplayAs` objects, you will
//! use something like `display_as_string!("hello world" value)`.  If
//! you want to implement `DisplayAs`, you will use the attribute
//! `with_template`.
//!
//! ### String literals
//!
//! The first thing you can include in a template is a string literal,
//! which is treated literally.
//!
//! ```
//! use display_as::{HTML, display_as_string};
//! assert_eq!(&display_as_string!(HTML, "Treat this literally <" ),
//!                                "Treat this literally <");
//! ```
//!
//! ### Expressions
//!
//! String literals are essential to representing some other format.
//! To include your data in the output, you can include any expression
//! that yields a type with `DisplayAs<F>` where `F` is your format.
//! Each expression is delimited by string literals (or the other
//! options below).  Note that since an expression is
//!
//! ```
//! use display_as::{HTML, display_as_string};
//! let s = "This is not a literal: <";
//! assert_eq!(&display_as_string!(HTML, s ),
//!                                "This is not a literal: &lt;");
//! ```
//!
//! ### Blocks and conditionals
//!
//! You can use braces to enclose any template expression.  Any rust
//! code before the braces is treated as literal rust.  This enables
//! you to write conditionals, match expressions, and loops.
//!
//! ```
//! use display_as::{HTML, display_as_string};
//! assert_eq!(&display_as_string!(HTML,
//!                                for i in 1..4 {
//!                                    "Counting " i "...\n"
//!                                }
//!                                "Blast off!"),
//!                                "Counting 1...\nCounting 2...\nCounting 3...\nBlast off!");
//! ```
//!
//! ### Semicolons
//!
//! You may also play any rust statements you wish, if you end them
//! with a semicolon.  This enables you to define local variables.
//!
//! ```
//! use display_as::{HTML, display_as_string};
//! assert_eq!(&display_as_string!(HTML, "I am counting " let count = 5;
//!                                      count " and again " count ),
//!                                "I am counting 5 and again 5");
//! ```
//!
//! ### Using another type
//!
//! You can also embed in one format a representation from another
//! type.  This can be helpful, for instance, if you want to use
//! MathJax to handle LaTeX math embedded in an HTML file.
//!
//! ```
//! use display_as::{HTML, Math, display_as_string};
//! assert_eq!(&display_as_string!(HTML, "The number $" 1.2e12 as Math "$"),
//!                                r"The number $1.2\times10^{12}$");
//! ```
//!
//! ### Nesting templates?
//!
//! I want to figure out how to enable you to nest one template in
//! another.
//!
//! ```
//! use display_as::{HTML, display_as_string};
//! assert_eq!(&display_as_string!(HTML, "The number " let x = 5; x ),
//!                                "The number 5");
//! ```


extern crate mime;
extern crate display_as_proc_macro;
extern crate proc_macro_hack;

use proc_macro_hack::proc_macro_hack;
#[proc_macro_hack]
pub use display_as_proc_macro::{display_as_string};

/// Can I write doc here?
pub use display_as_proc_macro::{with_template};

use std::fmt::{Display, Formatter, Error};

#[macro_use]
mod html;
mod latex;
mod mathlatex;
mod rust;

pub mod float;

pub use html::{HTML};
pub use latex::{LaTeX};
pub use mathlatex::{Math};
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

/// The `rouille` feature flag enables conversion of any `As<F,T>`
/// type into a `rouille::Response`.  Note that it is necessary to be
/// explicit about the format because a given type `T` may be
/// displayed in multiple different formats.
#[cfg(feature = "rouille")]
pub mod rouille {
    extern crate rouille;
    use super::{Format, As, DisplayAs};
    impl<F: Format, T: DisplayAs<F>> Into<rouille::Response> for As<F,T> {
        fn into(self) -> rouille::Response {
            let s = format!("{}", &self);
            rouille::Response::from_data(F::mime().as_ref().to_string(), s)
        }
    }
}

#[cfg(feature = "actix-web")]
pub mod actix {
    extern crate actix_web;
    use actix_web::{Responder, HttpRequest, HttpResponse};
    use super::{Format, As, DisplayAs};
    impl<F: Format, T: DisplayAs<F>> Responder for As<F,T> {
        fn respond_to(self, _req: &HttpRequest<S>)
                      -> Result<HttpResponse, Self::Error> {
            Ok(HttpResponse::Ok()
               .content_type(F::mime().as_ref().to_string())
               .body(format!("{}", &self)))
        }
    }
}

impl<F: Format> DisplayAs<F> for String {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        F::escape(f, self)
    }
}
impl<'a, F: Format> DisplayAs<F> for &'a String {
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
