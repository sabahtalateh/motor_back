use crate::{setup_with_default_user, trunc_collection};
use bson::oid::ObjectId;
use motor_back::container::Container;
use motor_back::db::DBIf;
use motor_back::errors::AppError;
use motor_back::handlers::stack::{NewStackItem, NewBlock, NewMark, UpdateBlock, UpdateStackItem};
use motor_back::logger::AppLoggerIf;
use motor_back::repos::db as ddbb;
use motor_back::repos::marks::InsertMark;
use motor_back::repos::users::User;
use motor_back::repos::Id;
use motor_back::services::stack::{StackItem, StackService, StackServiceIf};
use shaku::HasComponent;

#[actix_rt::test]
async fn can_not_add_emtpy_item() {
    let (ctr, user): (Container, User) = setup_with_default_user().await;
    let stack: &dyn StackServiceIf = ctr.resolve_ref();
    let result = stack
        .add_to_my_stack(user, NewStackItem { blocks: vec![] })
        .await;

    assert_eq!(
        result.map(|_| ()),
        Err(AppError::validation("Can not add empty stacks item"))
    );
}

#[actix_rt::test]
async fn item_with_empty_block_added() {
    let (ctr, user): (Container, User) = setup_with_default_user().await;
    let stack: &dyn StackServiceIf = ctr.resolve_ref();
    let result = stack
        .add_to_my_stack(
            user,
            NewStackItem {
                blocks: vec![NewBlock {
                    order: 0,
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
            NewStackItem {
                blocks: vec![NewBlock {
                    order: 0,
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

// TODO тест на корректности order в новых блоках

#[actix_rt::test]
async fn upd() {
    let (ctr, user): (Container, User) = setup_with_default_user().await;
    let stack: &dyn StackServiceIf = ctr.resolve_ref();

    let db: &dyn DBIf = ctr.resolve_ref();
    trunc_collection(&db.get(), "stack_history").await;
    trunc_collection(&db.get(), "marks").await;
    trunc_collection(&db.get(), "blocks").await;
    trunc_collection(&db.get(), "stacks").await;

    // Add stacks item
    let result = stack
        .add_to_my_stack(
            user.clone(),
            NewStackItem {
                blocks: vec![
                    NewBlock {
                        order: 0,
                        text: "Block 0".to_string(),
                        marks: vec![],
                    },
                    NewBlock {
                        order: 1,
                        text: "Block 1".to_string(),
                        marks: vec![NewMark { from: 0, to: 2 }],
                    },
                    NewBlock {
                        order: 2,
                        text: "Block 2 Hello! Hellllllo!!".to_string(),
                        marks: vec![NewMark { from: 0, to: 2 }, NewMark { from: 4, to: 8 }],
                    },
                ],
            },
        )
        .await
        .unwrap();

    stack
        .update_stack_item(
            user,
            UpdateStackItem {
                id: result.id,
                blocks: vec![UpdateBlock {
                    id: Some(result.blocks.get(0).unwrap().clone().id),
                    text: "123".to_string(),
                    marks: vec![],
                }],
            },
        )
        .await;

    // println!("{:#?}", result);

    // Update stacks item
    // let nn = stack.update_stack_item(
    //     user,
    //     UpdateStackItem {
    //         id: result.id,
    //         blocks: vec![
    //             UpdateBlock {
    //                 id: None,
    //                 text: "New Block Text".to_string(),
    //                 marks: vec![]
    //             },
    //             UpdateBlock {
    //                 id: Some(Id("123".to_string())),
    //                 text: "New Block Text".to_string(),
    //                 marks: vec![]
    //             },
    //             UpdateBlock {
    //                 id: Some(result.blocks.get(0).unwrap().clone().id),
    //                 text: "zzz".to_string(),
    //                 marks: vec![]
    //             }
    //         ],
    //     },
    // ).await;

    // StackService::extract_new_blocks(stacks, )

    // println!("{:#?}", nn);
}

// TODO тест - снапшот на такой то момент
// TODO тест на

#[actix_rt::test]
async fn jj() {
    let (ctr, user): (Container, User) = setup_with_default_user().await;
    let stack: &dyn StackServiceIf = ctr.resolve_ref();
    let logger: &dyn AppLoggerIf = ctr.resolve_ref();

    let db: &dyn DBIf = ctr.resolve_ref();

    // let bb = ddbb::insert_many_into(&db.get(), "marks", &vec![InsertMark {
    //     block_id: ObjectId::new().into(),
    //     from: 100,
    //     to: 500
    // }],
    //                        logger
    // ).await;
    // println!("{:#?}", bb);
}
