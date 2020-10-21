use motor_back::handlers::stack::{UpdateBlock, UpdateMark};
use motor_back::repos::Id;
use motor_back::services::stack::{diff, Block, Mark};

#[test]
fn new_blocks_returns_empty_result_if_both_args_has_empty_blocks() {
    let existent_blocks = vec![];
    let updated_blocks = vec![];

    let added_blocks = diff::get_new_blocks(&existent_blocks, &updated_blocks);
    assert!(added_blocks.is_empty());
}

#[test]
fn new_blocks_returns_nothing_if_updated_blocks_empty() {
    let existent_blocks = vec![
        Block {
            id: Id("1".to_string()),
            text: "some_test".to_string(),
            marks: vec![],
        },
        Block {
            id: Id("2".to_string()),
            text: "some_other_text".to_string(),
            marks: vec![Mark {
                id: Id("1".to_string()),
                from: 0,
                to: 3,
            }],
        },
    ];
    let updated_blocks = vec![];

    let added_blocks = diff::get_new_blocks(&existent_blocks, &updated_blocks);
    assert!(added_blocks.is_empty());
}

#[test]
fn new_blocks_returns_all_updated_blocks_if_existent_blocks_empty_1() {
    let existent_blocks = vec![];
    let updated_blocks = vec![UpdateBlock {
        id: None,
        text: "block_1".to_string(),
        marks: vec![],
    }];

    let added_blocks = diff::get_new_blocks(&existent_blocks, &updated_blocks);
    assert_eq!(
        added_blocks,
        vec![&UpdateBlock {
            id: None,
            text: "block_1".to_string(),
            marks: vec![]
        }]
    );
}

#[test]
fn new_blocks_returns_all_updated_blocks_if_existent_blocks_empty_2() {
    let existent_blocks = vec![];
    let updated_blocks = vec![UpdateBlock {
        id: None,
        text: "block_1".to_string(),
        marks: vec![UpdateMark {
            id: None,
            from: 0,
            to: 3,
        }],
    }];

    let added_blocks = diff::get_new_blocks(&existent_blocks, &updated_blocks);
    assert_eq!(
        added_blocks,
        vec![&UpdateBlock {
            id: None,
            text: "block_1".to_string(),
            marks: vec![UpdateMark {
                id: None,
                from: 0,
                to: 3
            }]
        }]
    );
}

#[test]
fn new_blocks_returns_all_updated_blocks_if_existent_blocks_empty_3() {
    let existent_blocks = vec![];
    let updated_blocks = vec![UpdateBlock {
        id: None,
        text: "block_1".to_string(),
        marks: vec![
            UpdateMark {
                id: None,
                from: 0,
                to: 3,
            },
            UpdateMark {
                id: Some(Id("1".to_string())),
                from: 2,
                to: 4,
            },
        ],
    }];

    let added_blocks = diff::get_new_blocks(&existent_blocks, &updated_blocks);
    assert_eq!(
        added_blocks,
        vec![&UpdateBlock {
            id: None,
            text: "block_1".to_string(),
            marks: vec![
                UpdateMark {
                    id: None,
                    from: 0,
                    to: 3
                },
                UpdateMark {
                    id: Some(Id("1".to_string())),
                    from: 2,
                    to: 4,
                },
            ]
        }]
    );
}

#[test]
fn new_blocks_returns_blocks_with_ids_that_absent_in_existent() {
    let existent_blocks = vec![Block {
        id: Id::from_str("1"),
        text: "some_text".to_string(),
        marks: vec![],
    }];
    let updated_blocks = vec![
        // this block should not be presented in added blocks
        UpdateBlock {
            id: Some(Id::from_str("1")),
            text: "block_1".to_string(),
            marks: vec![],
        },
        UpdateBlock {
            id: None,
            text: "block_1".to_string(),
            marks: vec![],
        },
        UpdateBlock {
            id: Some(Id::from_str("2")),
            text: "block_2".to_string(),
            marks: vec![],
        },
    ];

    let added_blocks = diff::get_new_blocks(&existent_blocks, &updated_blocks);
    assert_eq!(
        added_blocks,
        vec![
            &UpdateBlock {
                id: None,
                text: "block_1".to_string(),
                marks: vec![],
            },
            &UpdateBlock {
                id: Some(Id::from_str("2")),
                text: "block_2".to_string(),
                marks: vec![],
            },
        ]
    );
}

#[test]
fn deleted_blocks_returns_blocks_not_presented_in_updated_blocks_1() {
    let existent_blocks = vec![Block {
        id: Id::from_str("1"),
        text: "some_text".to_string(),
        marks: vec![],
    }];
    let updated_blocks = vec![];
    let deleted_blocks = diff::get_deleted_blocks(&existent_blocks, &updated_blocks);
    assert_eq!(
        deleted_blocks,
        vec![&Block {
            id: Id::from_str("1"),
            text: "some_text".to_string(),
            marks: vec![],
        }]
    );
}

#[test]
fn deleted_blocks_returns_blocks_not_presented_in_updated_blocks_2() {
    let existent_blocks = vec![Block {
        id: Id::from_str("1"),
        text: "some_text".to_string(),
        marks: vec![],
    }];
    let updated_blocks = vec![UpdateBlock {
        id: Some(Id::from_str("2")),
        text: "some_other_text".to_string(),
        marks: vec![],
    }];

    let deleted_blocks = diff::get_deleted_blocks(&existent_blocks, &updated_blocks);

    assert_eq!(
        deleted_blocks,
        vec![&Block {
            id: Id::from_str("1"),
            text: "some_text".to_string(),
            marks: vec![],
        }]
    );
}
