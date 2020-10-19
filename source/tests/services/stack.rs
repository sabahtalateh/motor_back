use crate::setup_with_default_user;
use motor_back::container::Container;
use motor_back::errors::AppError;
use motor_back::handlers::stack::{NewBlock, NewStackItem, NewMark};
use motor_back::repos::users::User;
use motor_back::services::stack::StackServiceIf;
use shaku::HasComponent;

#[actix_rt::test]
async fn can_not_add_emtpy_item() {
    let (ctr, user): (Container, User) = setup_with_default_user().await;
    let stack: &dyn StackServiceIf = ctr.resolve_ref();
    let result = stack
        .add_to_my_stack(
            user,
            NewStackItem {
                title: None,
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
async fn item_with_title_added() {
    let (ctr, user): (Container, User) = setup_with_default_user().await;
    let stack: &dyn StackServiceIf = ctr.resolve_ref();
    let result = stack
        .add_to_my_stack(
            user,
            NewStackItem {
                title: Some("title".to_string()),
                blocks: vec![],
            },
        )
        .await
        .unwrap();

    assert_eq!(&result.title.unwrap(), "title");
}

#[actix_rt::test]
async fn item_with_empty_block_added() {
    let (ctr, user): (Container, User) = setup_with_default_user().await;
    let stack: &dyn StackServiceIf = ctr.resolve_ref();
    let result = stack
        .add_to_my_stack(
            user,
            NewStackItem {
                title: None,
                blocks: vec![NewBlock {
                    text: "".to_string(),
                    marks: vec![],
                }],
            },
        )
        .await
        .unwrap();

    assert!(&result.title.is_none());
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
                title: None,
                blocks: vec![NewBlock {
                    text: "Block!".to_string(),
                    marks: vec![
                        NewMark {
                            from: 0,
                            to: 2
                        }
                    ],
                }],
            },
        )
        .await
        .unwrap();

    assert!(&result.title.is_none());
    assert_eq!(result.blocks.len(), 1 as usize);

    let first_block = result.blocks.get(0).unwrap();
    assert_eq!(first_block.text, "Block!");
    assert_eq!(first_block.marks.len(), 1 as usize);

    let first_mark = first_block.marks.get(0).unwrap();
    assert_eq!(first_mark.from, 0);
    assert_eq!(first_mark.to, 2);
}
