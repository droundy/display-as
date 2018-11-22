extern crate proc_macro;
extern crate proc_macro_hack;
// extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::{TokenStream, TokenTree, Group, Delimiter};
use proc_macro_hack::proc_macro_hack;

fn is_str(x: &TokenTree) -> bool {
    match x {
        TokenTree::Literal(_) => {
            let s = x.to_string();
            s.len() > 0 && s.contains("\"") && s.chars().next() != Some('b')
        }
        _ => false
    }
}

fn to_tokens(s: &str) -> impl Iterator<Item=TokenTree> {
    let ts: TokenStream = s.parse().unwrap();
    ts.into_iter()
}

#[proc_macro_hack]
pub fn display_as_to_string(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    if let Some(format) = tokens.next() {
    } else {
        panic!("display_as_to_string! needs a Format as its first argument");
    };
    if let Some(comma) = tokens.next() {
        if &comma.to_string() != "," {
            panic!("display_as_to_string! needs a Format followed by a comma, not {}",
                   comma.to_string());
        }
    } else {
        panic!("display_as_to_string! needs a Format followed by a comma");
    }
    let mut toks: Vec<TokenTree> = Vec::new();
    toks.extend(TokenStream::from(quote!{ use std::fmt::Write; let mut __o = String::new(); }).into_iter());
    let mut next_expr: Vec<TokenTree> = Vec::new();
    for t in tokens {
        if is_str(&t) {
            toks.extend(to_tokens("__o.write_str"));
            toks.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from(t))));
            toks.extend(to_tokens(".unwrap();"));
        }
    }
    toks.extend(to_tokens("__o"));
    let out = TokenStream::from(TokenTree::Group(Group::new(Delimiter::Brace,
                                                            toks.into_iter().collect())));
    println!("out is {}", out);
    out
    // tokens.collect()
}

#[proc_macro_hack]
pub fn display_as_to_rust(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn not_the_bees(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    input
}
