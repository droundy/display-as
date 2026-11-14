//! This is the implementation crate for `display-as-template`.

extern crate proc_macro;
// extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use std::collections::HashSet;
use std::fmt::Write;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

/// Is `dir` a linked git worktree?  Its `.git` is then a file pointing
/// into the `.git/worktrees` directory of the repository it was made
/// from (a submodule's `.git` file points into `.git/modules` instead).
fn is_linked_worktree(dir: &Path) -> bool {
    match fs::read_to_string(dir.join(".git")) {
        Ok(contents) => contents.starts_with("gitdir:") && contents.contains("/worktrees/"),
        Err(_) => false,
    }
}

/// Is the directory `path` somewhere that never holds the sources of the
/// crate being compiled, and so is not worth searching for templates?
///
/// That is true of hidden directories (version control and tool state,
/// like `.git` or `.claude`), of cargo's `target` directory, which can
/// hold tens of thousands of files that we would otherwise visit for every
/// template, and of other git worktrees, which each hold a copy of every
/// template.
fn is_unsearched_dir(path: &Path) -> bool {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy())
        .unwrap_or_default();
    name.starts_with('.')
        || (name == "target" && path.with_file_name("Cargo.toml").exists())
        || is_linked_worktree(path)
}

/// Add every directory under `dir` (including `dir` itself) that
/// contains a Rust source file to `found`.  An empty `dir` means the
/// current directory, and gives paths that are relative to it.
fn collect_source_dirs(dir: &Path, found: &mut HashSet<PathBuf>) {
    let listed = if dir.as_os_str().is_empty() {
        Path::new(".")
    } else {
        dir
    };
    let entries = match fs::read_dir(listed) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    let mut has_rust_source = false;
    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        let path = dir.join(&file_name);
        let is_dir = match entry.file_type() {
            Ok(t) if t.is_symlink() => path.is_dir(),
            Ok(t) => t.is_dir(),
            Err(_) => false,
        };
        if is_dir {
            if !is_unsearched_dir(&path) {
                collect_source_dirs(&path, found);
            }
        } else if name.ends_with(".rs") {
            has_rust_source = true;
        }
    }
    if has_rust_source {
        found.insert(dir.to_path_buf());
    }
}

/// The directory of the source file that contains the macro invocation
/// being expanded, if the compiler can tell us.
fn invoking_dir() -> Option<PathBuf> {
    let file = proc_macro::Span::call_site().local_file()?;
    file.parent().map(Path::to_path_buf)
}

fn find_template_file(path: &str) -> PathBuf {
    find_template_dir(invoking_dir(), path)
}

