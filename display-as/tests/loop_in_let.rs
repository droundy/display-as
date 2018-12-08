extern crate display_as;

use display_as::{display_as_string, HTML};

// #[test]
// fn test_let() {
//     let foos = vec!["hello", "world"];
//     assert_eq!(display_as_string!(HTML, let foo = {
//         for i in foos.iter() {
//             "counting " i " "
//         }
//     };
//     foo),
//     r"I am counting 0 counting 1 counting 2 and I am done!");
// }

#[test]
fn test_loop_no_let() {
    assert_eq!(display_as_string!(HTML,
                                  for i in [1u8,2].into_iter() {
                                      "counting " *i
                                  }),
               r"counting 1counting 2");
}

// #[test]
// fn test_loop_no_let() {
//     assert_eq!(display_as_string!(HTML,
//                                   for i in [1u8,2].into_iter() {
//                                       let j: u8 = *i;
//                                       "counting " j
//                                   }),
//                r"counting 1counting 2");
// }

// #[test]
// fn test_no_loop_no_let() {
//     assert_eq!(&display_as_string!(HTML, let i = 1u8; i),
//                r"1");
// }
