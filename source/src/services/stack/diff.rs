use crate::handlers::stack::UpdateBlock;
use crate::services::stack::Block;

// pub(crate) fn get_new_blocks<'a>(
//     existent_blocks: &'a Vec<Block>,
//     updated_blocks: &'a Vec<UpdateBlock>,
// ) -> Vec<&'a UpdateBlock> {
//     updated_blocks
//         .iter()
//         .filter(|u| {
//             if u.id.is_none() {
//                 return true;
//             }
//
//             let updated_block_id_presented_in_existent_blocks_ids =
//                 existent_blocks.iter().any(|e| match &u.id {
//                     None => false,
//                     Some(id) => id == &e.id,
//                 });
//
//             return !updated_block_id_presented_in_existent_blocks_ids;
//         })
//         .collect()
// }

#[cfg(test)]
mod new_blocks_tests {
    use super::*;
    use crate::handlers::stack::ChangeMark;
    use crate::repos::Id;
    use crate::services::stack::Mark;

    // #[test]
    // fn new_blocks_returns_empty_result_if_both_args_has_empty_blocks() {
    //     let existent_blocks = vec![];
    //     let updated_blocks = vec![];
    //
    //     let added_blocks = get_new_blocks(&existent_blocks, &updated_blocks);
    //     assert!(added_blocks.is_empty());
    // }
    //
    // #[test]
    // fn new_blocks_returns_nothing_if_updated_blocks_empty() {
    //     let existent_blocks = vec![
    //         Block {
    //             id: Id("1".to_string()),
    //             stack_id: Id::from_str("1"),
    //             order: 0,
    //             text: "some_test".to_string(),
    //             marks: vec![],
    //             current_version: 0,
    //             initial_version: 0
    //         },
    //         Block {
    //             id: Id("2".to_string()),
    //             stack_id: Id::from_str("1"),
    //             order: 1,
    //             text: "some_other_text".to_string(),
    //             marks: vec![Mark {
    //                 id: Id("1".to_string()),
    //                 from: 0,
    //                 to: 3,
    //             }],
    //             current_version: 0,
    //             initial_version: 0
    //         },
    //     ];
    //     let updated_blocks = vec![];
    //
    //     let added_blocks = get_new_blocks(&existent_blocks, &updated_blocks);
    //     assert!(added_blocks.is_empty());
    // }
    //
    // #[test]
    // fn new_blocks_returns_all_updated_blocks_if_existent_blocks_empty_1() {
    //     let existent_blocks = vec![];
    //     let updated_blocks = vec![UpdateBlock {
    //         id: None,
    //         order: 0,
    //         text: "block_1".to_string(),
    //         marks: vec![],
    //     }];
    //
    //     let added_blocks = get_new_blocks(&existent_blocks, &updated_blocks);
    //     assert_eq!(
    //         added_blocks,
    //         vec![&UpdateBlock {
    //             id: None,
    //             order: 0,
    //             text: "block_1".to_string(),
    //             marks: vec![]
    //         }]
    //     );
    // }
    //
    // #[test]
    // fn new_blocks_returns_all_updated_blocks_if_existent_blocks_empty_2() {
    //     let existent_blocks = vec![];
    //     let updated_blocks = vec![UpdateBlock {
    //         id: None,
    //         order: 0,
    //         text: "block_1".to_string(),
    //         marks: vec![UpdateMark {
    //             id: None,
    //             from: 0,
    //             to: 3,
    //         }],
    //     }];
    //
    //     let added_blocks = get_new_blocks(&existent_blocks, &updated_blocks);
    //     assert_eq!(
    //         added_blocks,
    //         vec![&UpdateBlock {
    //             id: None,
    //             order: 0,
    //             text: "block_1".to_string(),
    //             marks: vec![UpdateMark {
    //                 id: None,
    //                 from: 0,
    //                 to: 3
    //             }]
    //         }]
    //     );
    // }
    //
    // #[test]
    // fn new_blocks_returns_all_updated_blocks_if_existent_blocks_empty_3() {
    //     let existent_blocks = vec![];
    //     let updated_blocks = vec![UpdateBlock {
    //         id: None,
    //         order: 0,
    //         text: "block_1".to_string(),
    //         marks: vec![
    //             UpdateMark {
    //                 id: None,
    //                 from: 0,
    //                 to: 3,
    //             },
    //             UpdateMark {
    //                 id: Some(Id("1".to_string())),
    //                 from: 2,
    //                 to: 4,
    //             },
    //         ],
    //     }];
    //
    //     let added_blocks = get_new_blocks(&existent_blocks, &updated_blocks);
    //     assert_eq!(
    //         added_blocks,
    //         vec![&UpdateBlock {
    //             id: None,
    //             order: 0,
    //             text: "block_1".to_string(),
    //             marks: vec![
    //                 UpdateMark {
    //                     id: None,
    //                     from: 0,
    //                     to: 3
    //                 },
    //                 UpdateMark {
    //                     id: Some(Id("1".to_string())),
    //                     from: 2,
    //                     to: 4,
    //                 },
    //             ]
    //         }]
    //     );
    // }
    //
    // #[test]
    // fn new_blocks_returns_blocks_with_ids_that_absent_in_existent() {
    //     let existent_blocks = vec![Block {
    //         id: Id::from_str("1"),
    //         stack_id: Id::from_str("1"),
    //         order: 0,
    //         text: "some_text".to_string(),
    //         marks: vec![],
    //         current_version: 0,
    //         initial_version: 0
    //     }];
    //     let updated_blocks = vec![
    //         // this block should not be presented in added blocks
    //         UpdateBlock {
    //             id: Some(Id::from_str("1")),
    //             order: 0,
    //             text: "block_1".to_string(),
    //             marks: vec![],
    //         },
    //         UpdateBlock {
    //             id: None,
    //             order: 1,
    //             text: "block_1".to_string(),
    //             marks: vec![],
    //         },
    //         UpdateBlock {
    //             id: Some(Id::from_str("2")),
    //             order: 2,
    //             text: "block_2".to_string(),
    //             marks: vec![],
    //         },
    //     ];
    //
    //     let added_blocks = get_new_blocks(&existent_blocks, &updated_blocks);
    //     assert_eq!(
    //         added_blocks,
    //         vec![
    //             &UpdateBlock {
    //                 id: None,
    //                 order: 1,
    //                 text: "block_1".to_string(),
    //                 marks: vec![],
    //             },
    //             &UpdateBlock {
    //                 id: Some(Id::from_str("2")),
    //                 order: 2,
    //                 text: "block_2".to_string(),
    //                 marks: vec![],
    //             },
    //         ]
    //     );
    // }
}

