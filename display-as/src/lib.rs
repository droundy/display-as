#![deny(missing_docs)]
//! This template crate uses and defines a [`DisplayAs`] trait, which
//! allows a type to be displayed in a particular format.
//!
//! # Overview
//!
//! This crate defines three things that you need be aware of in order
//! to use it: the [`Format`] trait, which defines a markup language or
//! other format, the [`DisplayAs`] trait which is implemented for any
//! type that can be converted into some [`Format`], and finally the
//! template language and macros which allow you to conveniently
//! implement [`DisplayAs`] for your own types.  I will describe each of
//! these concepts in order.  (**FIXME** I should also have a
//! quick-start...)
//!
//! ## [`Format`]
//!
//! There are a number of predefined Formats (and I can easily add
//! more if there are user requests), so the focus here will be on
//! using these Formats, rather than on defining your own (which also
//! isn't too hard).  A format is a zero-size type that has a rule for
//! escaping strings and an associated MIME type.  The builtin formats
//! include [`HTML`], [`LaTeX`], and [`Math`] (which is math-mode LaTeX).
//!
//! ## [`DisplayAs`]`<F>`
//!
//! The `[`DisplayAs`]<F: Format>` trait is entirely analogous to the [Display](std::fmt::Display) trait
//! in the standard library, except that it is parametrized by a
//! [`Format`] so you can have different representations for the same
//! type in different formats.  This also makes it harder to
//! accidentally include the wrong representation in your output.
//!
//! Most of the primitive types already have [`DisplayAs`] implemented
//! for the included Formats.  If you encounter a type that you wish
//! had [`DisplayAs`] implemented for a given format, just let me know.
//! You can manually implement [`DisplayAs`] for any of your own types
//! (it's not worse than implementing [Display](std::fmt::Display)) but that isn't how
//! you are intended to do things (except perhaps in very simple
//! cases, like a wrapper around an integer).  Instead you will want
//! to use a template to implement [`DisplayAs`] for your own types.
//!
//! ## Templates!
//!
//! There are two template macros that you can use.  If you just want
//! to get a string out of one or more [`DisplayAs`] objects, you will
//! use something like `format_as!(HTML, "hello world" value).into_string()`.  If
//! you want to implement [`DisplayAs`], you will use the attribute
//! [`with_template!`].  In these examples I will use
//! [`format_as!`] because that makes it easy to write testable
//! documentation.  But in practice you will most likely primarily use
//! the [with_template] attribute.
//!
//! ### String literals
//!
//! The first thing you can include in a template is a string literal,
//! which is treated literally.
//!
//! ```
//! use display_as::{HTML, format_as};
//! assert_eq!(&format_as!(HTML, "Treat this literally <" ).into_string(),
//!            "Treat this literally <");
//! ```
//!
//! ### Expressions
//!
//! String literals are essential to representing some other [`Format`].
//! To include your data in the output, you can include any expression
//! that yields a type with [`DisplayAs`]`<F>` where `F` is your [`Format`].
//! Each expression is delimited by string literals (or the other
//! options below).  Note that since an expression is
//!
//! ```
//! use display_as::{HTML, format_as};
//! let s = "This is not a literal: <";
//! assert_eq!(&format_as!(HTML, s ).into_string(),
//!            "This is not a literal: &lt;");
//! ```
//!
//! ### Blocks and conditionals
//!
//! You can use braces to enclose any template expression.  Any rust
//! code before the braces is treated as literal rust.  This enables
//! you to write conditionals, match expressions, and loops.
//!
//! ```
//! use display_as::{HTML, format_as};
//! assert_eq!(&format_as!(HTML,
//!                        for i in 1..4 {
//!                            "Counting " i "...\n"
//!                        }
//!                        "Blast off!").into_string(),
//!            "Counting 1...\nCounting 2...\nCounting 3...\nBlast off!");
//! ```
//!
//! ### Semicolons
//!
//! You may also play any rust statements you wish, if you end them
//! with a semicolon.  This enables you to define local variables.
//!
//! ```
//! use display_as::{HTML, format_as};
//! assert_eq!(&format_as!(HTML, "I am counting " let count = 5;
//!                              count " and again " count ).into_string(),
//!            "I am counting 5 and again 5");
//! ```
//!
//! ### Embedding a different format
//!
//! You can also embed in one format a representation from another
//! type.  This can be helpful, for instance, if you want to use
//! MathJax to handle LaTeX math embedded in an HTML file.
//!
//! ```
//! use display_as::{HTML, Math, format_as};
//! assert_eq!(&format_as!(HTML, "The number $" 1.2e12 as Math "$").into_string(),
//!            r"The number $1.2\times10^{12}$");
//! ```
//!
//! ### Saving a portion of a template for reuse
//!
//! You can also save a template expression using a let statement,
//! provided the template expression is enclosed in braces.  This
//! allows you to achieve goals similar to the base templates in
//! Jinja2.  (Once we have an include feature... Example to come in
//! the future.)
//!
//! ```
//! use display_as::{HTML, format_as};
//! assert_eq!(&format_as!(HTML,
//!                        let x = 1;
//!                        let announce = { "number " x };
//!                        "The " announce " is silly " announce).into_string(),
//!            "The number 1 is silly number 1");
//! ```
//!
//! ## Differences when putting a template in a file
//!
//! You will most likely always put largish templates in a separate
//! file.  This makes editing your template simpler and keeps things
//! in general easier.  The template language for templates held in a
//! distinct file has one difference from those shown above: the file
//! always begins and ends with string literals, but their initial and
//! final quotes respectively are omitted.  Furthermore, the first and
//! last string literals must be "raw" literals with a number of #
//! signs equal to the maximum used in the template.  I suggest using
//! an equal number of # signs for all string literals in a given
//! template.  Thus a template might look like:
//!
//! ```ignore
//! <html>
//!   <body>
//!     "## self.title r##"
//!   </body>
//! </html>
//! ```

