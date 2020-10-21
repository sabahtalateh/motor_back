mod diff;

use crate::{setup_with_default_user, trunc_collection};
use motor_back::container::Container;
use motor_back::errors::AppError;
use motor_back::handlers::stack::{AddStackItem, NewBlock, NewMark, UpdateStackItem, UpdateBlock};
use motor_back::repos::users::User;
use motor_back::services::stack::{StackServiceIf, StackService, StackItem};
use shaku::HasComponent;
use motor_back::db::DBIf;
use motor_back::repos::Id;

#[actix_rt::test]
async fn can_not_add_emtpy_item() {
    let (ctr, user): (Container, User) = setup_with_default_user().await;
    let stack: &dyn StackServiceIf = ctr.resolve_ref();
    let result = stack
        .add_to_my_stack(
            user,
            AddStackItem {
                blocks: vec![],
            },
        )
        .await;

    assert_eq!(
        result.map(|_| ()),
        Err(AppError::validation("Can not add empty stack item"))
    );
}

#[actix_rt::test]
async fn item_with_empty_block_added() {
    let (ctr, user): (Container, User) = setup_with_default_user().await;
    let stack: &dyn StackServiceIf = ctr.resolve_ref();
    let result = stack
        .add_to_my_stack(
            user,
            AddStackItem {
                blocks: vec![NewBlock {
                    text: "".to_string(),
                    marks: vec![],
                }],
            },
        )
        .await
        .unwrap();

    assert_eq!(result.blocks.len(), 1 as usize);
}

#[actix_rt::test]
async fn item_with_blocks_and_marks_added() {
    let (ctr, user): (Container, User) = setup_with_default_user().await;
    let stack: &dyn StackServiceIf = ctr.resolve_ref();
    let result = stack
        .add_to_my_stack(
            user,
            AddStackItem {
                blocks: vec![NewBlock {
                    text: "Block!".to_string(),
                    marks: vec![NewMark { from: 0, to: 2 }],
                }],
            },
        )
        .await
        .unwrap();

    assert_eq!(result.blocks.len(), 1 as usize);

    let first_block = result.blocks.get(0).unwrap();
    assert_eq!(first_block.text, "Block!");
    assert_eq!(first_block.marks.len(), 1 as usize);

    let first_mark = first_block.marks.get(0).unwrap();
    assert_eq!(first_mark.from, 0);
    assert_eq!(first_mark.to, 2);
}

// #[actix_rt::test]
async fn upd() {
    let (ctr, user): (Container, User) = setup_with_default_user().await;
    let stack: &dyn StackServiceIf = ctr.resolve_ref();

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "marks").await;
    trunc_collection(&db.get(), "blocks").await;
    trunc_collection(&db.get(), "stack").await;

    // Add stack item
    let result = stack
        .add_to_my_stack(
            user.clone(),
            AddStackItem {
                blocks: vec![
                    NewBlock {
                        text: "Block 0".to_string(),
                        marks: vec![],
                    },
                    NewBlock {
                        text: "Block 1".to_string(),
                        marks: vec![NewMark { from: 0, to: 2 }],
                    },
                    NewBlock {
                        text: "Block 2 Hello! Hellllllo!!".to_string(),
                        marks: vec![NewMark { from: 0, to: 2 }, NewMark { from: 4, to: 8 }],
                    },
                ],
            },
        )
        .await
        .unwrap();

    // Update stack item
    let nn = stack.update_stack_item(
        user,
        UpdateStackItem {
            id: result.id,
            blocks: vec![
                UpdateBlock {
                    id: None,
                    text: "New Block Text".to_string(),
                    marks: vec![]
                },
                UpdateBlock {
                    id: Some(Id("123".to_string())),
                    text: "New Block Text".to_string(),
                    marks: vec![]
                },
                UpdateBlock {
                    id: Some(result.blocks.get(0).unwrap().clone().id),
                    text: "zzz".to_string(),
                    marks: vec![]
                }
            ],
        },
    ).await;

    // StackService::extract_new_blocks(stack, )

    println!("{:#?}", nn);
}
