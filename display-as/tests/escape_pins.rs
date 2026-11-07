//! Pin the exact output of the HTML, LaTeX and Math escapers, so that
//! refactoring their shared code cannot change what they emit.
use display_as::{format_as, Format, LaTeX, Math, HTML};

fn escaped<F: Format>(s: &str) -> String {
    struct Esc<'a, F>(&'a str, std::marker::PhantomData<F>);
    impl<'a, F: Format> std::fmt::Display for Esc<'a, F> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            F::escape(f, self.0)
        }
    }
    Esc::<F>(s, std::marker::PhantomData).to_string()
}

/// Every ASCII character, and its expected escape (None means unchanged).
fn check_ascii<F: Format>(table: &[(char, &str)]) {
    for b in 0u8..128 {
        let c = b as char;
        let expected = table
            .iter()
            .find(|(k, _)| *k == c)
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| c.to_string());
        assert_eq!(escaped::<F>(&c.to_string()), expected, "char {:?}", c);
        // The same character in the middle of other text.
        assert_eq!(
            escaped::<F>(&format!("ab{}cd", c)),
            format!("ab{}cd", expected),
            "char {:?}",
            c
        );
    }
}

const HTML_TABLE: &[(char, &str)] = &[
    ('<', "&lt;"),
    ('>', "&gt;"),
    ('&', "&amp;"),
    ('"', "&quot;"),
    ('\'', "&#x27;"),
    ('/', "&#x2f;"),
];
const LATEX_TABLE: &[(char, &str)] = &[
    ('&', r"\&"),
    ('{', r"\{"),
    ('}', r"\}"),
    ('#', r"\#"),
    ('%', r"\%"),
    ('\\', r"\textbackslash{}"),
    ('~', r"\textasciitilde{}"),
    ('$', r"\$"),
    ('_', r"\_"),
    ('^', r"\^{}"),
];
const MATH_TABLE: &[(char, &str)] = &[
    ('&', r"\&"),
    ('{', r"\{"),
    ('}', r"\}"),
    ('#', r"\#"),
    ('%', r"\%"),
    ('\\', r"\backslash{}"),
    ('~', r"\tilde{}"),
    ('$', r"\$"),
    ('_', r"\_"),
    ('^', r"\hat{}"),
];

#[test]
fn html_every_ascii_char() {
    check_ascii::<HTML>(HTML_TABLE);
}
#[test]
fn latex_every_ascii_char() {
    check_ascii::<LaTeX>(LATEX_TABLE);
}
#[test]
fn math_every_ascii_char() {
    check_ascii::<Math>(MATH_TABLE);
}

#[test]
fn edge_cases() {
    for escape in [escaped::<HTML>, escaped::<LaTeX>, escaped::<Math>] {
        assert_eq!(escape(""), "");
        assert_eq!(escape("plain text"), "plain text");
        // Non-ASCII text is passed through untouched, also next to special chars.
        assert_eq!(escape("héllo wörld ☃ 日本語 🦀"), "héllo wörld ☃ 日本語 🦀");
    }
    assert_eq!(escaped::<HTML>("é<☃>🦀&日"), "é&lt;☃&gt;🦀&amp;日");
    assert_eq!(escaped::<LaTeX>("é%☃_🦀&日"), r"é\%☃\_🦀\&日");
    assert_eq!(escaped::<Math>("é%☃_🦀&日"), r"é\%☃\_🦀\&日");
}

#[test]
fn repeated_and_adjacent() {
    assert_eq!(escaped::<HTML>("<<>>&&"), "&lt;&lt;&gt;&gt;&amp;&amp;");
    assert_eq!(escaped::<HTML>("'\"/"), "&#x27;&quot;&#x2f;");
    assert_eq!(escaped::<LaTeX>("{}{}"), r"\{\}\{\}");
    assert_eq!(escaped::<Math>("{}{}"), r"\{\}\{\}");
    assert_eq!(escaped::<LaTeX>("^^"), r"\^{}\^{}");
    assert_eq!(escaped::<Math>("^^"), r"\hat{}\hat{}");
    assert_eq!(escaped::<HTML>("a<"), "a&lt;");
    assert_eq!(escaped::<HTML>("<a"), "&lt;a");
}

#[test]
fn through_templates() {
    let s = "<a href='x'>R&D 50% of_it {x} $y ~z \\ #1 ^</a>";
    assert_eq!(
        format_as!(HTML, s).into_string(),
        "&lt;a href=&#x27;x&#x27;&gt;R&amp;D 50% of_it {x} $y ~z \\ #1 ^&lt;&#x2f;a&gt;"
    );
    assert_eq!(
        format_as!(LaTeX, s).into_string(),
        r"<a href='x'>R\&D 50\% of\_it \{x\} \$y \textasciitilde{}z \textbackslash{} \#1 \^{}</a>"
    );
    assert_eq!(
        format_as!(Math, s).into_string(),
        r"<a href='x'>R\&D 50\% of\_it \{x\} \$y \tilde{}z \backslash{} \#1 \hat{}</a>"
    );
}