//! You can see that the quotes appear "inside out."  This is
//! intentional, so that for most formats the quotes will appear to
//! enclose the rust code rather than everything else, and as a result
//! editors will hopefully be able to do the "right thing" for the
//! template format (e.g. HTML in this case).

//! ## Using `include!("...")` within a template
//!
//! Now I will demonstrate how you can include template files within
//! other template files by using the `include!` macro within a
//! template.  To demonstrate this, we will need a few template files.
//!
//! We will begin with a "base" template that describes how a page is
//! laid out.
//! #### `base.html`:
//! ```ignore
#![cfg_attr(feature = "docinclude", doc = include_str!("base.html"))]
//! ```
//! We can have a template for how we will display students...
//! #### `student.html`:
//! ```ignore
#![cfg_attr(feature = "docinclude", doc = include_str!("student.html"))]
//!```
//! Finally, an actual web page describing a class!
//! #### `class.html`:
//! ```ignore
#![cfg_attr(feature = "docinclude", doc = include_str!("class.html"))]
//! ```
//! Now to put all this together, we'll need some rust code.
//!
//! ```
//! use display_as::{DisplayAs, HTML, format_as, with_template};
//! struct Student { name: &'static str };
//! #[with_template("student.html")]
//! impl DisplayAs<HTML> for Student {}
//!
//! struct Class { coursename: &'static str, coursenumber: usize, students: Vec<Student> };
//! #[with_template("class.html")]
//! impl DisplayAs<HTML> for Class {}
//!
//! let myclass = Class {
//!       coursename: "Templates",
//!       coursenumber: 365,
//!       students: vec![Student {name: "David"}, Student {name: "Joel"}],
//! };
//! assert_eq!(&format_as!(HTML, myclass).into_string(), r#"<title>PH365: Templates</title>
//! <html>
//!   <ul><li><span class="student">Name: David</span>
//!
//! </li><li><span class="student">Name: Joel</span>
//!
//! </li></ul>
//! </html>
//!
//!
//! "#);
//! ```

extern crate display_as_proc_macro;
extern crate mime;
extern crate self as display_as;

