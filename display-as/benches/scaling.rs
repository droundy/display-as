//! How the cost of rendering with `display-as` grows with the size of the
//! input, measured with the `scaling` crate.
//!
//! Each benchmark is a function of a size `n` (a number of list items, a
//! string length, a number of table rows...) and is reported as a cost per
//! unit of `n` together with the fitted power law.  Run with
//! `cargo bench -p display-as --bench scaling`.

use display_as::{
    format_as, with_template, write_as, DisplayAs, FormattedString, LaTeX, Math, HTML, UTF8,
};
use std::hint::black_box;

fn main() -> Result<(), scaling::RegistrationError> {
    scaling::Config::default().run_and_print()
}

// ---- a long list rendered through a `for` loop ----

#[scaling::bench_scaling(nmin = 16, make_input = |n| (0..n).collect::<Vec<usize>>())]
fn list_for_loop(items: &mut Vec<usize>) -> FormattedString<HTML> {
    let items = black_box(&*items);
    black_box(format_as!(HTML, "<ul>" for i in items.iter() { "<li>" i "</li>" } "</ul>"))
}

#[scaling::bench_scaling(nmin = 16, make_input = |n| (0..n).map(|i| format!("item <{}> & co", i)).collect::<Vec<String>>())]
fn list_of_strings_for_loop(items: &mut Vec<String>) -> FormattedString<HTML> {
    let items = black_box(&*items);
    black_box(format_as!(HTML, "<ul>" for s in items.iter() { "<li>" s "</li>" } "</ul>"))
}

// ---- escaping strings of growing length ----

#[scaling::bench_scaling(nmin = 64, make_input = |n| "a".repeat(n))]
fn escape_html_plain(s: &mut String) -> FormattedString<HTML> {
    let s = black_box(&*s);
    black_box(format_as!(HTML, s))
}

#[scaling::bench_scaling(nmin = 64, make_input = |n| "<&>".repeat(n / 3 + 1))]
fn escape_html_all_special(s: &mut String) -> FormattedString<HTML> {
    let s = black_box(&*s);
    black_box(format_as!(HTML, s))
}

#[scaling::bench_scaling(nmin = 64, make_input = |n| "a".repeat(n))]
fn escape_latex_plain(s: &mut String) -> FormattedString<LaTeX> {
    let s = black_box(&*s);
    black_box(format_as!(LaTeX, s))
}

#[scaling::bench_scaling(nmin = 64, make_input = |n| "&%$#_{}".repeat(n / 7 + 1))]
fn escape_latex_all_special(s: &mut String) -> FormattedString<LaTeX> {
    let s = black_box(&*s);
    black_box(format_as!(LaTeX, s))
}

// ---- numbers: `n` values formatted in a loop ----

fn integers(n: usize) -> Vec<i64> {
    (0..n as i64).map(|i| i * 7919 - 1000).collect()
}

#[scaling::bench_scaling(nmin = 16, make_input = integers)]
fn integers_html(xs: &mut Vec<i64>) -> FormattedString<HTML> {
    let xs = black_box(&*xs);
    black_box(format_as!(HTML, for x in xs.iter() { x " " }))
}

#[scaling::bench_scaling(nmin = 16, make_input = integers)]
fn integers_utf8(xs: &mut Vec<i64>) -> FormattedString<UTF8> {
    let xs = black_box(&*xs);
    black_box(format_as!(UTF8, for x in xs.iter() { x " " }))
}

// Floats of many different magnitudes, so that both plain and exponent
// notation are exercised.
fn floats(n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| (i as f64 - 50.0) * 1.2345678e-3_f64.powi(i as i32 % 9 - 4))
        .collect()
}

#[scaling::bench_scaling(nmin = 16, make_input = floats)]
fn floats_html(xs: &mut Vec<f64>) -> FormattedString<HTML> {
    let xs = black_box(&*xs);
    black_box(format_as!(HTML, for x in xs.iter() { x " " }))
}

