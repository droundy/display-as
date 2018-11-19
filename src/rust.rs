use display_as::{DisplayAs, Rust};
use std::fmt::{Error, Formatter};

use regex::Regex;

#[derive(Eq, Debug, PartialEq, Clone)]
pub struct Rule {
    pub impl_text: String,
    pub format: String,
    pub template_name: String,
}

impl DisplayAs<Rust> for Rule {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} {{", &self.impl_text)
    }
}

pub fn parse_rust(s: &str) -> Vec<Rule> {
    let re = Regex::new(r#"\sdisplay_as_template!\((?P<fo>[^;]+);(?P<im>[^;]+)\s*\{\s*\}\s*;\s*"(?P<fn>[^\)]+)"\s*\);"#).unwrap();
    let mut out = Vec::new();
    for caps in re.captures_iter(s) {
        out.push(Rule { impl_text: caps["im"].to_string(),
                        format: caps["fo"].to_string(),
                        template_name: caps["fn"].to_string(),
        });
    }
    out
}
