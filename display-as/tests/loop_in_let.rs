extern crate display_as;

use display_as::{format_as, HTML};

#[test]
fn test_let() {
    let foos = vec!["hello", "world"];
    assert_eq!(
        format_as!(HTML, let foo = {
        for i in foos.iter() {
            "counting " i " "
        }
    };
    foo),
        r"counting hello counting world "
    );
}

#[test]
fn test_loop_no_let() {
    assert_eq!(
        format_as!(
            HTML,
            for i in [1u8, 2].into_iter() {
                "counting " * i
            }
        ),
        r"counting 1counting 2"
    );
}

#[test]
fn test_loop_no_let_b() {
    assert_eq!(
        format_as!(HTML,
        for i in [1u8,2].into_iter() {
            let j: u8 = *i;
            "counting " j
        }),
        r"counting 1counting 2"
    );
}

#[test]
fn test_no_loop_no_let_c() {
    assert_eq!(&format_as!(HTML, let i = 1u8; i), r"1");
}
