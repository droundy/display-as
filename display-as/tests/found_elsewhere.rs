//! A template that is not next to the source file using it is found by
//! searching the source directories for it.  The file is then tracked from
//! the source file that uses it, so the template still builds, including any
//! templates it includes.

use display_as::{format_as, HTML};

#[path = "found_elsewhere_user/mod.rs"]
mod user;

#[test]
fn template_found_in_another_source_directory() {
    assert_eq!(&format_as!(HTML, (user::Page)).into_string(), "found part");
}
