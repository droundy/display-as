//! Format as LaTeX math mode

use super::*;

/// Format as LaTeX math mode.
pub struct Math;
impl Format for Math {
    fn mime() -> mime::Mime {
        return "text/x-latex".parse().unwrap();
    }
    fn this_format() -> Self {
        Math
    }
    fn escape(f: &mut Formatter, mut s: &str) -> Result<(), Error> {
        let badstuff = "&{}#%\\~$_^";
        while let Some(idx) = s.find(|c| badstuff.contains(c)) {
            let (first, rest) = s.split_at(idx);
            let (badchar, tail) = rest.split_at(1);
            f.write_str(first)?;
            f.write_str(match badchar {
                "&" => r"\&",
                "{" => r"\{",
                "}" => r"\}",
                "#" => r"\#",
                "%" => r"\%",
                "\\" => r"\textbackslash{}",
                "~" => r"\textasciitilde{}",
                "$" => r"\$",
                "_" => r"\_",
                "^" => r"\^",
                _ => unreachable!(),
            })?;
            s = tail;
        }
        f.write_str(s)
    }
}

display_integers_as!(Math);
display_floats_as!(Math, r"\times10^{", "}", 3, Some("10^{"));

#[test]
fn escaping() {
    assert_eq!(&format!("{}", "&".display_as(Math)), r"\&");
    assert_eq!(
        &format!("{}", "hello &>this is cool".display_as(Math)),
        r"hello \&>this is cool"
    );
    assert_eq!(
        &format!("{}", "hello &>this is 'cool".display_as(Math)),
        r"hello \&>this is 'cool"
    );
}
#[test]
fn floats() {
    assert_eq!(&format!("{}", 3.0.display_as(Math)), "3");
    assert_eq!(&format!("{}", 3e5.display_as(Math)), r"3\times10^{5}");
    assert_eq!(&format!("{}", 1e5.display_as(Math)), r"10^{5}");
    assert_eq!(&format!("{}", 3e4.display_as(Math)), "30000");
}
