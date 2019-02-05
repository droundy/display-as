extern crate display_as;

use display_as::{format_as, HTML};

#[test]
fn test_match() {
    let mut someone = Some(1);
    assert_eq!(
        format_as!(HTML, match someone {
            Some(x) => { x }
            None => { "None!" }
        }),
        r"1"
    );

    someone = None;
    assert_eq!(
        format_as!(HTML, match someone {
            Some(x) => { x }
            None => { "None!" }
        }),
        r"None!"
    );
}
