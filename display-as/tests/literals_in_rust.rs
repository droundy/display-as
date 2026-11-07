//! String and character literals in the Rust parts of a template (the head
//! of an `if`, `while`, `for`, `match` or `let`) are Rust, not template text.

use display_as::{format_as, with_template, DisplayAs, HTML};

#[test]
fn string_comparison_in_if() {
    for (name, greeting) in [("dave", "hello dave"), ("bob", "hello bob"), ("eve", "who?")] {
        assert_eq!(
            format_as!(HTML,
                       if name == "dave" {
                           "hello dave"
                       } else if name == "bob" {
                           "hello bob"
                       } else {
                           "who?"
                       })
            .into_string(),
            greeting
        );
    }
}

#[test]
fn raw_string_comparison_in_if() {
    let name = "da\"ve";
    assert_eq!(
        format_as!(HTML, if name == r#"da"ve"# { "yes" } else { "no" }).into_string(),
        "yes"
    );
}

#[test]
fn template_text_around_a_condition() {
    let name = "b";
    assert_eq!(
        format_as!(HTML, "x" if name == "b" { "y" } "z").into_string(),
        "xyz"
    );
}

#[test]
fn string_in_let() {
    assert_eq!(
        format_as!(HTML, let greeting = "hello"; greeting " world").into_string(),
        "hello world"
    );
}

#[test]
fn string_in_for_iterator() {
    assert_eq!(
        format_as!(HTML, for c in "abc".chars() { c.to_string() "," }).into_string(),
        "a,b,c,"
    );
}

#[test]
fn string_in_while_condition() {
    let words = ["a", "b", "c"];
    assert_eq!(
        format_as!(HTML,
                   let mut i = 0;
                   while i < words.len() && words[i] != "c" {
                       words[i] ","
                       i += 1;
                   }).into_string(),
        "a,b,"
    );
}

#[test]
fn string_in_match_head() {
    assert_eq!(
        format_as!(HTML, match "abc".len() { 3 => { "three" } _ => { "other" } }).into_string(),
        "three"
    );
}

#[test]
fn char_literal_containing_a_quote() {
    let c = '"';
    assert_eq!(
        format_as!(HTML, if c == '"' { "quote" } else { "plain" }).into_string(),
        "quote"
    );
}

struct Greeting {
    name: &'static str,
}

#[with_template("[%" "%]" "literals_in_rust.html")]
impl DisplayAs<HTML> for Greeting {}

#[test]
fn string_comparison_in_a_template_file() {
    for (name, greeting) in [("David", "Hello David!"), ("Miri", "Hello Miri!"), ("Jo", "Hello stranger!")] {
        assert_eq!(Greeting { name }.display().to_string(), greeting);
    }
}