/// Use the given template to create a [`FormattedString`].
///
/// You can think of this as being kind of like [`format!`] on strange drugs.
/// We return a [`FormattedString`] instaed of a [String] so that
/// You can store the output and use it later in another template without
/// having the contents escaped.
///
/// To obtain a [`String`], use the [`FormattedString::into_string`] method.
pub use display_as_proc_macro::format_as;

/// Write the given template to a file.
///
/// You can think of this as being kind of like [`write!`] on strange drugs.
pub use display_as_proc_macro::write_as;

/// Can I write doc here?
pub use display_as_proc_macro::with_template;

use std::fmt::{Display, Error, Formatter};

#[macro_use]
mod html;
mod latex;
mod mathlatex;
mod rust;
mod url;
mod utf8;

pub mod float;

pub use crate::html::HTML;
pub use crate::latex::LaTeX;
pub use crate::mathlatex::Math;
pub use crate::rust::Rust;
pub use crate::url::URL;
pub use crate::utf8::UTF8;

/// Format is a format that we can use for displaying data.
pub trait Format: Sync + Send + Copy + Eq + Ord + std::hash::Hash {
    /// "Escape" the given string so it can be safely displayed in
    /// this format.  The precise meaning of this may vary from format
    /// to format, but the general sense is that this string does not
    /// have any internal formatting, and must be displayed
    /// appropriately.
    fn escape(f: &mut Formatter, s: &str) -> Result<(), Error>;
    /// The mime type of this format.
    fn mime() -> mime::Mime;
    /// Return an actual [`Format`] for use in [`As`] below.
    fn this_format() -> Self;
}

/// This trait is analogous to [Display](std::fmt::Display), but will display the data in
/// `F` [`Format`].
pub trait DisplayAs<F: Format> {
    /// Formats the value using the given formatter.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;

    /// Estimate the size of this when displayed
    fn estimate_size(&self) -> usize {
        4
    }

    /// Creates a display object
    fn display<'a>(&'a self) -> As<'a, F, Self> {
        As::from(self)
    }
}

/// Create a Display object, which can be used with various web frameworks.
pub fn display<'a, F: Format, T: DisplayAs<F>>(_f: F, x: &'a T) -> As<'a, F, T> {
    x.display()
}

struct Closure<F: Format, C: Fn(&mut Formatter) -> Result<(), Error>> {
    f: C,
    _format: F,
}
/// Display the given closure as this format.
///
/// This is used internally in template handling.
pub fn display_closure_as<F: Format>(
    f: F,
    c: impl Fn(&mut Formatter) -> Result<(), Error>,
) -> impl DisplayAs<F> {
    Closure { f: c, _format: f }
}
impl<F: Format, C: Fn(&mut Formatter) -> Result<(), Error>> DisplayAs<F> for Closure<F, C> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        (self.f)(f)
    }
}
impl<F> PartialEq<FormattedString<F>> for str {
    fn eq(&self, other: &FormattedString<F>) -> bool {
        self == &other.inner
    }
}
impl<F> PartialEq<str> for FormattedString<F> {
    fn eq(&self, other: &str) -> bool {
        &self.inner == other
    }
}
#[test]
fn test_closure() {
    let x = |__f: &mut Formatter| -> Result<(), Error> {
        __f.write_str("hello world")?;
        Ok(())
    };
    assert_eq!(
        "hello world",
        &format_as!(HTML, display_closure_as(HTML, x))
    );
    assert_eq!(
        &format_as!(HTML, display_closure_as(HTML, x)),
        "hello world"
    );
}

/// Choose to [Display](std::fmt::Display) this type using a particular [`Format`] `F`.
pub struct As<'a, F: Format, T: DisplayAs<F> + ?Sized> {
    inner: &'a T,
    _format: F,
}
impl<'a, F: Format, T: DisplayAs<F> + ?Sized> From<&'a T> for As<'a, F, T> {
    fn from(value: &'a T) -> Self {
        As {
            _format: F::this_format(),
            inner: value,
        }
    }
}
impl<'a, F: Format, T: DisplayAs<F> + ?Sized> Display for As<'a, F, T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.inner.fmt(f)
    }
}

