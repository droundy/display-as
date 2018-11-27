#![deny(missing_docs)]

//! This is a template library that uses and defines the `DisplayAs`
//! trait defined in the `display-as` crate (also reexported from
//! here).

extern crate display_as;

extern crate display_as_proc_macro;

extern crate proc_macro_hack;

extern crate glob;
#[macro_use]
extern crate combine;
extern crate regex;

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use proc_macro_hack::proc_macro_hack;
#[proc_macro_hack]
pub use display_as_proc_macro::{display_as_to_string};

/// Can I write doc here?
pub use display_as_proc_macro::{with_template};

pub use display_as::{DisplayAs, As, HTML, Rust};

mod parse;
mod rust;

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

/// Use this function in your `build.rs` to compile templates.
pub fn compile_all_templates() -> std::io::Result<()> {
    let mut sources: Vec<_> = glob::glob("src/**/*.rs").expect("unable to read src directory").collect();
    sources.extend(glob::glob("tests/**/*.rs").expect("unable to read src directory"));
    for rs in sources {
        if let Ok(rs) = rs {
            let code = std::fs::read_to_string(&rs)?;
            println!("Looking inside {:?}", rs);
            for r in rust::parse_rust(&code) {
                println!("looking at {:?}", r);
                let reversed_name: String = r.template_name.chars().rev().collect();
                let template_name: String =
                    reversed_name.replacen("_", ".", 1).chars().rev().collect();
                let template_name = template_name.replace(".rs", "");
                let mut template_path = PathBuf::from(rs.parent().unwrap());
                template_path.push(&template_name);
                println!("reading file {:?}", &template_path);
                let template = std::fs::read_to_string(&template_path)?;
                let tt = parse::parse_template(&template);
                let mut newcode_path = PathBuf::from(rs.parent().unwrap());
                newcode_path.push(&r.template_name);
                println!("about to create {:?}", &newcode_path);
                let mut f = File::create(&newcode_path)?;
                writeln!(f, "{} {{", r.impl_text);
                writeln!(f, "  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {{");
                for t in tt.iter() {
                    writeln!(f, "{}", As(Rust, t.clone()));
                }
                writeln!(f, "Ok(()) }}\n}}");
            }
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

#[macro_export]
macro_rules! display_as_template {
    ($format:ident; $impl:stmt; $template_name:tt) => {
        include!($template_name);
    }
}

