//! Format as LaTeX

use super::*;

/// Format as LaTeX.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LaTeX;
impl Format for LaTeX {
    fn mime() -> mime::Mime {
        return "text/x-latex".parse().unwrap();
    }
    fn this_format() -> Self {
        LaTeX
    }
    fn escape(f: &mut Formatter, s: &str) -> Result<(), Error> {
        escape_with(f, s, |c| match c {
            '&' => Some(r"\&"),
            '{' => Some(r"\{"),
            '}' => Some(r"\}"),
            '#' => Some(r"\#"),
            '%' => Some(r"\%"),
            '\\' => Some(r"\textbackslash{}"),
            '~' => Some(r"\textasciitilde{}"),
            '$' => Some(r"\$"),
            '_' => Some(r"\_"),
            '^' => Some(r"\^{}"),
            _ => None,
        })
    }
}

display_integers_as!(LaTeX);
display_floats_as!(LaTeX, r"$\times10^{", "}$", 3, Some("$10^{"));

#[test]
fn escaping() {
    assert_eq!(&format_as!(LaTeX, ("&")).into_string(), r"\&");
    assert_eq!(
        &format_as!(LaTeX, ("hello &>this is cool")).into_string(),
        r"hello \&>this is cool"
    );
    assert_eq!(
        &format_as!(LaTeX, ("hello &>this is 'cool")).into_string(),
        r"hello \&>this is 'cool"
    );
    // A bare \^ would put an accent on whatever follows.
    assert_eq!(&format_as!(LaTeX, ("a^b^")).into_string(), r"a\^{}b\^{}");
}
#[test]
fn floats() {
    assert_eq!(&format_as!(LaTeX, 3.0).into_string(), "3");
    assert_eq!(&format_as!(LaTeX, 3e5).into_string(), r"3$\times10^{5}$");
    assert_eq!(&format_as!(LaTeX, 3e4).into_string(), "30000");
}
