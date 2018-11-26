#![deny(missing_docs)]

//! This will be a template library that uses `display-as`.

extern crate display_as;
extern crate display_as_proc_macro;
extern crate proc_macro_hack;

use proc_macro_hack::proc_macro_hack;
#[proc_macro_hack]
pub use display_as_proc_macro::{display_as_to_string};

/// Can I write doc here?
pub use display_as_proc_macro::{with_template};

pub use display_as::{DisplayAs, As, HTML, Rust};
