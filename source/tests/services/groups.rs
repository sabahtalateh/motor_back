use crate::{drop_and_setup_with_default_user, drop_db, setup_with_default_user, trunc_collection};
use bson::oid::ObjectId;
use motor_back::container::Container;
use motor_back::db::DBIf;
use motor_back::errors::AppError;
use motor_back::handlers::groups::UserGroup;
use motor_back::handlers::stack::{
    NewBlock, NewMark, NewStackItem, StackItemChangeSet, UpdateBlock,
};
use motor_back::logger::AppLoggerIf;
use motor_back::repos::db as ddbb;
use motor_back::repos::marks::InsertMark;
use motor_back::repos::users::User;
use motor_back::repos::Id;
use motor_back::services::groups::{GroupsServiceIf, PAGING_MAX_LIMIT};
use motor_back::services::stack::{StackItem, StackService, StackServiceIf};
use motor_back::services::Paging;
use shaku::HasComponent;

#[actix_rt::test]
async fn can_not_get_groups_if_pagination_limit_too_big() {
    let (ctr, user): (Container, User) = drop_and_setup_with_default_user().await;
    let groups_service: &dyn GroupsServiceIf = ctr.resolve_ref();

    let res = groups_service
        .list(
            &user,
            &Paging {
                offset: 0,
                limit: PAGING_MAX_LIMIT + 1,
            },
        )
        .await;

    assert_eq!(
        res.map(|_| ()),
        Err(AppError::validation(&format!(
            "Paging limit can not be more then {}",
            PAGING_MAX_LIMIT
        )))
    );
}

#[actix_rt::test]
async fn can_create_group_when_no_groups_created() {
    let (ctr, user): (Container, User) = drop_and_setup_with_default_user().await;
    let groups_service: &dyn GroupsServiceIf = ctr.resolve_ref();

    let group = groups_service.add(&user, "some group", None).await.unwrap();

    assert_eq!("some group", &group.name);
    assert_eq!(0, group.order);
}

#[actix_rt::test]
async fn groups_presented_after_insertion() {
    let (ctr, user): (Container, User) = drop_and_setup_with_default_user().await;
    let groups_service: &dyn GroupsServiceIf = ctr.resolve_ref();

    let inserted_group_0 = groups_service.add(&user, "group 0", None).await.unwrap();
    let inserted_group_1 = groups_service
        .add(&user, "group 1", Some(&inserted_group_0.id))
        .await
        .unwrap();

    // let group_0 = groups_service
    //     .list(
    //         &user,
    //         &Paging {
    //             offset: 0,
    //             limit: 1,
    //         },
    //     )
    //     .await
    //     .unwrap()
    //     .get(0)
    //     .unwrap()
    //     .clone();
    // assert_eq!(group_0, inserted_group_0);
    //
    // let group_1 = groups_service
    //     .list(
    //         &user,
    //         &Paging {
    //             offset: 1,
    //             limit: 1,
    //         },
    //     )
    //     .await
    //     .unwrap()
    //     .get(0)
    //     .unwrap()
    //     .clone();
    // assert_eq!(group_1, inserted_group_1);
}

// #[actix_rt::test]
// async fn () {
//     let (ctr, user): (Container, User) = drop_and_setup_with_default_user().await;
//     let groups_service: &dyn GroupsServiceIf = ctr.resolve_ref();
// }
