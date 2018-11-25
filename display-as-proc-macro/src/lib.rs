extern crate proc_macro;
extern crate proc_macro_hack;
// extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

use proc_macro::{TokenStream, TokenTree, Group, Delimiter};
use proc_macro_hack::proc_macro_hack;

fn proc_to_two(i: TokenStream) -> proc_macro2::TokenStream {
    i.into()
}
fn two_to_proc(i: proc_macro2::TokenStream) -> TokenStream {
    i.into()
}

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
    let format = if let Some(format) = tokens.next() {
        proc_to_two(format.into())
    } else {
        panic!("display_as_to_string! needs a Format as its first argument")
    };
    if let Some(comma) = tokens.next() {
        if &comma.to_string() != "," {
            panic!("display_as_to_string! needs a Format followed by a comma, not {}",
                   comma.to_string());
        }
    } else {
        panic!("display_as_to_string! needs a Format followed by a comma");
    }

    let statements = proc_to_two(template_to_statements(&format, tokens.collect()));

    quote!(
        {
            use std::fmt::Write;
            let doit = || -> Result<String, std::fmt::Error> {
                let mut __f = String::new();
                #statements
                Ok(__f)
            };
            doit().expect("trouble writing to String??!")
        }
    ).into()
}

fn expr_toks_to_stmt(format: &proc_macro2::TokenStream, expr: &mut Vec<TokenTree>)
                     -> impl Iterator<Item=TokenTree> {
    if expr.len() > 0 {
        let expr = proc_to_two(expr.drain(..).collect());
        let format = format.clone();
        two_to_proc(quote!{
            __f.write_fmt(format_args!("{}", display_as_template::As(#format, #expr)))?;
        }).into_iter()
    } else {
        two_to_proc(quote!{}).into_iter()
    }
}
fn expr_toks_to_conditional(format: &proc_macro2::TokenStream, expr: &mut Vec<TokenTree>)
                            -> TokenStream {
    expr.drain(..).collect()
}

fn template_to_statements(format: &proc_macro2::TokenStream, template: TokenStream)
                          -> TokenStream {
    let mut toks: Vec<TokenTree> = Vec::new();
    let mut next_expr: Vec<TokenTree> = Vec::new();
    for t in template.into_iter() {
        if let TokenTree::Group(g) = t.clone() {
            if g.delimiter() == Delimiter::Brace {
                toks.extend(expr_toks_to_conditional(&format, &mut next_expr).into_iter());
                toks.push(TokenTree::Group(
                    Group::new(Delimiter::Brace,
                               template_to_statements(format, g.stream()))));
            } else {
                next_expr.push(t);
            }
        } else if is_str(&t) {
            // First print the previous expression...
            toks.extend(expr_toks_to_stmt(&format, &mut next_expr));
            // Now we print this str...
            toks.extend(to_tokens("__f.write_str"));
            toks.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from(t))));
            toks.extend(to_tokens("?;"));
        } else {
            next_expr.push(t);
        }
    }
    // Now print the final expression...
    toks.extend(expr_toks_to_stmt(&format, &mut next_expr));
    TokenTree::Group(Group::new(Delimiter::Brace, toks.into_iter().collect())).into()
}

#[proc_macro_attribute]
pub fn with_template(input: TokenStream, my_impl: TokenStream) -> TokenStream {
    let mut impl_toks: Vec<_> = my_impl.into_iter().collect();
    if &impl_toks[0].to_string() != "impl" || impl_toks.len() < 3 {
        panic!("with_template can only be applied to an impl of DisplayAs");
    }
    let mut my_format: proc_macro2::TokenStream = quote!();
    for i in 0..impl_toks.len()-2 {
        if impl_toks[i].to_string() == "DisplayAs" && impl_toks[i+1].to_string() == "<" {
            my_format = proc_to_two(impl_toks[i+2].clone().into());
            break;
        }
    }
    let last = impl_toks.pop().unwrap();
    if last.to_string() != "{  }" {
        panic!("with_template must be applied to an impl that ends in '{{}}', not {}",
               last.to_string());
    }
    let my_format = my_format; // no longer mut
    let statements = proc_to_two(template_to_statements(&my_format, input));

    let out = quote!{
        {
            #statements
            Ok(())
        }
    };
    let mut new_impl: Vec<TokenTree> = Vec::new();
    new_impl.extend(impl_toks.into_iter());
    new_impl.extend(two_to_proc(quote!{
        {
            fn fmt(&self, __f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                #out
            }
        }
    }).into_iter());
    let new_impl = new_impl.into_iter().collect();

    // println!("new_impl is {}", &new_impl);
    new_impl
}

#[proc_macro_hack]
pub fn display_as_to_rust(input: TokenStream) -> TokenStream {
    input
}

// #[proc_macro_attribute]
// pub fn not_the_bees(_metadata: TokenStream, input: TokenStream) -> TokenStream {
//     input
// }
