//! This is the implementation crate for `display-as-template`.

extern crate proc_macro;
extern crate proc_macro_hack;
// extern crate syn;
#[macro_use]
extern crate quote;
extern crate glob;
extern crate proc_macro2;

use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use proc_macro_hack::proc_macro_hack;
use std::fmt::Write;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

fn find_template_file(path: &str) -> PathBuf {
    let sourcedirs: std::collections::HashSet<_> = glob::glob("**/*.rs").unwrap()
        .flat_map(|x| x.ok())
        .filter(|x| !x.starts_with("target/"))
        .map(|x| PathBuf::from(x.clone().parent().unwrap()))
        .collect();
    let paths: Vec<_> = sourcedirs.into_iter()
        .filter(|d| d.join(path).exists())
        .collect();
    if paths.len() == 0 {
        panic!("No template file named {:?} exists.", path);
    } else if paths.len() > 1 {
        panic!(r"Multiple files named {:?} exist.  Eventually display-as will
    support this, but for now each template file must have a unique name.", path);
    }
    paths.into_iter().next().unwrap()
}

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
        _ => false,
    }
}

fn to_tokens(s: &str) -> impl Iterator<Item = TokenTree> {
    let ts: TokenStream = s.parse().unwrap();
    ts.into_iter()
}

fn count_pounds(x: &str) -> &'static str {
    for pounds in &["#######", "######", "#####", "####", "###", "##", "#", ""] {
        if x.contains(pounds) {
            return pounds;
        }
    }
    ""
}

/// Use the given template to create a string.
///
/// You can think of this as being kind of like `format!` on strange drugs.
#[proc_macro_hack]
pub fn format_as(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let format = if let Some(format) = tokens.next() {
        proc_to_two(format.into())
    } else {
        panic!("format_as! needs a Format as its first argument")
    };
    if let Some(comma) = tokens.next() {
        if &comma.to_string() != "," {
            panic!(
                "format_as! needs a Format followed by a comma, not {}",
                comma.to_string()
            );
        }
    } else {
        panic!("format_as! needs a Format followed by a comma");
    }

    let statements = proc_to_two(template_to_statements(
        "templates".as_ref(),
        &format,
        tokens.collect(), "", ""
    ));

    quote!(
        {
            use std::fmt::Write;
            use $crate::DisplayAs;
            let doit = || -> Result<String, std::fmt::Error> {
                let mut __f = String::new();
                #statements
                Ok(__f)
            };
            doit().expect("trouble writing to String??!")
        }
    )
    .into()
}

/// Write the given template to a file.
///
/// You can think of this as being kind of like `write!` on strange drugs.
#[proc_macro_hack]
pub fn write_as(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let format = if let Some(format) = tokens.next() {
        proc_to_two(format.into())
    } else {
        panic!("write_as! needs a Format as its first argument")
    };
    if let Some(comma) = tokens.next() {
        if &comma.to_string() != "," {
            panic!(
                "write_as! needs a Format followed by a comma, not {}",
                comma.to_string()
            );
        }
    } else {
        panic!("write_as! needs a Format followed by a comma");
    }

    let mut writer: Vec<TokenTree> = Vec::new();
    while let Some(tok) = tokens.next() {
        if &tok.to_string() == "," {
            break;
        } else {
            writer.push(tok);
        }
    }
    if writer.len() == 0 {
        panic!("write_as! needs a Writer as its second argument followed by comma.")
    }
    let writer = proc_to_two(writer.into_iter().collect());

    let statements = proc_to_two(template_to_statements(
        "templates".as_ref(),
        &format,
        tokens.collect(), "", ""
    ));

    quote!(
        {
            use std::fmt::Write;
            use $crate::DisplayAs;
            let __f = &mut #writer;
            let mut doit = || -> Result<(), std::fmt::Error> {
                #statements
                Ok(())
            };
            doit()
        }
    )
    .into()
}

