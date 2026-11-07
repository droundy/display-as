//! Together with `template-clash-a`, this crate checks that two members of
//! a workspace can have templates with the same name.  If the template is
//! looked for outside of this crate, building the workspace fails.
use display_as::{with_template, DisplayAs, HTML};

pub struct Page {
    pub name: String,
}

#[with_template("index.html")]
impl DisplayAs<HTML> for Page {}

#[test]
fn uses_its_own_index_html() {
    let page = Page { name: "Wilbur".to_string() };
    assert_eq!(
        format!("{}", page.display()),
        "This is crate b's index page for Wilbur!"
    );
}
