# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

`display-as` is a compile-time template engine for Rust (like askama/ructe), but templates contain near-arbitrary Rust and the output is typed by a `Format` (HTML, LaTeX, Math, URL, UTF8, Rust...), so a value can't be emitted into the wrong format or double-escaped.

Cargo workspace with two crates, published separately (versions are bumped independently; when releasing, also raise the `display-as-proc-macro` version requirement in `display-as/Cargo.toml`):
- `display-as` — the runtime library (`Format`, `DisplayAs<F>` traits, built-in formats, `FormattedString`, web-framework glue) and the user-facing docs.
- `display-as-proc-macro` — the proc-macro implementation (`format_as!`, `write_as!`, `#[with_template]`, `#[with_response_template]`). `display-as` re-exports these.

## Commands

```bash
cargo test --all                          # CI test job (unit, integration tests in display-as/tests, and doctests)
cargo check --all --all-features          # CI check job; run on stable and on the MSRV, Rust 1.88.0 (`rust-version` in both Cargo.toml files)
cargo test -p display-as --test with_template   # one integration test file
cargo test -p display-as --test with_template foo   # one test by name
cargo bench -p display-as                 # all benches in display-as/benches (criterion and scaling)
cargo bench -p display-as --bench scaling # how render time scales with input size (`scaling` crate, ~15 s)
scripts/check-reproducible.sh             # macro output must be deterministic: two builds give identical binaries
```

Optional features on `display-as`: `rouille`, `actix-web`, `gotham-web`, `warp`, `axum` (web responders), `serde1`. `docs.rs` builds with `all-features`. CI resolves dependencies with `CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback` so the MSRV job picks compatible versions (`Cargo.lock` is not committed).

## Architecture

**Template language** is a token stream, not a string: a sequence of string literals (emitted verbatim), Rust expressions (emitted via `DisplayAs<F>`, escaped for the format), `expr as OtherFormat` (embed another format), brace blocks after Rust control-flow (`if`/`for`/`match`... `{ template }`), and `;`-terminated Rust statements (`let x = 5;`). `let name = { template };` stores a reusable sub-template. `include!("file")` inlines another template file.

**Proc-macro flow** (`display-as-proc-macro/src/lib.rs`): `format_as!`/`write_as!`/`with_template` all funnel into `template_to_statements`, which walks the token trees (via `expr_toks_to_stmt` / `expr_toks_to_conditional`) and generates a series of `write!`/`DisplayAs::fmt` statements. `#[with_template(...)]` is applied to an empty `impl DisplayAs<F> for T {}` and injects a `fn fmt` body; it finds `F` by scanning the impl tokens.

Each expression is wrapped in a local helper trait whose method name comes from `to_display_as_name`, a deterministic FNV-1a hash of that template level's tokens (unique per nesting level, which is what nested `format_as!` calls need). The expansion must stay deterministic — no randomness, no `DefaultHasher` — so builds are reproducible and incremental compilation works (`display-as/tests/nested.rs`, `scripts/check-reproducible.sh`).

**Template files** (`#[with_template("foo.html")]`): `find_template_dir` first looks next to the invoking source file (`Span::call_site().local_file()`, stable since Rust 1.88, which is why that is the MSRV), so same-named templates in different modules work (`display-as/tests/same_name*`). If the compiler can't say which file that is, it falls back to `collect_source_dirs`, a walk from the compiler's cwd (the workspace root under cargo) over directories containing `.rs` files, skipping hidden directories, a `target` next to a `Cargo.toml`, and linked git worktrees; it panics unless exactly one directory has the file. `read_template_file` wraps the file contents into a token stream: the file begins/ends in *string-literal text* (opening/closing quotes omitted, so the file looks like the target format with the Rust in quotes "inside out"), and `count_pounds` picks the raw-string `#` count. Three string args (`left`, `right`, `"file"`) let you use custom delimiters instead of quotes. An `assert_eq!(include_str!(...).len(), N)` is appended to the generated tokens so cargo tracks the template file for rebuilds. Test fixtures for this are `display-as/tests/*.html`; `display-as/src/{base,student,class}.html` are both doctest fixtures and are embedded into the crate docs via `#![doc = include_str!(...)]`.

**Runtime crate** (`display-as/src/lib.rs`): `Format` defines `escape` + `mime`; each format lives in its own module (`html.rs`, `latex.rs`, `mathlatex.rs`, `url.rs`, `utf8.rs`, `rust.rs`, `float.rs` for number formatting). Helper macros `display_integers_as!` / `display_floats_as!` implement `DisplayAs` for numeric types when adding a new `Format`; they must use `$crate` and fully qualified paths because they are invoked from other crates (`display-as/tests/custom_format.rs`). Feature-gated modules in `lib.rs` (`rouille`, `actix`, `gotham`, `warp`, `axum`) provide responder impls; `with_response_template` currently just delegates to `with_template`.

`#![deny(missing_docs)]` is set on `display-as`, so public items need doc comments. The crate-level docs in `lib.rs` double as the user guide and contain doctests — update them when changing template syntax.
