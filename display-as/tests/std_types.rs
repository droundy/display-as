//! `bool`, `char` and the string-like pointer types can be used in templates,
//! by value, by reference and by double reference, in every built-in format.

use display_as::{format_as, LaTeX, Math, Rust, HTML, URL, UTF8};
use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

/// Generate a test module for one format.  `text` is how the string
/// `"a&b c"` is displayed, and `amp`, `space` and `accent` how the characters
/// `'&'`, `' '` and `'é'` are.
macro_rules! check_format {
    ($module:ident, $format:ident, text: $text:expr, amp: $amp:expr, space: $space:expr, accent: $accent:expr) => {
        mod $module {
            use super::*;

            #[test]
            fn bools() {
                let t = true;
                let f = false;
                assert_eq!(format_as!($format, t).into_string(), "true");
                assert_eq!(format_as!($format, f).into_string(), "false");
                assert_eq!(format_as!($format, (&t)).into_string(), "true");
                assert_eq!(format_as!($format, (&f)).into_string(), "false");
                assert_eq!(format_as!($format, (&&t)).into_string(), "true");
                assert_eq!(format_as!($format, (&&f)).into_string(), "false");
                assert_eq!(
                    format_as!($format, "is " (1 < 2) ", not " (2 < 1)).into_string(),
                    "is true, not false"
                );
            }

            #[test]
            fn chars() {
                let amp = '&';
                assert_eq!(format_as!($format, amp).into_string(), $amp);
                assert_eq!(format_as!($format, (&amp)).into_string(), $amp);
                assert_eq!(format_as!($format, (&&amp)).into_string(), $amp);
                let space = ' ';
                assert_eq!(format_as!($format, space).into_string(), $space);
                assert_eq!(format_as!($format, (&space)).into_string(), $space);
                assert_eq!(format_as!($format, (&&space)).into_string(), $space);
                let accent = 'é';
                assert_eq!(format_as!($format, accent).into_string(), $accent);
                assert_eq!(format_as!($format, (&accent)).into_string(), $accent);
                assert_eq!(format_as!($format, (&&accent)).into_string(), $accent);
            }

            #[test]
            fn cow_str() {
                let borrowed: Cow<str> = Cow::Borrowed("a&b c");
                let owned: Cow<str> = Cow::Owned("a&b c".to_string());
                assert_eq!(format_as!($format, borrowed).into_string(), $text);
                assert_eq!(format_as!($format, (&borrowed)).into_string(), $text);
                assert_eq!(format_as!($format, (&&borrowed)).into_string(), $text);
                assert_eq!(format_as!($format, owned).into_string(), $text);
                assert_eq!(format_as!($format, (&owned)).into_string(), $text);
                assert_eq!(format_as!($format, (&&owned)).into_string(), $text);
                assert_eq!(
                    format_as!($format, (Cow::Borrowed("a&b c"))).into_string(),
                    $text
                );
            }

            #[test]
            fn boxed_and_shared_str() {
                let boxed: Box<str> = "a&b c".into();
                let rc: Rc<str> = "a&b c".into();
                let arc: Arc<str> = "a&b c".into();
                assert_eq!(format_as!($format, (&boxed)).into_string(), $text);
                assert_eq!(format_as!($format, (&&boxed)).into_string(), $text);
                assert_eq!(format_as!($format, boxed).into_string(), $text);
                assert_eq!(format_as!($format, (&rc)).into_string(), $text);
                assert_eq!(format_as!($format, (&&rc)).into_string(), $text);
                assert_eq!(format_as!($format, rc).into_string(), $text);
                assert_eq!(format_as!($format, (&arc)).into_string(), $text);
                assert_eq!(format_as!($format, (&&arc)).into_string(), $text);
                assert_eq!(format_as!($format, arc).into_string(), $text);
            }

            /// These worked before `bool` and `char` were added, because
            /// `format_as!` auto-dereferences them to a `String` or integer.
            #[test]
            fn rc_and_arc_of_sized() {
                assert_eq!(
                    format_as!($format, (Rc::new("a&b c".to_string()))).into_string(),
                    $text
                );
                assert_eq!(
                    format_as!($format, (Arc::new("a&b c".to_string()))).into_string(),
                    $text
                );
                assert_eq!(format_as!($format, (Rc::new(true))).into_string(), "true");
                assert_eq!(format_as!($format, (Arc::new('&'))).into_string(), $amp);
            }
        }
    };
}

check_format!(html, HTML, text: "a&amp;b c", amp: "&amp;", space: " ", accent: "é");
check_format!(latex, LaTeX, text: r"a\&b c", amp: r"\&", space: " ", accent: "é");
check_format!(math, Math, text: r"a\&b c", amp: r"\&", space: " ", accent: "é");
check_format!(url, URL, text: "a&b%20c", amp: "&", space: "%20", accent: "%C3%A9");
check_format!(utf8, UTF8, text: "a&b c", amp: "&", space: " ", accent: "é");
check_format!(rust, Rust, text: r#""a&b c""#, amp: "'&'", space: "' '", accent: "'é'");

#[test]
fn rust_chars_are_escaped_like_rust_literals() {
    assert_eq!(format_as!(Rust, '\'').into_string(), r"'\''");
    // (a variable, because `format_as!` mistakes a literal `'"'` for a string)
    let quote = '"';
    assert_eq!(format_as!(Rust, quote).into_string(), r#"'"'"#);
    assert_eq!(format_as!(Rust, '\n').into_string(), r"'\n'");
    assert_eq!(format_as!(Rust, '\\').into_string(), r"'\\'");
}

#[test]
fn html_chars_are_escaped() {
    assert_eq!(format_as!(HTML, '<').into_string(), "&lt;");
    assert_eq!(format_as!(HTML, '\'').into_string(), "&#x27;");
}

#[test]
fn bool_is_not_quoted_in_rust() {
    assert_eq!(format_as!(Rust, "let x = " true ";").into_string(), "let x = true;");
}
