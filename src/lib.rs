#![deny(missing_docs)]

//! This will be a template library that uses `display-as`.

extern crate display_as;
extern crate glob;
#[macro_use]
extern crate combine;

mod parse;

/// Use this function in your `build.rs` to compile templates.
pub fn compile_templates(files_glob: &str) -> std::io::Result<()> {
    for path in glob::glob(files_glob).expect("unable to read template directory") {
        if let Ok(path) = path {
            let template = std::fs::read_to_string(&path)?;
            println!("template {:?}: {}", &path, template);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::compile_templates;
    #[test]
    fn it_works() {
        compile_templates("tests/foo.html").unwrap();
    }
}
