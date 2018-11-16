#![deny(missing_docs)]

//! This will be a template library that uses `display-as`.

pub extern crate display_as;
extern crate glob;
#[macro_use]
extern crate combine;

use display_as::{As, Rust};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

mod parse;

fn add_rs_extension(p: &Path) -> PathBuf {
    let mut p = p.to_string_lossy().to_string();
    p.push_str(".rs");
    PathBuf::from(&p)
}

/// Use this function in your `build.rs` to compile templates.
pub fn compile_templates(files_glob: &str) -> std::io::Result<()> {
    for path in glob::glob(files_glob).expect("unable to read template directory") {
        if let Ok(path) = path {
            let template = std::fs::read_to_string(&path)?;
            let tt = parse::parse_template(&template);
            let mut f = File::create(add_rs_extension(&path))?;
            writeln!(f, "|f: &mut ::std::fmt::Formatter| {{");
            for t in tt.iter() {
                writeln!(f, "{}", As(Rust, t.clone()));
            }
            writeln!(f, "Ok(()) }}");
        }
    }
    Ok(())
}

#[macro_export]
macro_rules! display_as_with_template {
    ($format:ident, $type:ty, $template_name:tt) => {
        impl $crate::display_as::DisplayAs<$format> for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                use display_as::DisplayAs;
                include!(concat!(env!("CARGO_MANIFEST_DIR"),
                                 "/templates/", $template_name, ".rs"))(f)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::compile_templates;
    #[test]
    fn compile_foo_template() {
        compile_templates("templates/foo.html").unwrap();
    }
}
