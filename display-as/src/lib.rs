#![deny(missing_docs)]
//! This template crate uses and defines a [DisplayAs] trait, which
//! allows a type to be displayed in a particular format.
//!
//! # Overview
//!
//! This crate defines three things that you need be aware of in order
//! to use it: the [Format] trait, which defines a markup language or
//! other format, the [DisplayAs] trait which is implemented for any
//! type that can be converted into some [Format], and finally the
//! template language and macros which allow you to conveniently
//! implement [DisplayAs] for your own types.  I will describe each of
//! these concepts in order.  (**FIXME** I should also have a
//! quick-start...)
//!
//! ## [Format]
//!
//! There are a number of predefined Formats (and I can easily add
//! more if there are user requests), so the focus here will be on
//! using these Formats, rather than on defining your own (which also
//! isn't too hard).  A format is a zero-size type that has a rule for
//! escaping strings and an associated MIME type.  The builtin formats
//! include [HTML], [LaTeX], and [Math] (which is math-mode LaTeX).
//!
//! ## [DisplayAs]`<F>`
//!
//! The `[DisplayAs]<F: Format>` trait is entirely analogous to the [Display](std::fmt::Display) trait
//! in the standard library, except that it is parametrized by a
//! [Format] so you can have different representations for the same
//! type in different formats.  This also makes it harder to
//! accidentally include the wrong representation in your output.
//!
//! Most of the primitive types already have [DisplayAs] implemented
//! for the included Formats.  If you encounter a type that you wish
//! had [DisplayAs] implemented for a given format, just let me know.
//! You can manually implement [DisplayAs] for any of your own types
//! (it's not worse than implementing [Display](std::fmt::Display)) but that isn't how
//! you are intended to do things (except perhaps in very simple
//! cases, like a wrapper around an integer).  Instead you will want
//! to use a template to implement [DisplayAs] for your own types.
//!
//! ## Templates!
//!
//! There are two template macros that you can use.  If you just want
//! to get a string out of one or more [DisplayAs] objects, you will
//! use something like `display_as_string!(HTML, "hello world" value)`.  If
//! you want to implement [DisplayAs], you will use the attribute
//! [with_template!].  In these examples I will use
//! [display_as_string!] because that makes it easy to write testable
//! documentation.  But in practice you will most likely primarily use
//! the [with_template] attribute.
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
//! String literals are essential to representing some other [Format].
//! To include your data in the output, you can include any expression
//! that yields a type with [DisplayAs]`<F>` where `F` is your [Format].
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
//! ### Embedding a different format
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
//! ### Saving a portion of a template for reuse
//!
//! You can also save a template expression using a let statement,
//! provided the template expression is enclosed in braces.  This
//! allows you to achieve goals similar to the base templates in
//! Jinja2.  (Once we have an include feature... Example to come in
//! the future.)
//!
//! ```
//! use display_as::{HTML, display_as_string};
//! assert_eq!(&display_as_string!(HTML,
//!                                let x = 1;
//!                                let announce = { "number " x };
//!                                "The " announce " is silly " announce),
//!                                "The number 1 is silly number 1");
//! ```
//!
//! ## Differences when putting a template in a file
//!
//! You will most likely always put largish templates in a separate
//! file.  This makes editing your template simpler and keeps things
//! in general easier.  The template language for templates held in a
//! distinct file has one differnce from those shown above: the file
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

#![cfg_attr(feature = "docinclude", feature(external_doc))]
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
#![cfg_attr(feature = "docinclude", doc(include = "base.html"))]
//! ```
//! We can have a template for how we will display students...
//! #### `student.html`:
//! ```ignore
#![cfg_attr(feature = "docinclude", doc(include = "student.html"))]
//!```
//! Finally, an actual web page describing a class!
//! #### `class.html`:
//! ```ignore
#![cfg_attr(feature = "docinclude", doc(include = "class.html"))]
//! ```
//! Now to put all this together, we'll need some rust code.
//!
//! ```
//! use display_as::{DisplayAs, HTML, display_as_string, with_template};
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
//! assert_eq!(&display_as_string!(HTML, myclass), r#"<title>PH365: Templates</title>
//! <html>
//!   <ul>
//!
//!   // This is buggy:  I want to iterate, but it fails!
//!   for s in self.students.iter() {
//!     "<li>" s "</li>"
//!   }
//!
//!   </ul>
//! </html>
//!
//!
//!"#);
//! ```

