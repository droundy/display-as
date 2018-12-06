extern crate display_as;

use display_as::{with_template, DisplayAs, As, HTML};

struct Foo {
    name: String,
    age: usize,
}

#[with_template("Foo: " &self.name " with age " self.age)]
impl DisplayAs<HTML> for Foo {}

#[test]
fn foo() {
    assert_eq!(&format!("{}", As(HTML, Foo { name: "David".to_string(),
                                             age: 45 })),
               "Foo: David with age 45");
}

struct TestingIf {
    name: String,
    age: usize,
}

#[with_template(r"TestingIf: "
                if self.age < 18 {
                    r"minor " &self.name
                } else {
                    r"grown-up " &self.name r" who is " self.age r" years old"
                }
                r" (THE END)"
)]
impl DisplayAs<HTML> for TestingIf {}

#[test]
fn testing_if() {
    assert_eq!(&format!("{}", As(HTML, TestingIf { name: "David".to_string(),
                                                   age: 45 })),
               "TestingIf: grown-up David who is 45 years old (THE END)");
    assert_eq!(&format!("{}", As(HTML, TestingIf { name: "Miri".to_string(),
                                                   age: 2 })),
               "TestingIf: minor Miri (THE END)");
}


struct FromFile {
    name: String,
    age: usize,
}

#[with_template("from-file.html")]
impl DisplayAs<HTML> for FromFile {}

#[test]
fn from_file() {
    assert_eq!(&format!("{}", As(HTML, FromFile { name: "David".to_string(),
                                                   age: 45 })),
               "FromFile: grown-up David who is 45 years old (THE END)\n");
    assert_eq!(&format!("{}", As(HTML, FromFile { name: "Miri".to_string(),
                                                   age: 2 })),
               "FromFile: minor Miri (THE END)\n");
}

struct FromFileInclude {
    name: String,
    age: usize,
}

#[with_template("from-file-include.html")]
impl DisplayAs<HTML> for FromFileInclude {}

#[test]
fn from_file_include() {
    assert_eq!(&format!("{}", As(HTML, FromFileInclude { name: "David".to_string(),
                                                         age: 45 })),
               "FromFile: grown-up David who is 45 years old (THE END)\n\n");
    assert_eq!(&format!("{}", As(HTML, FromFileInclude { name: "Miri".to_string(),
                                                         age: 2 })),
               "FromFile: minor Miri (THE END)\n\n");
}


struct FromFileBase {
    name: String,
    age: usize,
}

#[with_template("from-file-base.html")]
impl DisplayAs<HTML> for FromFileBase {}

#[test]
fn from_file_base() {
    assert_eq!(&format!("{}", As(HTML, FromFileBase { name: "David".to_string(),
                                                         age: 45 })),
               "FromFile: grown-up David who is 45 years old (THE END)\n\n");
    assert_eq!(&format!("{}", As(HTML, FromFileBase { name: "Miri".to_string(),
                                                         age: 2 })),
               "FromFile: minor Miri (THE END)\n\n");
}
