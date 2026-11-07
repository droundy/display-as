//! Templates whose format is a generic parameter.
use display_as::{
    format_as, with_template, write_as, DisplayAs, Format, FormattedString, HTML, UTF8,
};

struct Points(u32);

impl<F: Format> DisplayAs<F> for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} points", self.0)
    }
}

fn sentence<F: Format>(p: &Points) -> FormattedString<F> {
    format_as!(F, "You have " p "!")
}

#[test]
fn format_as_with_generic_format() {
    assert_eq!(
        sentence::<HTML>(&Points(3)).into_string(),
        "You have 3 points!"
    );
    assert_eq!(
        sentence::<UTF8>(&Points(1)).into_string(),
        "You have 1 points!"
    );
}

fn escaped<F: Format>(s: &str, t: &String) -> FormattedString<F> {
    format_as!(F, s " " t " " (t.as_str()) " " (&&t))
}

#[test]
fn generic_format_escapes_expressions() {
    assert_eq!(
        escaped::<HTML>("<b>", &"&".to_string()).into_string(),
        "&lt;b&gt; &amp; &amp; &amp;"
    );
    assert_eq!(
        escaped::<UTF8>("<b>", &"&".to_string()).into_string(),
        "<b> & & &"
    );
}

fn sub_template<F: Format>(items: &[&str]) -> FormattedString<F> {
    format_as!(F,
    let item = { "<li>" (items.len().to_string()) "</li>" };
    for x in items {
        item " " x
    })
}

#[test]
fn generic_format_with_let_for_and_nested_format_as() {
    assert_eq!(
        sub_template::<HTML>(&["a", "b"]).into_string(),
        "<li>2</li> a<li>2</li> b"
    );
    let nested = |s: &str| -> FormattedString<HTML> {
        fn go<F: Format>(s: &str) -> FormattedString<F> {
            format_as!(F, "[" (format_as!(F, "(" s ")")) "]")
        }
        go(s)
    };
    assert_eq!(nested("&").into_string(), "[(&amp;)]");
}

fn write_generic<F: Format>(out: &mut String, s: &str) -> std::fmt::Result {
    write_as!(F, *out, "a" s "b")
}

#[test]
fn write_as_with_generic_format() {
    let mut out = String::new();
    write_generic::<HTML>(&mut out, "<").unwrap();
    assert_eq!(out, "a&lt;b");
    let mut out = String::new();
    write_generic::<UTF8>(&mut out, "<").unwrap();
    assert_eq!(out, "a<b");
}

struct Named(String);

#[with_template("Name: " self.0 ".")]
impl<F: Format> DisplayAs<F> for Named {}

#[test]
fn with_template_on_generic_impl() {
    let n = Named("R&D".to_string());
    assert_eq!(format_as!(HTML, n).into_string(), "Name: R&amp;D.");
    assert_eq!(format_as!(UTF8, n).into_string(), "Name: R&D.");
}

// An `as Format` expression still overrides the generic format.
fn mixed<F: Format>(p: &Points) -> FormattedString<F> {
    format_as!(F, "a " p " b " p as UTF8)
}

#[test]
fn generic_format_with_as_override() {
    assert_eq!(
        mixed::<HTML>(&Points(2)).into_string(),
        "a 2 points b 2 points"
    );
}