// pub(crate) fn get_deleted_blocks<'a>(
//     existent_blocks: &'a Vec<Block>,
//     updated_blocks: &'a Vec<UpdateBlock>,
// ) -> Vec<&'a Block> {
//     existent_blocks
//         .iter()
//         .filter(|e| {
//             let existent_block_presented_in_updated = updated_blocks.iter().any(|u| match &u.id {
//                 None => false,
//                 Some(id) => id == &e.id,
//             });
//
//             !existent_block_presented_in_updated
//         })
//         .collect()
// }

#[cfg(test)]
mod deleted_blocks_tests {
    use super::*;
    use crate::repos::Id;
    use crate::services::stack::Block;

    // #[test]
    // fn deleted_blocks_returns_blocks_not_presented_in_updated_blocks_1() {
    //     let existent_blocks = vec![Block {
    //         id: Id::from_str("1"),
    //         stack_id: Id::from_str("1"),
    //         order: 0,
    //         text: "some_text".to_string(),
    //         marks: vec![],
    //         current_version: 0,
    //         initial_version: 0
    //     }];
    //     let updated_blocks = vec![];
    //     let deleted_blocks = get_deleted_blocks(&existent_blocks, &updated_blocks);
    //     assert_eq!(
    //         deleted_blocks,
    //         vec![&Block {
    //             id: Id::from_str("1"),
    //             stack_id: Id::from_str("1"),
    //             order: 0,
    //             text: "some_text".to_string(),
    //             marks: vec![],
    //             current_version: 0,
    //             initial_version: 0
    //         }]
    //     );
    // }
    //
    // #[test]
    // fn deleted_blocks_returns_blocks_not_presented_in_updated_blocks_2() {
    //     let existent_blocks = vec![Block {
    //         id: Id::from_str("1"),
    //         stack_id: Id::from_str("1"),
    //         order: 0,
    //         text: "some_text".to_string(),
    //         marks: vec![],
    //         current_version: 0,
    //         initial_version: 0
    //     }];
    //     let updated_blocks = vec![UpdateBlock {
    //         id: Some(Id::from_str("2")),
    //         order: 0,
    //         text: "some_other_text".to_string(),
    //         marks: vec![],
    //     }];
    //
    //     let deleted_blocks = get_deleted_blocks(&existent_blocks, &updated_blocks);
    //
    //     assert_eq!(
    //         deleted_blocks,
    //         vec![&Block {
    //             id: Id::from_str("1"),
    //             stack_id: Id::from_str("1"),
    //             order: 0,
    //             text: "some_text".to_string(),
    //             marks: vec![],
    //             current_version: 0,
    //             initial_version: 0
    //         }]
    //     );
    // }
}