/// The `rouille` feature flag enables conversion of any `As<F,T>`
/// type into a [rouille::Response].  Note that it is necessary to be
/// explicit about the format because a given type `T` may be
/// displayed in multiple different formats.
#[cfg(feature = "rouille")]
pub mod rouille {
    extern crate rouille;
    use super::{As, DisplayAs, Format};
    impl<'a, F: Format, T: DisplayAs<F>> Into<rouille::Response> for As<'a, F, T> {
        fn into(self) -> rouille::Response {
            let s = format!("{}", &self);
            rouille::Response::from_data(F::mime().as_ref().to_string(), s)
        }
    }
}

/// The `actix-web` feature flag makes any [`As`] type a
/// [actix_web::Responder].
#[cfg(feature = "actix-web")]
pub mod actix {
    extern crate actix_web;
    use self::actix_web::{HttpRequest, HttpResponse, Responder};
    use super::{As, DisplayAs, Format};
    impl<'a, F: Format, T: 'a + DisplayAs<F>> Responder for As<'a, F, T> {
        type Item = HttpResponse;
        type Error = ::std::io::Error;
        fn respond_to<S: 'static>(
            self,
            _req: &HttpRequest<S>,
        ) -> Result<HttpResponse, Self::Error> {
            Ok(HttpResponse::Ok()
                .content_type(F::mime().as_ref().to_string())
                .body(format!("{}", &self)))
        }
    }
}

/// The `gotham-web` feature flag makes any [`As`] type a
/// [::gotham::handler::IntoResponse].
#[cfg(feature = "gotham")]
pub mod gotham {
    use crate::{As, DisplayAs, Format};
    use gotham::{
        handler::IntoResponse,
        hyper::{Body, Response, StatusCode},
        state::State,
    };

    impl<'a, F: Format, T: 'a + DisplayAs<F>> IntoResponse for As<'a, F, T> {
        fn into_response(self, state: &State) -> Response<Body> {
            let s = format!("{}", self);
            (StatusCode::OK, F::mime(), s).into_response(state)
        }
    }
}

/// The `usewarp` feature flag makes any [`DisplayAs`] type a [warp::Reply].
#[cfg(feature = "usewarp")]
pub mod warp {
    use crate::{As, DisplayAs, Format};
    impl<'a, F: Format, T: DisplayAs<F> + Sync> warp::Reply for As<'a, F, T> {
        /// Convert into a [warp::Reply].
        fn into_response(self) -> warp::reply::Response {
            let s = format!("{}", self);
            let m = F::mime().as_ref().to_string();
            http::Response::builder()
                .header("Content-type", m.as_bytes())
                .status(http::StatusCode::OK)
                .body(s)
                .unwrap()
                .map(hyper::Body::from)
        }
    }

    #[test]
    fn test_warp() {
        use crate::{display, HTML};
        use warp::Reply;
        // This sloppy test just verify that the code runs.
        display(HTML, &"hello world".to_string()).into_response();
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
    use super::{format_as, HTML};
    #[test]
    fn html_escaping() {
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
}

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// A `String` that is formatted in `F`
///
/// The `serde1`` feature flag enables a [`FormattedString`] to be
/// serialized and deserialized by [serde].
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde1", serde(transparent))]
#[cfg_attr(feature = "serde1", serde(bound(deserialize = "F: Format")))]
pub struct FormattedString<F> {
    inner: String,
    #[cfg_attr(feature = "serde1", serde(skip, default = "F::this_format"))]
    _format: F,
}

impl<F: Format> FormattedString<F> {
    /// Create a new `FormattedString` from an already-formatted `String`.
    pub fn from_formatted<S: Into<String>>(s: S) -> Self {
        FormattedString {
            inner: s.into(),
            _format: F::this_format(),
        }
    }
    /// Convert back into a string
    pub fn into_string(self) -> String {
        self.inner
    }
    /// Reference a `&str` from this
    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

impl<F: Format> DisplayAs<F> for FormattedString<F> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&self.inner)
    }
}
impl<F: Format> std::fmt::Debug for FormattedString<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