/// Find the directory holding the template file `path`.  The generated code
/// reads the template with `include_str!`, which looks next to the source file
/// containing the macro, so if we know which file that is (`invoking`), that
/// is where the template is.  Otherwise search the tree for it.
fn find_template_dir(invoking: Option<PathBuf>, path: &str) -> PathBuf {
    if let Some(dir) = invoking {
        if dir.join(path).exists() {
            return dir;
        }
    }
    let mut sourcedirs = HashSet::new();
    collect_source_dirs(Path::new(""), &mut sourcedirs);
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

/// Is `x` a string literal (normal or raw), as opposed to a byte string, a
/// character literal such as `'"'`, or a number?
fn is_str(x: &TokenTree) -> bool {
    match x {
        TokenTree::Literal(_) => {
            let s = x.to_string();
            s.starts_with('"') || s.starts_with("r\"") || s.starts_with("r#")
        }
        _ => false,
    }
}

/// Is `expr`, the Rust code collected since the last template string or
/// statement, the start of an `if`, `while`, `for`, `match` or `let` (or of an
/// `else if`)?  If so, a string literal that follows is part of that Rust code
/// (a condition, an iterator or an initial value) rather than template text.
fn starts_statement(expr: &[TokenTree]) -> bool {
    match expr.first() {
        Some(TokenTree::Ident(ident)) => {
            ["if", "while", "for", "match", "let", "else"].contains(&ident.to_string().as_str())
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

/// Parse the `Format,` that starts the input of `format_as!` and
/// `write_as!`, consuming those tokens.
fn parse_format(
    macro_name: &str,
    tokens: &mut impl Iterator<Item = TokenTree>,
) -> proc_macro2::TokenStream {
    let format = tokens
        .next()
        .unwrap_or_else(|| panic!("{}! needs a Format as its first argument", macro_name));
    match tokens.next() {
        Some(comma) if comma.to_string() == "," => {}
        Some(comma) => panic!(
            "{}! needs a Format followed by a comma, not {}",
            macro_name, comma
        ),
        None => panic!("{}! needs a Format followed by a comma", macro_name),
    }
    proc_to_two(format.into())
}

/// Use the given template to create a `FormattedString`.
///
/// You can think of this as being kind of like `format!` on strange drugs.
/// We return a `FormattedString` instead of a `String` so that you can store
/// the output and use it later in another template without having the
/// contents escaped.  To obtain a `String`, use the
/// `FormattedString::into_string` method.
#[proc_macro]
pub fn format_as(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let format = parse_format("format_as", &mut tokens);

    let statements = proc_to_two(template_to_statements(
        "templates".as_ref(),
        &format,
        tokens.collect(), "", ""
    ));

    quote!(
        {
            use std::fmt::Write;
            use display_as::DisplayAs;
            let doit = || -> Result<String, std::fmt::Error> {
                let mut __f = String::with_capacity(32);
                #statements
                Ok(__f)
            };
            display_as::FormattedString::<#format>::from_formatted(doit().expect("trouble writing to String??!"))
        }
    )
    .into()
}

/// Write the given template to anything that implements `std::fmt::Write`,
/// such as a `String`.
///
/// You can think of this as being kind of like `write!` on strange drugs.
#[proc_macro]
pub fn write_as(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let format = parse_format("write_as", &mut tokens);

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
            use display_as::DisplayAs;
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

/// A deterministic 64-bit hash (FNV-1a) of a template.
///
/// We deliberately do not use `std::collections::hash_map::DefaultHasher`
/// (whose algorithm is not guaranteed to stay the same between Rust
/// releases) or anything random: the generated code must be identical every
/// time we are given the same input, so that builds are reproducible and
/// incremental compilation is not defeated.
fn hash_template(template: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in template.bytes() {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// The name of the helper method used by `expr_toks_to_stmt` for the
/// template `template` (the tokens of one level of a template: the whole
/// input of the macro, or a sub-template such as the body of a `let x = {
/// ... };`, a `match` arm or an `include!`d file).
///
/// The name needs to be unique among nested calls to `format_as!`.  If an
/// expression in a template contains a nested `format_as!`, both the outer
/// helper trait and the inner one are in scope for the inner expressions,
/// and two methods with the same name would be ambiguous.  The tokens of a
/// nested invocation are strictly fewer than (and a part of) those of the
/// invocation containing it, so a hash of the template tokens differs for
/// every level of nesting.  Sibling expressions in one template are never
/// nested in each other, so they may share a name.
fn to_display_as_name(
    format: &proc_macro2::TokenStream,
    template: &TokenStream,
) -> proc_macro2::Ident {
    let hash = hash_template(&template.to_string());
    proc_macro2::Ident::new(
        &format!("ToDisplayAs{}xxx{:016x}", format, hash),
        proc_macro2::Span::call_site(),
    )
}

fn expr_toks_to_stmt(
    to_display_as: &proc_macro2::Ident,
    format: &proc_macro2::TokenStream,
    expr: &mut Vec<TokenTree>,
) -> impl Iterator<Item = TokenTree> {
    let len = expr.len();
    if len == 0 {
        return two_to_proc(quote! {}).into_iter();
    }

    // An `as Format` suffix overrides the format of the template.
    let format = if len > 2 && expr[len - 2].to_string() == "as" {
        let format = proc_to_two(expr.pop().unwrap().into());
        expr.pop();
        format
    } else {
        format.clone()
    };
    let expr = proc_to_two(expr.drain(..).collect());

    // The ToDisplayAs type below is my hokey approach to use deref coersion
    // (which happens on method calls) ensure that either references to
    // DisplayAs types or the types themselves can be used.
    two_to_proc(quote! {
        {
            trait ToDisplayAs {
                fn #to_display_as(&self) -> &Self;
            }
            impl<T: DisplayAs<#format>> ToDisplayAs for T {
                fn #to_display_as(&self) -> &Self { self }
            }
            __f.write_fmt(format_args!("{}", <_ as DisplayAs<#format>>::display((#expr).#to_display_as())))?;
        }
    })
    .into_iter()
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
    let to_display_as = to_display_as_name(format, &template);
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
        } else if is_str(&t) && !starts_statement(&next_expr) {
            // First print the previous expression...
            toks.extend(expr_toks_to_stmt(&to_display_as, &format, &mut next_expr));
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
    toks.extend(expr_toks_to_stmt(&to_display_as, &format, &mut next_expr));
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
/// 4. The file is looked for next to the source file that contains the
///    attribute, when the compiler tells us which file that is.  Otherwise
///    (for instance when the code was generated by another macro) it is
///    looked for in every directory containing
///    a `.rs` file below the directory in which the compiler is running
///    (the workspace root, when building with cargo), so no two of those
///    directories may contain a file of that name.  Hidden directories,
///    cargo's `target` directory and other git worktrees are not searched.
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
    match last.to_string().as_ref() {
        "{  }" | "{ }" | "{}" => (), // this is what we expect.
        s => panic!(
            "with_template must be applied to an impl that ends in '{{}}', not {}",
            s
        ),
    };
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

/// Like [macro@with_template], but also generate any web responder
/// implementations that are handled via feature flags.
#[proc_macro_attribute]
pub fn with_response_template(input: TokenStream, my_impl: TokenStream) -> TokenStream {
    let displayas_impl = with_template(input, my_impl.clone());
    displayas_impl
}

#[cfg(test)]
mod tests {
    use super::{collect_source_dirs, find_template_dir};
    use std::collections::HashSet;
    use std::fs;
    use std::panic::catch_unwind;
    use std::path::{Path, PathBuf};

    /// A scratch directory tree that is removed when dropped.
    struct Tree(PathBuf);
    impl Tree {
        fn new(name: &str) -> Tree {
            let root = std::env::temp_dir().join(format!(
                "display-as-proc-macro-{}-{}",
                name,
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&root);
            fs::create_dir_all(&root).unwrap();
            Tree(root)
        }
        fn file(&self, path: &str) {
            self.write(path, "");
        }
        fn write(&self, path: &str, contents: &str) {
            let path = self.0.join(path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, contents).unwrap();
        }
    }
    impl Drop for Tree {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    /// Run in the given directory (which is where the compiler runs
    /// when it expands our macros), restoring the old one afterwards.
    struct InDir(PathBuf);
    impl InDir {
        fn new(dir: &Path) -> InDir {
            let old = std::env::current_dir().unwrap();
            std::env::set_current_dir(dir).unwrap();
            InDir(old)
        }
    }
    impl Drop for InDir {
        fn drop(&mut self) {
            std::env::set_current_dir(&self.0).unwrap();
        }
    }

    /// The source directories found under the root of the tree, relative to it.
    fn source_dirs(tree: &Tree) -> Vec<String> {
        let mut found = HashSet::new();
        collect_source_dirs(&tree.0, &mut found);
        let mut dirs: Vec<_> = found
            .iter()
            .map(|d| {
                d.strip_prefix(&tree.0)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        dirs.sort();
        dirs
    }

    fn panic_message(find: &str) -> String {
        let find = find.to_string();
        let payload = catch_unwind(move || find_template_dir(None, &find)).unwrap_err();
        payload.downcast_ref::<String>().unwrap().clone()
    }

    // All the scenarios live in one test, because they change the
    // working directory of the whole process.
    #[test]
    fn find_template_dir_searches_source_directories() {
        let tree = Tree::new("find-template-file");
        // A template next to a Rust file is found, also via a subdirectory.
        tree.file("src/lib.rs");
        tree.file("src/page.html");
        tree.file("src/sub/inner.html");
        // A directory without any Rust files is not a source directory.
        tree.file("docs/notes.html");
        // The same name in two source directories is ambiguous.
        tree.file("a/lib.rs");
        tree.file("a/dup.html");
        tree.file("b/lib.rs");
        tree.file("b/dup.html");
        // Build output is ignored, even if it contains Rust files.
        tree.file("Cargo.toml");
        tree.file("target/debug/gen.rs");
        tree.file("target/debug/shared.html");
        tree.file("target/debug/only-in-target.html");
        tree.file("src/shared.html");
        // The working directory itself may be a source directory.
        tree.file("main.rs");
        tree.file("top.html");
        // Copies of the project in other worktrees do not make names ambiguous.
        tree.file(".claude/worktrees/one/src/lib.rs");
        tree.file(".claude/worktrees/one/src/page.html");
        tree.write("copies/two/.git", "gitdir: /repo/.git/worktrees/two\n");
        tree.file("copies/two/src/lib.rs");
        tree.file("copies/two/src/page.html");

        let _cwd = InDir::new(&tree.0);
        assert_eq!(find_template_dir(None, "page.html"), PathBuf::from("src"));
        assert_eq!(find_template_dir(None, "sub/inner.html"), PathBuf::from("src"));
        assert_eq!(find_template_dir(None, "shared.html"), PathBuf::from("src"));
        assert_eq!(find_template_dir(None, "top.html"), PathBuf::from(""));
        assert!(panic_message("notes.html").starts_with("No template file named \"notes.html\""));
        assert!(panic_message("only-in-target.html")
            .starts_with("No template file named \"only-in-target.html\""));
        assert!(panic_message("dup.html").starts_with("Multiple files named \"dup.html\""));
        // A template next to the file that uses it wins over the same name elsewhere...
        assert_eq!(
            find_template_dir(Some(PathBuf::from("a")), "dup.html"),
            PathBuf::from("a")
        );
        assert_eq!(
            find_template_dir(Some(PathBuf::from("b")), "dup.html"),
            PathBuf::from("b")
        );
        // ... and if it is not there after all, we search for it.
        assert_eq!(
            find_template_dir(Some(PathBuf::from("docs")), "page.html"),
            PathBuf::from("src")
        );
    }

    #[test]
    fn source_dirs_contain_rust_files() {
        let tree = Tree::new("source-dirs");
        tree.file("lib.rs");
        tree.file("src/a/mod.rs");
        tree.file("src/a/deep/deeper/x.rs");
        tree.file("src/b/page.html");
        tree.file("docs/readme.md");
        assert_eq!(source_dirs(&tree), vec!["", "src/a", "src/a/deep/deeper"]);
    }

    #[test]
    fn target_is_skipped_next_to_cargo_toml() {
        let tree = Tree::new("target-dirs");
        tree.file("Cargo.toml");
        tree.file("src/lib.rs");
        tree.file("target/debug/build/dep-1234/out/generated.rs");
        tree.file("member/Cargo.toml");
        tree.file("member/src/lib.rs");
        tree.file("member/target/debug/build.rs");
        assert_eq!(source_dirs(&tree), vec!["member/src", "src"]);
    }

    #[test]
    fn source_directories_named_target_are_searched() {
        let tree = Tree::new("target-module");
        tree.file("Cargo.toml");
        tree.file("src/lib.rs");
        tree.file("src/target/mod.rs");
        assert_eq!(source_dirs(&tree), vec!["src", "src/target"]);
    }

    #[test]
    fn hidden_directories_are_skipped() {
        let tree = Tree::new("hidden-dirs");
        tree.file("src/lib.rs");
        tree.file(".git/hooks/pre-commit.rs");
        tree.file(".claude/scratch/src/lib.rs");
        assert_eq!(source_dirs(&tree), vec!["src"]);
    }

    #[test]
    fn linked_worktrees_are_skipped() {
        let tree = Tree::new("worktrees");
        tree.file("src/lib.rs");
        // A worktree in a directory that is not hidden...
        tree.write(
            "copies/feature/.git",
            "gitdir: /repo/.git/worktrees/feature\n",
        );
        tree.file("copies/feature/src/lib.rs");
        // ... but not a submodule or a separate repository.
        tree.write("vendor/sub/.git", "gitdir: ../../.git/modules/sub\n");
        tree.file("vendor/sub/src/lib.rs");
        tree.file("other/.git/HEAD");
        tree.file("other/src/lib.rs");
        assert_eq!(
            source_dirs(&tree),
            vec!["other/src", "src", "vendor/sub/src"]
        );
    }
}
