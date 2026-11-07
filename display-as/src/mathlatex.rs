//! Format as LaTeX math mode

use super::*;

/// Format as LaTeX math mode.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Math;
impl Format for Math {
    fn mime() -> mime::Mime {
        return "text/x-latex".parse().unwrap();
    }
    fn this_format() -> Self {
        Math
    }
    fn escape(f: &mut Formatter, s: &str) -> Result<(), Error> {
        escape_with(f, s, |c| match c {
            '&' => Some(r"\&"),
            '{' => Some(r"\{"),
            '}' => Some(r"\}"),
            '#' => Some(r"\#"),
            '%' => Some(r"\%"),
            '\\' => Some(r"\backslash{}"),
            '~' => Some(r"\tilde{}"),
            '$' => Some(r"\$"),
            '_' => Some(r"\_"),
            '^' => Some(r"\hat{}"),
            _ => None,
        })
    }
}

display_integers_as!(Math);
display_floats_as!(Math, r"\times10^{", "}", 3, Some("10^{"));

#[test]
fn escaping() {
    assert_eq!(&format_as!(Math, ("&")).into_string(), r"\&");
    assert_eq!(
        &format_as!(Math, ("hello &>this is cool")).into_string(),
        r"hello \&>this is cool"
    );
    assert_eq!(
        &format_as!(Math, ("hello &>this is 'cool")).into_string(),
        r"hello \&>this is 'cool"
    );
    // Text-mode commands like \textbackslash and \^ do not work in math mode.
    assert_eq!(
        &format_as!(Math, ("a\\b~c^d")).into_string(),
        r"a\backslash{}b\tilde{}c\hat{}d"
    );
}
#[test]
fn floats() {
    assert_eq!(&format_as!(Math, 3.0).into_string(), "3");
    assert_eq!(&format_as!(Math, 3e5).into_string(), r"3\times10^{5}");
    assert_eq!(&format_as!(Math, 1e5).into_string(), r"10^{5}");
    assert_eq!(&format_as!(Math, 3e4).into_string(), "30000");
}
