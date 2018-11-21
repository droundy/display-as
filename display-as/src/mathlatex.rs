//! Format as LaTeX math mode

use super::*;

/// Format as LaTeX math mode.
pub struct Math;
impl Format for Math {
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

#[test]
fn escaping() {
    assert_eq!(&format!("{}", As(Math,"&")), r"\&");
    assert_eq!(&format!("{}", As(Math,"hello &>this is cool")),
               r"hello \&>this is cool");
    assert_eq!(&format!("{}", As(Math,"hello &>this is 'cool")),
               r"hello \&>this is 'cool");
}

