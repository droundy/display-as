//! Templates with the same file name in different directories.  Each is
//! found next to the source file that uses it.

use display_as::{format_as, HTML};

#[path = "same_name_a/mod.rs"]
mod a;
#[path = "same_name_b/mod.rs"]
mod b;

#[test]
fn same_template_name_in_different_directories() {
    assert_eq!(&format_as!(HTML, (a::Page)).into_string(), "page a");
    assert_eq!(&format_as!(HTML, (b::Page)).into_string(), "page b");
}