fn expr_toks_to_stmt(
    format: &proc_macro2::TokenStream,
    expr: &mut Vec<TokenTree>,
) -> impl Iterator<Item = TokenTree> {
    let len = expr.len();
    if len > 2 && expr[len - 2].to_string() == "as" {
        let format = proc_to_two(expr.pop().unwrap().into());
        expr.pop();
        let expr = proc_to_two(expr.drain(..).collect());
        two_to_proc(quote! {
            {
                trait ToDisplayAs {
                    fn to_display_as(&self) -> &Self;
                }
                impl<T: DisplayAs<#format>> ToDisplayAs for T {
                    fn to_display_as(&self) -> &Self { self }
                }
                __f.write_fmt(format_args!("{}", <_ as DisplayAs<#format>>::display((#expr).to_display_as())))?;
            }
        })
        .into_iter()
    } else if expr.len() > 0 {
        let expr = proc_to_two(expr.drain(..).collect());
        let format = format.clone();
        two_to_proc(quote! {
            {
                trait ToDisplayAs {
                    fn to_display_as(&self) -> &Self;
                }
                impl<T: DisplayAs<#format>> ToDisplayAs for T {
                    fn to_display_as(&self) -> &Self { self }
                }
                __f.write_fmt(format_args!("{}", <_ as DisplayAs<#format>>::display((#expr).to_display_as())))?;
            }
        })
        .into_iter()
    } else {
        two_to_proc(quote! {}).into_iter()
    }
}
fn expr_toks_to_conditional(expr: &mut Vec<TokenTree>) -> TokenStream {
    expr.drain(..).collect()
}

fn read_template_file(dirname: &Path, pathname: &str,
                      left_delim: &str, right_delim: &str) -> TokenStream {
    let path = dirname.join(&pathname);
    if let Ok(mut f) = File::open(&path) {
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        let raw_template_len = contents.len();
        let pounds: String = if left_delim == "" {
            count_pounds(&contents).to_string()
        } else {
            let mut pounds = count_pounds(&contents).to_string();
            pounds.write_str("#").unwrap();
            contents = contents.replace(left_delim, &format!(r#""{}"#, pounds));
            contents = contents.replace(right_delim, &format!(r#"r{}""#, pounds));
            pounds
        };
        contents.write_str("\"").unwrap();
        contents.write_str(&pounds).unwrap();
        let mut template = "r".to_string();
        template.write_str(&pounds).unwrap();
        template.write_str("\"").unwrap();
        template.write_str(&contents).unwrap();
        template
            .write_str("  ({ assert_eq!(include_str!(\"")
            .unwrap();
        template.write_str(&pathname).unwrap();
        write!(template, "\").len(), {}); \"\"}}); ", raw_template_len).unwrap();
        template.parse().expect("trouble parsing file")
    } else {
        panic!("No such file: {}", path.display())
    }
}

fn template_to_statements(
    dir: &Path,
    format: &proc_macro2::TokenStream,
    template: TokenStream,
    left_delim: &str,
    right_delim: &str) -> TokenStream
{
    let mut toks: Vec<TokenTree> = Vec::new();
    let mut next_expr: Vec<TokenTree> = Vec::new();
    for t in template.into_iter() {
        if let TokenTree::Group(g) = t.clone() {
            let next_expr_len = next_expr.len();
            if g.delimiter() == Delimiter::Brace {
                if next_expr_len > 2
                    && !next_expr.iter().any(|x| x.to_string() == "=")
                    && &next_expr[0].to_string() == "if"
                    && &next_expr[1].to_string() == "let"
                {
                    // We presumably are looking at a destructuring
                    // pattern.
                    next_expr.push(t);
                } else if next_expr_len > 1
                    && &next_expr[next_expr_len - 1].to_string() != "="
                    && &next_expr[0].to_string() == "let"
                {
                    // We presumably are looking at a destructuring
                    // pattern.
                    next_expr.push(t);
                } else if next_expr_len > 2
                    && &next_expr[next_expr_len - 1].to_string() == "="
                    && &next_expr[0].to_string() == "let"
                {
                    // We are doing an assignment to a template
                    // thingy, so let's create a DisplayAs thingy
                    // rather than adding the stuff right now.
                    toks.extend(expr_toks_to_conditional(&mut next_expr).into_iter());
                    let actions = proc_to_two(template_to_statements(dir, format, g.stream(),
                                                                     left_delim, right_delim));
                    toks.extend(
                        two_to_proc(quote! {
                            display_as::display_closure_as(#format, |__f: &mut ::std::fmt::Formatter|
                                 -> Result<(), ::std::fmt::Error> {
                                { #actions };
                                Ok(())
                            })
                            // |_format: #format, __f: &mut ::std::fmt::Formatter|
                            //      -> Result<(), ::std::fmt::Error> {
                            //     { #actions };
                            //     Ok(())
                            // }
                        })
                        .into_iter(),
                    );
                } else if next_expr_len > 0 && &next_expr[0].to_string() == "match" {
                    toks.extend(expr_toks_to_conditional(&mut next_expr).into_iter());
                    let mut interior_toks: Vec<TokenTree> = Vec::new();
                    for x in g.stream() {
                        if let TokenTree::Group(g) = x.clone() {
                            if g.delimiter() == Delimiter::Brace {
                                interior_toks.push(TokenTree::Group(Group::new(
                                    Delimiter::Brace,
                                    template_to_statements(dir, format, g.stream(),
                                                           left_delim, right_delim),
                                    )));
                            } else {
                                interior_toks.push(x);
                            }
                        } else {
                            interior_toks.push(x);
                        }
                    }
                    toks.push(TokenTree::Group(Group::new(Delimiter::Brace,
                                                          interior_toks.into_iter().collect())));
                } else {
                    toks.extend(expr_toks_to_conditional(&mut next_expr).into_iter());
                    toks.push(TokenTree::Group(Group::new(
                        Delimiter::Brace,
                        template_to_statements(dir, format, g.stream(),
                                               left_delim, right_delim),
                    )));
                }
            } else if g.delimiter() == Delimiter::Parenthesis
                && next_expr.len() >= 2
                && &next_expr[next_expr_len - 1].to_string() == "!"
                && &next_expr[next_expr_len - 2].to_string() == "include"
            {
                next_expr.pop();
                next_expr.pop(); // remove the include!
                let filenames: Vec<_> = g.stream().into_iter().collect();
                if filenames.len() != 1 {
                    panic!(
                        "include! macro within a template must have one argument, a string literal"
                    );
                }
                let filename = filenames[0].to_string().replace("\"", "");
                let templ = read_template_file(dir, &filename, left_delim, right_delim);
                let statements = template_to_statements(dir, format, templ,
                                                        left_delim, right_delim);
                next_expr.extend(statements.into_iter());
                next_expr.extend(to_tokens(";").into_iter());
                toks.extend(expr_toks_to_conditional(&mut next_expr).into_iter());
                toks.push(t);
            } else {
                next_expr.push(t);
            }
        } else if t.to_string() == ";" {
            toks.extend(expr_toks_to_conditional(&mut next_expr).into_iter());
            toks.push(t);
        } else if is_str(&t) {
            // First print the previous expression...
            toks.extend(expr_toks_to_stmt(&format, &mut next_expr));
            // Now we print this str...
            toks.extend(to_tokens("__f.write_str"));
            toks.push(TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                TokenStream::from(t),
            )));
            toks.extend(to_tokens("?;"));
        } else {
            next_expr.push(t);
        }
    }
    // Now print the final expression...
    toks.extend(expr_toks_to_stmt(&format, &mut next_expr));
    TokenTree::Group(Group::new(Delimiter::Brace, toks.into_iter().collect())).into()
}

/// Implement `DisplayAs` for a given type.
///
/// Why not use `derive`? Because we need to be able to specify which
/// format we want to implement, and we might want to also use
/// additional generic bounds.
///
/// You may use `with_template` in two different ways: inline or with
/// a separate template file.  To use an inline template, you provide
/// your template as an argument, as in `#[with_template("Vec(" self.x
/// "," self.y "," self.z ",")]`.  The template consists of
/// alternating strings and expressions, although you can also use if
/// statements, for loops, or match expressions, although match
/// expressions must use curly braces on each branch.
///
/// A template file is specified by giving the path relative to the
/// current source file as a string argument:
/// `#[with_template("filename.html")]`.  There are a few hokey
/// restrictions on your filenames.
///
/// 1. Your filename cannot have an embedded `"` character.
/// 2. Your string specifying the filename cannot be a "raw" string.
/// 3. You cannot use any characters (including a backslash) that need escaping in rust strings.
///
/// These constraints are very hokey, and may be lifted in the future.
/// File a bug report if you have a good use for lifting these
/// constraints.
///
/// The file itself will have a template like those above, but without
/// the beginning or ending quotation marks.  Furthermore, it is
/// assumed that you are using raw strings, and that you use an equal
/// number of `#` signs throughout.
///
/// You may also give **three** strings to `with_template`, in which
/// case the first two strings are the left and right delimiters for
/// rust content.  This can make your template files a little easier
/// to read.
#[proc_macro_attribute]
pub fn with_template(input: TokenStream, my_impl: TokenStream) -> TokenStream {
    let mut sourcedir = PathBuf::from(".");

    let mut impl_toks: Vec<_> = my_impl.into_iter().collect();
    if &impl_toks[0].to_string() != "impl" || impl_toks.len() < 3 {
        panic!("with_template can only be applied to an impl of DisplayAs");
    }
    let mut my_format: proc_macro2::TokenStream = quote!();
    for i in 0..impl_toks.len() - 2 {
        if impl_toks[i].to_string() == "DisplayAs" && impl_toks[i + 1].to_string() == "<" {
            my_format = proc_to_two(impl_toks[i + 2].clone().into());
            break;
        }
    }
    let last = impl_toks.pop().unwrap();
    if last.to_string() != "{  }" {
        panic!(
            "with_template must be applied to an impl that ends in '{{}}', not {}",
            last.to_string()
        );
    }
    let my_format = my_format; // no longer mut

    let input_vec: Vec<_> = input.clone().into_iter().collect();
    let mut left_delim = "".to_string();
    let mut right_delim = "".to_string();
    let input = if input_vec.len() == 1 {
        let pathname = input_vec[0].to_string().replace("\"", "");
        sourcedir = find_template_file(&pathname);
        read_template_file(&sourcedir, &pathname, "", "")
    } else if input_vec.len() == 3
        && input_vec[0].to_string().contains("\"")
        && input_vec[1].to_string().contains("\"")
        && input_vec[2].to_string().contains("\"")
    {
        // If we have three string literals, the first two are the
        // delimiters we want to use.
        let pathname = input_vec[2].to_string().replace("\"", "");
        sourcedir = find_template_file(&pathname);
        left_delim = input_vec[0].to_string().replace("\"", "");
        right_delim = input_vec[1].to_string().replace("\"", "");
        read_template_file(&sourcedir, &pathname, &left_delim, &right_delim)
    } else {
        input
    };
    let statements = proc_to_two(template_to_statements(&sourcedir, &my_format, input,
                                                        &left_delim, &right_delim));

    let out = quote! {
        {
            #statements
            Ok(())
        }
    };
    let mut new_impl: Vec<TokenTree> = Vec::new();
    new_impl.extend(impl_toks.into_iter());
    new_impl.extend(
        two_to_proc(quote! {
            {
                fn fmt(&self, __f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                    #out
                }
            }
        })
        .into_iter(),
    );
    let new_impl = new_impl.into_iter().collect();

    // println!("new_impl is {}", &new_impl);
    new_impl
}

/// Like [with_template], but also generate any web responder
/// implementations that are handled via feature flags.
#[proc_macro_attribute]
pub fn with_response_template(input: TokenStream, my_impl: TokenStream) -> TokenStream {
    let displayas_impl = with_template(input, my_impl.clone());
    displayas_impl
}
