#![deny(missing_docs)]

//! This will be a template library that uses `display-as`.

extern crate display_as;
extern crate glob;
#[macro_use]
extern crate combine;

use display_as::{As, Rust};
use std::fs::File;
use std::io::Write;

mod parse;

/// Use this function in your `build.rs` to compile templates.
pub fn compile_templates(files_glob: &str) -> std::io::Result<()> {
    for path in glob::glob(files_glob).expect("unable to read template directory") {
        if let Ok(path) = path {
            let template = std::fs::read_to_string(&path)?;
            let tt = parse::parse_template(&template);
            let mut f = File::create(path.with_extension("rs"))?;
            writeln!(f, "fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {{");
            for t in tt.iter() {
                writeln!(f, "{}", As(Rust, t.clone()));
            }
            writeln!(f, "}}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::compile_templates;
    #[test]
    fn it_works() {
        compile_templates("templates/foo.html").unwrap();
    }
}