#[scaling::bench_scaling(nmin = 16, make_input = floats)]
fn floats_math(xs: &mut Vec<f64>) -> FormattedString<Math> {
    let xs = black_box(&*xs);
    black_box(format_as!(Math, for x in xs.iter() { x " " }))
}

#[scaling::bench_scaling(nmin = 16, make_input = floats)]
fn floats_utf8(xs: &mut Vec<f64>) -> FormattedString<UTF8> {
    let xs = black_box(&*xs);
    black_box(format_as!(UTF8, for x in xs.iter() { x " " }))
}

// ---- file templates and nesting, with growing row counts ----

struct BigTable {
    table: Vec<Vec<usize>>,
}

#[with_template("[%" "%]" "big-table-displayas.html")]
impl DisplayAs<HTML> for BigTable {}

/// The `big-table` fixture: a table of `n` rows of 8 cells each.
#[scaling::bench_scaling(nmin = 16, make_input = |n| BigTable {
    table: (0..n).map(|_| (0..8).collect()).collect(),
})]
fn file_template_table_rows(ctx: &mut BigTable) -> FormattedString<HTML> {
    let ctx = black_box(&*ctx);
    black_box(format_as!(HTML, ctx))
}

struct Team {
    name: String,
    score: u8,
}

struct Teams {
    year: u16,
    teams: Vec<Team>,
}

#[with_template("[%" "%]" "teams-displayas.html")]
impl DisplayAs<HTML> for Teams {}

/// The `teams` fixture: a file template with an `if` in a `for` loop.
#[scaling::bench_scaling(nmin = 16, make_input = |n| Teams {
    year: 2015,
    teams: (0..n)
        .map(|i| Team { name: format!("Team <{}>", i), score: (i % 100) as u8 })
        .collect(),
})]
fn file_template_teams(ctx: &mut Teams) -> FormattedString<HTML> {
    let ctx = black_box(&*ctx);
    black_box(format_as!(HTML, ctx))
}

/// Each row is itself rendered by a nested `format_as!`, whose result is
/// embedded in the outer template without being escaped again.
#[scaling::bench_scaling(nmin = 16, make_input = |n| (0..n).collect::<Vec<usize>>())]
fn nested_format_as(rows: &mut Vec<usize>) -> FormattedString<HTML> {
    let rows = black_box(&*rows);
    black_box(format_as!(HTML, "<table>"
        for i in rows.iter() {
            "<tr>" format_as!(HTML, "<td>" i "</td><td>" i * 2 "</td>") as HTML "</tr>"
        }
        "</table>"))
}

// ---- where the output goes ----

/// Text with nothing to escape, so that the cost of the output buffer (and
/// not of escaping) is what shows up.
fn text(n: usize) -> String {
    "abc ".repeat(n)
}

/// `format_as!` builds a fresh `String`, growing it as it goes.
#[scaling::bench_scaling(nmin = 16, make_input = text)]
fn output_formatted_string(s: &mut String) -> FormattedString<HTML> {
    let s = black_box(&*s);
    black_box(format_as!(HTML, "<p>" s "</p>"))
}

/// `write_as!` into a `String` allocated up front for the escaped output.
#[scaling::bench_scaling(nmin = 16, make_input = text)]
fn output_write_as_preallocated(s: &mut String) -> String {
    let s = black_box(&*s);
    let mut out = String::with_capacity(s.len() * 2 + 16);
    write_as!(HTML, out, "<p>" s "</p>").unwrap();
    black_box(out)
}

/// `write_as!` into an empty `String`, which has to grow as it is written.
#[scaling::bench_scaling(nmin = 16, make_input = text)]
fn output_write_as_empty(s: &mut String) -> String {
    let s = black_box(&*s);
    let mut out = String::new();
    write_as!(HTML, out, "<p>" s "</p>").unwrap();
    black_box(out)
}