// pub(crate) fn get_updated_blocks<'a>(
//     existent_blocks: &'a Vec<Block>,
//     updated_blocks: &'a Vec<UpdateBlock>,
// ) -> Vec<(&'a Block, &'a UpdateBlock)> {
//     // let result = vec![];
//
//     existent_blocks
//         .iter()
//         .filter_map(|existent_block| {
//             let updated_block = updated_blocks.iter().find(|u| match &u.id {
//                 None => false,
//                 Some(id) => &existent_block.id == id,
//             })?;
//
//             if existent_block != updated_block {
//                 Some((existent_block, updated_block))
//             } else {
//                 None
//             }
//         })
//         .collect()
// }

#[cfg(test)]
mod test_updated_blocks {
    use super::*;
    use crate::handlers::stack::ChangeMark;
    use crate::repos::Id;
    use crate::services::stack::{Block, Mark};

    // #[test]
    // fn empty_updated_blocks_returned_if_empty_updated_blocks_passed() {
    //     let existent_blocks = vec![Block {
    //         id: Id::from_str("1"),
    //         stack_id: Id::from_str("1"),
    //         order: 0,
    //         text: "some_text".to_string(),
    //         marks: vec![Mark {
    //             id: Id::from_str("1"),
    //             from: 0,
    //             to: 2,
    //         }],
    //         current_version: 0,
    //         initial_version: 0
    //     }];
    //     let updated_blocks = vec![];
    //
    //     let result = get_updated_blocks(&existent_blocks, &updated_blocks);
    //     assert_eq!(result.len(), 0);
    // }
    //
    // #[test]
    // fn empty_updated_blocks_returned_if_blocks_identical() {
    //     let existent_blocks = vec![Block {
    //         id: Id::from_str("1"),
    //         stack_id: Id::from_str("1"),
    //         order: 0,
    //         text: "some_text".to_string(),
    //         marks: vec![Mark {
    //             id: Id::from_str("1"),
    //             from: 0,
    //             to: 2,
    //         }],
    //         current_version: 0,
    //         initial_version: 0
    //     }];
    //     let updated_blocks = vec![UpdateBlock {
    //         id: Some(Id::from_str("1")),
    //         order: 0,
    //         text: "some_text".to_string(),
    //         marks: vec![UpdateMark {
    //             id: Some(Id::from_str("1")),
    //             from: 0,
    //             to: 2,
    //         }],
    //     }];
    //
    //     let result = get_updated_blocks(&existent_blocks, &updated_blocks);
    //     assert_eq!(result.len(), 0);
    // }
    //
    // #[test]
    // fn empty_updated_blocks_returned_if_no_matched_ids() {
    //     let existent_blocks = vec![Block {
    //         id: Id::from_str("1"),
    //         stack_id: Id::from_str("1"),
    //         order: 0,
    //         text: "some_text".to_string(),
    //         marks: vec![Mark {
    //             id: Id::from_str("1"),
    //             from: 0,
    //             to: 2,
    //         }],
    //         current_version: 0,
    //         initial_version: 0
    //     }];
    //     let updated_blocks = vec![UpdateBlock {
    //         id: Some(Id::from_str("2")),
    //         order: 0,
    //         text: "some_text".to_string(),
    //         marks: vec![UpdateMark {
    //             id: Some(Id::from_str("1")),
    //             from: 0,
    //             to: 2,
    //         }],
    //     }];
    //
    //     let result = get_updated_blocks(&existent_blocks, &updated_blocks);
    //     assert_eq!(result.len(), 0);
    // }
    //
    // #[test]
    // fn updated_blocks_returned_if_text_differs_and_marks_the_same() {
    //     let existent_blocks = vec![Block {
    //         id: Id::from_str("1"),
    //         stack_id: Id::from_str("1"),
    //         order: 0,
    //         text: "some_text".to_string(),
    //         marks: vec![Mark {
    //             id: Id::from_str("1"),
    //             from: 0,
    //             to: 2,
    //         }],
    //         current_version: 0,
    //         initial_version: 0
    //     }];
    //     let updated_blocks = vec![UpdateBlock {
    //         id: Some(Id::from_str("1")),
    //         order: 0,
    //         text: "some_other_text".to_string(),
    //         marks: vec![UpdateMark {
    //             id: Some(Id::from_str("1")),
    //             from: 0,
    //             to: 2,
    //         }],
    //     }];
    //
    //     let result = get_updated_blocks(&existent_blocks, &updated_blocks);
    //     assert_eq!(
    //         result,
    //         vec![(
    //             &Block {
    //                 id: Id::from_str("1"),
    //                 stack_id: Id::from_str("1"),
    //                 order: 0,
    //                 text: "some_text".to_string(),
    //                 marks: vec![Mark {
    //                     id: Id::from_str("1"),
    //                     from: 0,
    //                     to: 2,
    //                 }],
    //                 current_version: 0,
    //                 initial_version: 0
    //             },
    //             &UpdateBlock {
    //                 id: Some(Id::from_str("1")),
    //                 order: 0,
    //                 text: "some_other_text".to_string(),
    //                 marks: vec![UpdateMark {
    //                     id: Some(Id::from_str("1")),
    //                     from: 0,
    //                     to: 2,
    //                 }],
    //             }
    //         )]
    //     );
    // }
    //
    // #[test]
    // fn updated_blocks_returned_if_text_same_and_marks_added() {
    //     let existent_blocks = vec![Block {
    //         id: Id::from_str("1"),
    //         stack_id: Id::from_str("1"),
    //         order: 0,
    //         text: "some_text".to_string(),
    //         marks: vec![],
    //         current_version: 0,
    //         initial_version: 0
    //     }];
    //     let updated_blocks = vec![UpdateBlock {
    //         id: Some(Id::from_str("1")),
    //         order: 0,
    //         text: "some_text".to_string(),
    //         marks: vec![UpdateMark {
    //             id: Some(Id::from_str("1")),
    //             from: 1,
    //             to: 2,
    //         }],
    //     }];
    //
    //     let result = get_updated_blocks(&existent_blocks, &updated_blocks);
    //     assert_eq!(
    //         result,
    //         vec![(
    //             &Block {
    //                 id: Id::from_str("1"),
    //                 stack_id: Id::from_str("1"),
    //                 order: 0,
    //                 text: "some_text".to_string(),
    //                 marks: vec![],
    //                 current_version: 0,
    //                 initial_version: 0
    //             },
    //             &UpdateBlock {
    //                 id: Some(Id::from_str("1")),
    //                 order: 0,
    //                 text: "some_text".to_string(),
    //                 marks: vec![UpdateMark {
    //                     id: Some(Id::from_str("1")),
    //                     from: 1,
    //                     to: 2,
    //                 }],
    //             }
    //         )]
    //     );
    // }
    //
    // #[test]
    // fn updated_blocks_returned_if_text_same_and_marks_removed() {
    //     let existent_blocks = vec![Block {
    //         id: Id::from_str("1"),
    //         stack_id: Id::from_str("1"),
    //         order: 0,
    //         text: "some_text".to_string(),
    //         marks: vec![Mark {
    //             id: Id::from_str("1"),
    //             from: 0,
    //             to: 2,
    //         }],
    //         current_version: 0,
    //         initial_version: 0
    //     }];
    //     let updated_blocks = vec![UpdateBlock {
    //         id: Some(Id::from_str("1")),
    //         order: 0,
    //         text: "some_text".to_string(),
    //         marks: vec![],
    //     }];
    //
    //     let result = get_updated_blocks(&existent_blocks, &updated_blocks);
    //     assert_eq!(
    //         result,
    //         vec![(
    //             &Block {
    //                 id: Id::from_str("1"),
    //                 stack_id: Id::from_str("1"),
    //                 order: 0,
    //                 text: "some_text".to_string(),
    //                 marks: vec![Mark {
    //                     id: Id::from_str("1"),
    //                     from: 0,
    //                     to: 2,
    //                 }],
    //                 current_version: 0,
    //                 initial_version: 0
    //             },
    //             &UpdateBlock {
    //                 id: Some(Id::from_str("1")),
    //                 order: 0,
    //                 text: "some_text".to_string(),
    //                 marks: vec![],
    //             }
    //         )]
    //     );
    // }
    //
    // #[test]
    // fn updated_blocks_returned_if_text_same_and_marks_changed() {
    //     let existent_blocks = vec![Block {
    //         id: Id::from_str("1"),
    //         stack_id: Id::from_str("1"),
    //         order: 0,
    //         text: "some_text".to_string(),
    //         marks: vec![Mark {
    //             id: Id::from_str("1"),
    //             from: 0,
    //             to: 2,
    //         }],
    //         current_version: 0,
    //         initial_version: 0
    //     }];
    //     let updated_blocks = vec![UpdateBlock {
    //         id: Some(Id::from_str("1")),
    //         order: 0,
    //         text: "some_text".to_string(),
    //         marks: vec![UpdateMark {
    //             id: Some(Id::from_str("1")),
    //             from: 1,
    //             to: 2,
    //         }],
    //     }];
    //
    //     let result = get_updated_blocks(&existent_blocks, &updated_blocks);
    //     assert_eq!(
    //         result,
    //         vec![(
    //             &Block {
    //                 id: Id::from_str("1"),
    //                 stack_id: Id::from_str("1"),
    //                 order: 0,
    //                 text: "some_text".to_string(),
    //                 marks: vec![Mark {
    //                     id: Id::from_str("1"),
    //                     from: 0,
    //                     to: 2,
    //                 }],
    //                 current_version: 0,
    //                 initial_version: 0
    //             },
    //             &UpdateBlock {
    //                 id: Some(Id::from_str("1")),
    //                 order: 0,
    //                 text: "some_text".to_string(),
    //                 marks: vec![UpdateMark {
    //                     id: Some(Id::from_str("1")),
    //                     from: 1,
    //                     to: 2,
    //                 }],
    //             }
    //         )]
    //     );
    // }
    //
    // #[test]
    // fn updated_blocks_correct_for_multiple_different_blocks() {
    //     let existent_blocks = vec![
    //         Block {
    //             id: Id::from_str("1"),
    //             stack_id: Id::from_str("1"),
    //             order: 0,
    //             text: "some_text".to_string(),
    //             marks: vec![],
    //             current_version: 0,
    //             initial_version: 0
    //         },
    //         Block {
    //             id: Id::from_str("2"),
    //             stack_id: Id::from_str("1"),
    //             order: 1,
    //             text: "some_other_text".to_string(),
    //             marks: vec![Mark {
    //                 id: Id::from_str("1"),
    //                 from: 0,
    //                 to: 2,
    //             }],
    //             current_version: 0,
    //             initial_version: 0
    //         },
    //         Block {
    //             id: Id::from_str("3"),
    //             stack_id: Id::from_str("1"),
    //             order: 2,
    //             text: "some_other_other_text".to_string(),
    //             marks: vec![
    //                 Mark {
    //                     id: Id::from_str("3"),
    //                     from: 0,
    //                     to: 2,
    //                 },
    //                 Mark {
    //                     id: Id::from_str("4"),
    //                     from: 5,
    //                     to: 7,
    //                 },
    //             ],
    //             current_version: 0,
    //             initial_version: 0
    //         },
    //     ];
    //     let updated_blocks = vec![
    //         UpdateBlock {
    //             id: Some(Id::from_str("1")),
    //             order: 0,
    //             text: "some_text".to_string(),
    //             marks: vec![],
    //         },
    //         UpdateBlock {
    //             id: Some(Id::from_str("2")),
    //             order: 1,
    //             text: "new_some_other_text".to_string(),
    //             marks: vec![UpdateMark {
    //                 id: Some(Id::from_str("1")),
    //                 from: 0,
    //                 to: 2,
    //             }],
    //         },
    //         UpdateBlock {
    //             id: Some(Id::from_str("3")),
    //             order: 2,
    //             text: "some_other_other_text".to_string(),
    //             marks: vec![
    //                 UpdateMark {
    //                     id: Some(Id::from_str("3")),
    //                     from: 0,
    //                     to: 2,
    //                 },
    //                 UpdateMark {
    //                     id: Some(Id::from_str("4")),
    //                     from: 5,
    //                     to: 7,
    //                 },
    //             ],
    //         },
    //     ];
    //
    //     let result = get_updated_blocks(&existent_blocks, &updated_blocks);
    //     assert_eq!(result, vec![(
    //         &Block {
    //             id: Id::from_str("2"),
    //             stack_id: Id::from_str("1"),
    //             order: 0,
    //             text: "some_other_text".to_string(),
    //             marks: vec![Mark {
    //                 id: Id::from_str("1"),
    //                 from: 0,
    //                 to: 2,
    //             }],
    //             current_version: 0,
    //             initial_version: 0
    //         },
    //         &UpdateBlock {
    //             id: Some(Id::from_str("2")),
    //             order: 0,
    //             text: "new_some_other_text".to_string(),
    //             marks: vec![UpdateMark {
    //                 id: Some(Id::from_str("1")),
    //                 from: 0,
    //                 to: 2,
    //             }],
    //         },
    //     )])
    // }
}
