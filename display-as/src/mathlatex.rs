//! Format as LaTeX math mode

use super::*;

/// Format as LaTeX math mode.
pub struct Math;
impl Format for Math {
    fn mime() -> mime::Mime { return "text/x-latex".parse().unwrap(); }
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
                "\\"=> r"\textbackslash{}",
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

display_as_primitives!(Math);
display_as_floats!(Math, r"\times10^{", "}", 3);

#[test]
fn escaping() {
    assert_eq!(&format!("{}", As(Math,"&")), r"\&");
    assert_eq!(&format!("{}", As(Math,"hello &>this is cool")),
               r"hello \&>this is cool");
    assert_eq!(&format!("{}", As(Math,"hello &>this is 'cool")),
               r"hello \&>this is 'cool");
}
#[test]
fn floats() {
    assert_eq!(&format!("{}", As(Math, 3.0)), "3");
    assert_eq!(&format!("{}", As(Math, 3e5)), r"3\times10^{5}");
    assert_eq!(&format!("{}", As(Math, 3e4)), "30000");
}
