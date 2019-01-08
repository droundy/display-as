extern crate display_as;

use display_as::{with_template, DisplayAs, HTML, URL, format_as};

struct Foo {
    name: String,
    age: usize,
}

#[with_template("Foo: " &self.name " with age " self.age)]
impl DisplayAs<HTML> for Foo {}

#[test]
fn foo() {
    assert_eq!(
        &format!(
            "{}",
            Foo {
                name: "David".to_string(),
                age: 45
            }
            .display()
        ),
        "Foo: David with age 45"
    );
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
    assert_eq!(
        &format!(
            "{}",
            TestingIf {
                name: "David".to_string(),
                age: 45
            }
            .display()
        ),
        "TestingIf: grown-up David who is 45 years old (THE END)"
    );
    assert_eq!(
        &format!(
            "{}",
            TestingIf {
                name: "Miri".to_string(),
                age: 2
            }
            .display()
        ),
        "TestingIf: minor Miri (THE END)"
    );
}

struct FromFile {
    name: String,
    age: usize,
}

#[with_template("from-file.html")]
impl DisplayAs<HTML> for FromFile {}

#[test]
fn from_file() {
    assert_eq!(
        &format!(
            "{}",
            FromFile {
                name: "David".to_string(),
                age: 45
            }
            .display()
        ),
        "FromFile: grown-up David who is 45 years old (THE END)\n"
    );
    assert_eq!(
        &format!(
            "{}",
            FromFile {
                name: "Miri".to_string(),
                age: 2
            }
            .display()
        ),
        "FromFile: minor Miri (THE END)\n"
    );
}

struct FromFileInclude {
    name: String,
    age: usize,
}

#[with_template("from-file-include.html")]
impl DisplayAs<HTML> for FromFileInclude {}

#[test]
fn from_file_include() {
    assert_eq!(
        &format!(
            "{}",
            FromFileInclude {
                name: "David".to_string(),
                age: 45
            }
            .display()
        ),
        "FromFile: grown-up David who is 45 years old (THE END)\n\n"
    );
    assert_eq!(
        &format!(
            "{}",
            FromFileInclude {
                name: "Miri".to_string(),
                age: 2
            }
            .display()
        ),
        "FromFile: minor Miri (THE END)\n\n"
    );
}

struct FromFileBase {
    name: String,
    age: usize,
}

#[with_template("from-file-base.html")]
impl DisplayAs<HTML> for FromFileBase {}
#[with_template("url/" self.name)]
impl DisplayAs<URL> for FromFileBase {}

#[test]
fn from_file_base() {
    assert_eq!(
        &format_as!(HTML, (FromFileBase {
            name: "David".to_string(),
            age: 45
        })),
        "FromFile: grown-up url/David who is 45 years old (THE END)\n\n"
    );
    assert_eq!(
        &format_as!(HTML, (FromFileBase {
            name: "Miri".to_string(),
            age: 2
        })),
        "FromFile: minor url/Miri (THE END)\n\n"
    );
}


struct Delimiters {
    name: String,
    age: usize,
}
#[with_template("}}" "{{" "delimiters.html")]
impl DisplayAs<HTML> for Delimiters {}

#[test]
fn from_delimiters() {
    assert_eq!(
        &format!(
            "{}",
            Delimiters {
                name: "David".to_string(),
                age: 45
            }
            .display()
        ),
        "FromFile: grown-up David who is 45 years old (THE END)\n"
    );
    assert_eq!(
        &format!(
            "{}",
            Delimiters {
                name: "Miri".to_string(),
                age: 2
            }
            .display()
        ),
        "FromFile: minor Miri (THE END)\n"
    );
}
