use display_as::{with_template, DisplayAs, HTML};

pub struct Page;

#[with_template("page.html")]
impl DisplayAs<HTML> for Page {}
