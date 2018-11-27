#![deny(missing_docs)]

//! This is a template library that uses and defines the `DisplayAs`
//! trait defined in the `display-as` crate (also reexported from
//! here).

extern crate display_as;
extern crate display_as_proc_macro;
extern crate proc_macro_hack;

use proc_macro_hack::proc_macro_hack;
#[proc_macro_hack]
pub use display_as_proc_macro::{display_as_to_string};

/// Can I write doc here?
pub use display_as_proc_macro::{with_template};

pub use display_as::{DisplayAs, As, HTML, Rust};
