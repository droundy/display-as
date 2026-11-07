//! Braces in patterns (struct patterns) are Rust, not template blocks.

use display_as::{format_as, HTML};

enum Shape {
    Circle { r: u32 },
    Rect { w: u32, h: u32 },
    Unit,
}

fn describe(shape: &Shape) -> String {
    format_as!(HTML,
               match shape {
                   Shape::Circle { r } => { "circle " r }
                   Shape::Rect { w, h } if w == h => { "square " w }
                   Shape::Rect { w, h } => { "rect " w "x" h }
                   Shape::Unit => { "unit" }
               })
    .into_string()
}

#[test]
fn struct_patterns_in_match() {
    assert_eq!(describe(&Shape::Circle { r: 2 }), "circle 2");
    assert_eq!(describe(&Shape::Rect { w: 3, h: 3 }), "square 3");
    assert_eq!(describe(&Shape::Rect { w: 3, h: 4 }), "rect 3x4");
    assert_eq!(describe(&Shape::Unit), "unit");
}

struct Item {
    n: u32,
}

#[test]
fn struct_pattern_in_while_let() {
    assert_eq!(
        format_as!(HTML,
                   let mut stack = vec![Item { n: 1 }, Item { n: 2 }];
                   while let Some(Item { n }) = stack.pop() {
                       n ","
                   })
        .into_string(),
        "2,1,"
    );
}

#[test]
fn struct_pattern_in_for() {
    let items = vec![Item { n: 1 }, Item { n: 2 }];
    assert_eq!(
        format_as!(HTML, for Item { n } in items.iter() { n "," }).into_string(),
        "1,2,"
    );
    let pairs = vec![(1, Item { n: 10 }), (2, Item { n: 20 })];
    assert_eq!(
        format_as!(HTML, for (i, Item { n }) in pairs.iter() { i ":" n " " }).into_string(),
        "1:10 2:20 "
    );
}

#[test]
fn struct_pattern_in_else_if_let() {
    let shape = Shape::Rect { w: 1, h: 2 };
    assert_eq!(
        format_as!(HTML,
                   if let Shape::Circle { r } = shape {
                       "circle " r
                   } else if let Shape::Rect { w, h } = shape {
                       "rect " w "x" h
                   } else {
                       "other"
                   })
        .into_string(),
        "rect 1x2"
    );
}