extern crate display_as_proc_macro;
extern crate mime;
extern crate proc_macro_hack;

#[proc_macro_hack]
pub use display_as_proc_macro::display_as_string;
use proc_macro_hack::proc_macro_hack;

/// Can I write doc here?
pub use display_as_proc_macro::with_template;

use std::fmt::{Display, Error, Formatter};

#[macro_use]
mod html;
mod latex;
mod mathlatex;
mod rust;

pub mod float;

pub use crate::html::HTML;
pub use crate::latex::LaTeX;
pub use crate::mathlatex::Math;
pub use crate::rust::Rust;

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
    /// Return an actual [Format] for use in [As] below.
    fn this_format() -> Self;
}

/// This trait is analogous to [Display](std::fmt::Display), but will display the data in
/// `F` [Format].
pub trait DisplayAs<F: Format> {
    /// Formats the value using the given formatter.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;

    /// Creates a display object
    fn display_as<'a>(&'a self, _format: F) -> As<'a, F, Self> {
        As::from(self)
    }
}

impl<F: Format, C: for<'a> Fn(F, &'a mut Formatter) -> Result<(), Error>> DisplayAs<F> for C {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self(F::this_format(), f)
    }
}

#[test]
fn test_closure() {
    let x = |_format: HTML, __f: &mut Formatter| -> Result<(), Error> {
        __f.write_str("hello world")?;
        Ok(())
    };
    assert_eq!("hello world", &format!("{}", x.display_as(HTML)));
}

/// Choose to [Display](std::fmt::Display) this type using a particular [Format] `F`.
pub struct As<'a, F: Format, T: DisplayAs<F> + ?Sized> {
    inner: &'a T,
    phantom: std::marker::PhantomData<F>,
}
impl<'a, F: Format, T: DisplayAs<F> + ?Sized> From<&'a T> for As<'a, F, T> {
    fn from(value: &'a T) -> Self {
        As {
            phantom: std::marker::PhantomData,
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
    impl<F: Format, T: DisplayAs<F>> Into<rouille::Response> for As<F, T> {
        fn into(self) -> rouille::Response {
            let s = format!("{}", &self);
            rouille::Response::from_data(F::mime().as_ref().to_string(), s)
        }
    }
}

/// The `actix-web` feature flag makes any [As] type a
/// [actix_web::Responder].
#[cfg(feature = "actix-web")]
pub mod actix {
    extern crate actix_web;
    use self::actix_web::{HttpRequest, HttpResponse, Responder};
    use super::{As, DisplayAs, Format};
    impl<F: Format, T: DisplayAs<F>> Responder for As<F, T> {
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

/// The `gotham-web` feature flag makes any [As] type a
/// [gotham::IntoResponse].
#[cfg(feature = "gotham-web")]
pub mod gotham {
    extern crate gotham;
    extern crate http;
    extern crate hyper;
    use super::{As, DisplayAs, Format};
    impl<F: Format, T: DisplayAs<F>> gotham::handler::IntoResponse for As<F, T> {
        fn into_response(self, state: &gotham::state::State) -> http::Response<hyper::Body> {
            let s = format!("{}", &self);
            (http::StatusCode::OK, F::mime(), s).into_response(state)
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
    use super::{DisplayAs, HTML};
    #[test]
    fn html_escaping() {
        assert_eq!(&format!("{}", "&".display_as(HTML)), "&amp;");
        assert_eq!(
            &format!("{}", "hello &>this is cool".display_as(HTML)),
            "hello &amp;&gt;this is cool"
        );
        assert_eq!(
            &format!("{}", "hello &>this is 'cool".display_as(HTML)),
            "hello &amp;&gt;this is &#x27;cool"
        );
    }
}
