use shaku::HasComponent;

use motor_back::container::Container;
use motor_back::db::DBIf;
use motor_back::errors::AppError;
use motor_back::handlers::stack::{
    NewBlock, NewMark, NewStackItem, StackItemChangeSet, UpdateBlock,
};
use motor_back::logger::AppLoggerIf;
use motor_back::repos::users::User;
use motor_back::services::stack::StackServiceIf;

use crate::{setup_with_random_user, trunc_collection};

// #[actix_rt::test]
// async fn can_not_add_emtpy_item() {
//     let (ctr, user): (Container, User) = setup_with_random_user().await;
//     let stack: &dyn StackServiceIf = ctr.resolve_ref();
//     let result = stack
//         .add_to_my_stack(user, NewStackItem { blocks: vec![] })
//         .await;
//
//     assert_eq!(
//         result.map(|_| ()),
//         Err(AppError::validation("Can not add empty stack item"))
//     );
// }
//
// #[actix_rt::test]
// async fn item_with_empty_block_added() {
//     let (ctr, user): (Container, User) = setup_with_random_user().await;
//     let stack: &dyn StackServiceIf = ctr.resolve_ref();
//     let result = stack
//         .add_to_my_stack(
//             user,
//             NewStackItem {
//                 blocks: vec![NewBlock {
//                     text: "".to_string(),
//                     marks: vec![],
//                 }],
//             },
//         )
//         .await
//         .unwrap();
//
//     assert_eq!(result.blocks.len(), 1 as usize);
// }
//
// #[actix_rt::test]
// async fn error_if_orders_duplicated() {
//     let (ctr, user): (Container, User) = setup_with_random_user().await;
//     let stack: &dyn StackServiceIf = ctr.resolve_ref();
//     let _result = stack
//         .add_to_my_stack(
//             user,
//             NewStackItem {
//                 blocks: vec![
//                     NewBlock {
//                         text: "".to_string(),
//                         marks: vec![],
//                     },
//                     NewBlock {
//                         text: "".to_string(),
//                         marks: vec![],
//                     },
//                 ],
//             },
//         )
//         .await;
//
//     // assert_eq!(
//     //     result.map(|_| ()),
//     //     Err(AppError::validation("Duplicated orders occurred"))
//     // );
// }
//
// #[actix_rt::test]
// async fn orders_recounted_when_no_sequential() {
//     let (ctr, user): (Container, User) = setup_with_random_user().await;
//     let stack: &dyn StackServiceIf = ctr.resolve_ref();
//
//     let db: &dyn DBIf = ctr.resolve_ref();
//     trunc_collection(&db.get(), "stack_history").await;
//     trunc_collection(&db.get(), "marks").await;
//     trunc_collection(&db.get(), "blocks").await;
//     trunc_collection(&db.get(), "stacks").await;
//
//     // Add stacks item
//     let result = stack
//         .add_to_my_stack(
//             user.clone(),
//             NewStackItem {
//                 blocks: vec![
//                     NewBlock {
//                         text: "Block 0".to_string(),
//                         marks: vec![],
//                     },
//                     NewBlock {
//                         text: "Block 1".to_string(),
//                         marks: vec![],
//                     },
//                     NewBlock {
//                         text: "Block 2 Hello! Hellllllo!!".to_string(),
//                         marks: vec![],
//                     },
//                 ],
//             },
//         )
//         .await
//         .unwrap();
//
//     assert_eq!(0, result.blocks.get(0).unwrap().order);
//     assert_eq!(1, result.blocks.get(1).unwrap().order);
//     assert_eq!(2, result.blocks.get(2).unwrap().order);
// }
//
// #[actix_rt::test]
// async fn item_with_blocks_and_marks_added() {
//     let (ctr, user): (Container, User) = setup_with_random_user().await;
//     let stack: &dyn StackServiceIf = ctr.resolve_ref();
//     let result = stack
//         .add_to_my_stack(
//             user,
//             NewStackItem {
//                 blocks: vec![NewBlock {
//                     text: "Block!".to_string(),
//                     marks: vec![NewMark { from: 0, to: 2 }],
//                 }],
//             },
//         )
//         .await
//         .unwrap();
//
//     assert_eq!(result.blocks.len(), 1 as usize);
//
//     let first_block = result.blocks.get(0).unwrap();
//     assert_eq!(first_block.text, "Block!");
//     assert_eq!(first_block.marks.len(), 1 as usize);
//
//     let first_mark = first_block.marks.get(0).unwrap();
//     assert_eq!(first_mark.from, 0);
//     assert_eq!(first_mark.to, 2);
// }
//
// #[actix_rt::test]
// async fn error_id_deleted_and_updated_ids_intersects() {
//     let (ctr, user): (Container, User) = setup_with_random_user().await;
//     let stack: &dyn StackServiceIf = ctr.resolve_ref();
//
//     let db: &dyn DBIf = ctr.resolve_ref();
//     trunc_collection(&db.get(), "stack_history").await;
//     trunc_collection(&db.get(), "marks").await;
//     trunc_collection(&db.get(), "blocks").await;
//     trunc_collection(&db.get(), "stacks").await;
//
//     // Add stacks item
//     let result = stack
//         .add_to_my_stack(
//             user.clone(),
//             NewStackItem {
//                 blocks: vec![
//                     NewBlock {
//                         text: "Block 0".to_string(),
//                         marks: vec![],
//                     },
//                     NewBlock {
//                         text: "Block 1".to_string(),
//                         marks: vec![NewMark { from: 0, to: 2 }],
//                     },
//                     NewBlock {
//                         text: "Block 2 Hello! Hellllllo!!".to_string(),
//                         marks: vec![NewMark { from: 0, to: 2 }, NewMark { from: 4, to: 8 }],
//                     },
//                 ],
//             },
//         )
//         .await
//         .unwrap();
//
//     let result = stack
//         .update_stack_item(
//             user,
//             StackItemChangeSet {
//                 stack_id: result.id,
//                 inserted: None,
//                 removed: vec![result.blocks.get(1).unwrap().clone().id],
//                 updated: vec![UpdateBlock {
//                     id: result.blocks.get(1).unwrap().clone().id,
//                     text: "123".to_string(),
//                     marks: vec![],
//                 }],
//             },
//         )
//         .await;
//
//     assert_eq!(
//         result.map(|_| ()),
//         Err(AppError::validation(
//             "updated and removed changes intersects"
//         ))
//     );
// }
//
// #[actix_rt::test]
// async fn upd() {
//     let (ctr, user): (Container, User) = setup_with_random_user().await;
//     let stack: &dyn StackServiceIf = ctr.resolve_ref();
//
//     let db: &dyn DBIf = ctr.resolve_ref();
//     trunc_collection(&db.get(), "stack_history").await;
//     trunc_collection(&db.get(), "marks").await;
//     trunc_collection(&db.get(), "blocks").await;
//     trunc_collection(&db.get(), "stacks").await;
//
//     // Add stacks item
//     let result = stack
//         .add_to_my_stack(
//             user.clone(),
//             NewStackItem {
//                 blocks: vec![
//                     NewBlock {
//                         text: "Block 0".to_string(),
//                         marks: vec![],
//                     },
//                     NewBlock {
//                         text: "Block 1".to_string(),
//                         marks: vec![NewMark { from: 0, to: 2 }],
//                     },
//                     NewBlock {
//                         text: "Block 2 Hello! Hellllllo!!".to_string(),
//                         marks: vec![NewMark { from: 0, to: 2 }, NewMark { from: 4, to: 8 }],
//                     },
//                 ],
//             },
//         )
//         .await
//         .unwrap();
//
//     let nn = stack
//         .update_stack_item(
//             user,
//             StackItemChangeSet {
//                 stack_id: result.id,
//                 inserted: None,
//                 removed: vec![result.blocks.get(1).unwrap().clone().id],
//                 updated: vec![UpdateBlock {
//                     id: result.blocks.get(2).unwrap().clone().id,
//                     text: "123".to_string(),
//                     marks: vec![],
//                 }],
//             },
//         )
//         .await;
//
//     println!("{:#?}", nn);
//
//     // println!("{:#?}", result);
//
//     // Update stacks item
//     // let nn = stack.update_stack_item(
//     //     user,
//     //     UpdateStackItem {
//     //         id: result.id,
//     //         blocks: vec![
//     //             UpdateBlock {
//     //                 id: None,
//     //                 text: "New Block Text".to_string(),
//     //                 marks: vec![]
//     //             },
//     //             UpdateBlock {
//     //                 id: Some(Id("123".to_string())),
//     //                 text: "New Block Text".to_string(),
//     //                 marks: vec![]
//     //             },
//     //             UpdateBlock {
//     //                 id: Some(result.blocks.get(0).unwrap().clone().id),
//     //                 text: "zzz".to_string(),
//     //                 marks: vec![]
//     //             }
//     //         ],
//     //     },
//     // ).await;
//
//     // StackService::extract_new_blocks(stacks, )
//
//     // println!("{:#?}", nn);
// }
//
// // TODO тест - снапшот на такой то момент
// // TODO тест на кукуху
//
// #[actix_rt::test]
// async fn jj() {
//     let (ctr, _user): (Container, User) = setup_with_random_user().await;
//     let _stack: &dyn StackServiceIf = ctr.resolve_ref();
//     let _logger: &dyn AppLoggerIf = ctr.resolve_ref();
//
//     let _db: &dyn DBIf = ctr.resolve_ref();
//
//     // let bb = ddbb::insert_many_into(&db.get(), "marks", &vec![InsertMark {
//     //     block_id: ObjectId::new().into(),
//     //     from: 100,
//     //     to: 500
//     // }],
//     //                        logger
//     // ).await;
//     // println!("{:#?}", bb);
// }
