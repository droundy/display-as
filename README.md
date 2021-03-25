# DisplayAs

[![Documentation](https://docs.rs/display-as/badge.svg)](https://docs.rs/display-as)
[![Build Status](https://github.com/droundy/display-as/actions)](https://github.com/droundy/display_as/actions/workflows/test.yml/badge.svg)

These crates creates rusty templates that are evaluated at
compile-time (much like [askama](https://docs.rs/askama)).
`DisplayAs` is explicitly designed to support multiple output formats
(thus the "as" in its name).

## Comparison with other template engines in rust

Given there are numerous existing template engines, you might ask what
distinguishes `display-as-template` from these other engines?

1. The most notable distinction is that `display-as-template`
   compiles the templates at compile time, like
   [askama](https://docs.rs/askama) and
   [ructe](https://crates.io/crates/ructe) but unlike most other
   engines.

2. `diplay-as-template` supports (almost) arbitrary rust code in the
   template, unlike [askama](https://github.com/djc/askama/issues/95)
   or ructe.  In the case of askama, there is a conscious decision not
   to support this.  I believe that it is nicer and easier not to
   learn a new language for the expressiosn within templates.

3. `DisplayAs` and `display-as-template` support embedding one format
   into another, so that you can mix languages.  This is most common
   in HTML, which supports numerous formats such as javascript or CSS,
   but also to allows math mode within either LaTeX or in HTML using
   MathJax.  This has been discussed as a
   [possible feature in ructe](https://github.com/kaj/ructe/issues/1).

4. Using `display-as-template` is typesafe on the output side as well
   as the input side.  You can't accidentally include javascript
   formatted text into HTML, or
   [double-escape HTML strings](https://github.com/djc/askama/issues/108).
   
