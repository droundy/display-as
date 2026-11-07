//! Templates nested inside other templates.
//!
//! Every expression in a template gets a helper trait with a method name that
//! is derived from the template, and these names must differ between a
//! template and the templates nested inside it (otherwise the compiler
//! reports an ambiguous method call).

extern crate display_as;

use display_as::{format_as, with_template, write_as, DisplayAs, HTML, LaTeX};

struct Foo(isize);

#[with_template("Foo " self.0)]
impl DisplayAs<HTML> for Foo {}

#[test]
fn format_in_format() {
    assert_eq!(
        format_as!(HTML, "testing " format_as!(HTML, "hello " Foo(2)) " and " Foo(1)).into_string(),
        "testing hello Foo 2 and Foo 1"
    );
}

#[test]
fn format_in_format_in_format() {
    assert_eq!(
        format_as!(
            HTML,
            "a " format_as!(HTML, "b " format_as!(HTML, "c " Foo(3)) " " Foo(2)) " " Foo(1)
        )
        .into_string(),
        "a b c Foo 3 Foo 2 Foo 1"
    );
}

#[test]
fn identical_templates_nested() {
    // The inner and outer templates have the same tokens up to the nesting.
    assert_eq!(
        format_as!(HTML, "x" Foo(1) "x" format_as!(HTML, "x" Foo(1))).into_string(),
        "xFoo 1xxFoo 1"
    );
}

#[test]
fn different_formats_nested() {
    assert_eq!(
        format_as!(HTML, "html " format_as!(LaTeX, "latex " 7) as LaTeX " " 8).into_string(),
        "html latex 7 8"
    );
}

struct Outer(isize);

#[with_template("Outer " format_as!(HTML, "inner " Foo(self.0) " innermost " format_as!(HTML, "<" self.0 ">")))]
impl DisplayAs<HTML> for Outer {}

#[test]
fn format_in_with_template() {
    assert_eq!(
        format_as!(HTML, Outer(5)).into_string(),
        "Outer inner Foo 5 innermost <5>"
    );
}

#[test]
fn format_in_let_subtemplate() {
    let s = format_as!(
        HTML,
        let sub = {
            "sub " format_as!(HTML, "inner " Foo(2) " innermost " format_as!(HTML, "<" Foo(3) ">"))
        };
        "outer " sub " " format_as!(HTML, "after " sub)
    );
    assert_eq!(
        s.into_string(),
        "outer sub inner Foo 2 innermost <Foo 3> after sub inner Foo 2 innermost <Foo 3>"
    );
}

#[test]
fn format_in_control_flow() {
    let v = vec![1isize, 2];
    assert_eq!(
        format_as!(
            HTML,
            for i in v.iter() {
                if *i > 1 {
                    "big " format_as!(HTML, "[" i "]")
                } else {
                    "small " format_as!(HTML, "[" i "]")
                }
            }
            match v.len() {
                2 => {
                    " two " format_as!(HTML, "(" v.len() ")")
                }
                _ => {
                    " other"
                }
            }
        )
        .into_string(),
        "small [1]big [2] two (2)"
    );
}

#[test]
fn format_in_write_as_and_back() {
    let mut out = String::new();
    write_as!(
        HTML,
        out,
        "w " format_as!(HTML, "f " format_as!(HTML, "g " Foo(1)))
    )
    .unwrap();
    assert_eq!(out, "w f g Foo 1");
}

fn written(n: isize) -> String {
    let mut inner = String::new();
    write_as!(HTML, inner, "inner " Foo(n) " " format_as!(HTML, "<" n ">")).unwrap();
    inner
}

#[test]
fn format_around_write() {
    // `written` is a function, so the write_as! is not nested inside the
    // format_as! below, but the helper traits of both must coexist.
    assert_eq!(
        format_as!(HTML, "outer " written(1) as HTML).into_string(),
        "outer inner Foo 1 &lt;1&gt;"
    );
}
