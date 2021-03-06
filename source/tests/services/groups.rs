use crate::drop_and_setup_with_random_user;
use motor_back::container::Container;
use motor_back::db::DBIf;
use motor_back::errors::AppError;
use motor_back::handlers::groups::UserGroup;
use motor_back::handlers::Paging;
use motor_back::repos::users::User;
use motor_back::repos::Id;
use motor_back::services::groups::{GroupsServiceIf, IntoSet, PAGING_MAX_LIMIT};
use motor_back::services::Paged;
use shaku::HasComponent;

#[actix_rt::test]
async fn can_not_get_groups_if_pagination_limit_too_big() {
    let (ctr, user): (Container, User) = drop_and_setup_with_random_user().await;
    let groups_service: &dyn GroupsServiceIf = ctr.resolve_ref();

    let res = groups_service
        .list_groups(
            user,
            IntoSet::Default,
            Some(Paging {
                offset: Some(0),
                limit: Some(PAGING_MAX_LIMIT + 1),
            }),
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
    let (ctr, user): (Container, User) = drop_and_setup_with_random_user().await;
    let groups_service: &dyn GroupsServiceIf = ctr.resolve_ref();

    let group = groups_service
        .create_group(user, "some group".to_string(), IntoSet::Default, None)
        .await
        .unwrap();

    assert_eq!("some group", &group.name);
    assert_eq!(0, group.order);
}

#[actix_rt::test]
async fn default_group_set_empty_if_nothing_inserted_in_it() {
    let (ctr, user): (Container, User) = drop_and_setup_with_random_user().await;
    let groups_service: &dyn GroupsServiceIf = ctr.resolve_ref();

    let response = groups_service
        .list_groups(
            user,
            IntoSet::Default,
            Some(Paging {
                offset: Some(0),
                limit: Some(100),
            }),
        )
        .await
        .unwrap();

    assert_eq!(response.objects, vec![]);
}

#[actix_rt::test]
async fn group_set_empty_if_nothing_inserted_in_it() {
    let (ctr, user): (Container, User) = drop_and_setup_with_random_user().await;
    let groups_service: &dyn GroupsServiceIf = ctr.resolve_ref();

    let response = groups_service
        .list_groups(
            user,
            IntoSet::Named("some_set".to_string()),
            Some(Paging {
                offset: Some(0),
                limit: Some(100),
            }),
        )
        .await
        .unwrap();

    assert_eq!(response.objects, vec![]);
}

#[actix_rt::test]
async fn pagination_params_returned_same_as_passed() {
    let (ctr, user): (Container, User) = drop_and_setup_with_random_user().await;
    let groups_service: &dyn GroupsServiceIf = ctr.resolve_ref();

    let response: Paged<UserGroup> = groups_service
        .list_groups(
            user,
            IntoSet::Default,
            Some(Paging {
                offset: Some(19),
                limit: Some(84),
            }),
        )
        .await
        .unwrap();

    assert_eq!(response.page_info.offset, 19);
    assert_eq!(response.page_info.limit, 84);
    assert_eq!(response.page_info.total, None);
}

#[actix_rt::test]
async fn groups_presented_after_insertion_in_default_set() {
    let (ctr, user): (Container, User) = drop_and_setup_with_random_user().await;
    let groups_service: &dyn GroupsServiceIf = ctr.resolve_ref();

    let inserted_group_0 = groups_service
        .create_group(user.clone(), "group 0".to_string(), IntoSet::Default, None)
        .await
        .unwrap();

    let inserted_group_1 = groups_service
        .create_group(
            user.clone(),
            "group 1".to_string(),
            IntoSet::Default,
            Some(inserted_group_0.clone().id),
        )
        .await
        .unwrap();

    let groups = groups_service
        .list_groups(
            user.clone(),
            IntoSet::Default,
            Some(Paging {
                offset: Some(0),
                limit: Some(2),
            }),
        )
        .await
        .unwrap()
        .objects;

    println!("{:#?}", groups);

    let group_0 = groups.get(0).unwrap().clone();
    assert_eq!(group_0.id, inserted_group_0.id);
    assert_eq!(group_0.name, inserted_group_0.name);
    assert_eq!(group_0.order, inserted_group_0.order);

    let group_1 = groups.get(1).unwrap().clone();
    assert_eq!(group_1.id, inserted_group_1.id);
    assert_eq!(group_1.name, inserted_group_1.name);
    assert_eq!(group_1.order, inserted_group_1.order);
}

// TODO
// Тест на добавление группы в разные сеты
// Чо делать когда одна группа в нескольких сетах, сколько раз она в дефолтном сете

// #[actix_rt::test]
// async fn groups_inserted_in_correct_order() {
//     let (ctr, user): (Container, User) = drop_and_setup_with_random_user().await;
//     let groups_service: &dyn GroupsServiceIf = ctr.resolve_ref();
//
//     let inserted_group_0 = groups_service.create_group(&user, "group 0", None).await.unwrap();
//     let inserted_group_1 = groups_service
//         .create_group(&user, "group 1", Some(&inserted_group_0.id))
//         .await
//         .unwrap();
//     let inserted_group_2 = groups_service
//         .create_group(&user, "group 2", Some(&inserted_group_1.id))
//         .await
//         .unwrap();
//
//     let group_0 = groups_service
//         .list(
//             &user,
//             &Paging {
//                 offset: 0,
//                 limit: 1,
//             },
//         )
//         .await
//         .unwrap()
//         .objects
//         .get(0)
//         .unwrap()
//         .clone();
//
//     assert_eq!(group_0, inserted_group_0);
//
//     let groups_1_and_2 = groups_service
//         .list(
//             &user,
//             &Paging {
//                 offset: 1,
//                 limit: 10,
//             },
//         )
//         .await
//         .unwrap()
//         .objects;
//
//     assert_eq!(groups_1_and_2.get(0).unwrap().clone(), inserted_group_1);
//     assert_eq!(groups_1_and_2.get(1).unwrap().clone(), inserted_group_2);
// }
//
// #[actix_rt::test]
// async fn error_when_removing_non_existing_group() {
//     let (ctr, user): (Container, User) = drop_and_setup_with_random_user().await;
//     let groups_service: &dyn GroupsServiceIf = ctr.resolve_ref();
//
//     let inserted_group_200 = groups_service.create_group(&user, "200", None).await.unwrap();
//
//     let remove_result = groups_service
//         .remove(&user, &Id::from_str("no group with such id"))
//         .await;
//
//     assert_eq!(
//         remove_result,
//         Err(AppError::validation(
//             "Group `Id(no group with such id)` you are trying to remove not exists"
//         ))
//     );
//
//     let removed_group = groups_service
//         .remove(&user, &inserted_group_200.id)
//         .await
//         .unwrap();
//
//     assert_eq!(removed_group.name, "200");
// }
//
// #[actix_rt::test]
// async fn can_not_remove_twice() {
//     let (ctr, user): (Container, User) = drop_and_setup_with_random_user().await;
//     let groups_service: &dyn GroupsServiceIf = ctr.resolve_ref();
//
//     let inserted_group_200 = groups_service.create_group(&user, "200", None).await.unwrap();
//
//     let removed_group = groups_service
//         .remove(&user, &inserted_group_200.id)
//         .await
//         .unwrap();
//
//     assert_eq!(removed_group.name, "200");
//
//     let remove_result = groups_service.remove(&user, &inserted_group_200.id).await;
//
//     assert_eq!(
//         remove_result,
//         Err(AppError::validation(&format!(
//             "Group `{}` you are trying to remove not exists",
//             inserted_group_200.id
//         )))
//     );
// }
//
// #[actix_rt::test]
// async fn check_groups_ordering_recounted_after_insertion_and_deletion() {
//     let (ctr, user): (Container, User) = drop_and_setup_with_random_user().await;
//     let groups_service: &dyn GroupsServiceIf = ctr.resolve_ref();
//
//     // Insert some groups and ensure ordering is correct
//     let _inserted_group_200 = groups_service.create_group(&user, "200", None).await.unwrap();
//     let inserted_group_100 = groups_service.create_group(&user, "100", None).await.unwrap();
//     let _inserted_group_150 = groups_service
//         .create_group(&user, "150", Some(&inserted_group_100.id))
//         .await
//         .unwrap();
//     let inserted_group_125 = groups_service
//         .create_group(&user, "125", Some(&inserted_group_100.id))
//         .await
//         .unwrap();
//
//     let paged_groups = groups_service
//         .list(
//             &user,
//             &Paging {
//                 offset: 0,
//                 limit: 10,
//             },
//         )
//         .await
//         .unwrap();
//
//     // assert pagination
//     assert_eq!(paged_groups.total, 4);
//     assert_eq!(paged_groups.limit, 10);
//     assert_eq!(paged_groups.offset, 0);
//
//     // assert objects
//     assert_eq!(paged_groups.objects.get(0).unwrap().order, 0);
//     assert_eq!(paged_groups.objects.get(0).unwrap().name, "100");
//
//     assert_eq!(paged_groups.objects.get(1).unwrap().order, 1);
//     assert_eq!(paged_groups.objects.get(1).unwrap().name, "125");
//
//     assert_eq!(paged_groups.objects.get(2).unwrap().order, 2);
//     assert_eq!(paged_groups.objects.get(2).unwrap().name, "150");
//
//     assert_eq!(paged_groups.objects.get(3).unwrap().order, 3);
//     assert_eq!(paged_groups.objects.get(3).unwrap().name, "200");
//
//     // Now remove a group and assert ordering recounted
//     let removed_group = groups_service
//         .remove(&user, &inserted_group_125.id)
//         .await
//         .unwrap();
//     assert_eq!(removed_group.name, "125");
//
//     let paged_groups = groups_service
//         .list(
//             &user,
//             &Paging {
//                 offset: 0,
//                 limit: 10,
//             },
//         )
//         .await
//         .unwrap();
//     // assert pagination
//     assert_eq!(paged_groups.total, 3);
//     assert_eq!(paged_groups.limit, 10);
//     assert_eq!(paged_groups.offset, 0);
//
//     // assert objects
//     assert_eq!(paged_groups.objects.get(0).unwrap().order, 0);
//     assert_eq!(paged_groups.objects.get(0).unwrap().name, "100");
//
//     assert_eq!(paged_groups.objects.get(1).unwrap().order, 1);
//     assert_eq!(paged_groups.objects.get(1).unwrap().name, "150");
//
//     assert_eq!(paged_groups.objects.get(2).unwrap().order, 2);
//     assert_eq!(paged_groups.objects.get(2).unwrap().name, "200");
//
//     // Now remove a group and assert ordering recounted
//     let removed_group = groups_service
//         .remove(&user, &inserted_group_100.id)
//         .await
//         .unwrap();
//     assert_eq!(removed_group.name, "100");
//
//     let paged_groups = groups_service
//         .list(
//             &user,
//             &Paging {
//                 offset: 0,
//                 limit: 10,
//             },
//         )
//         .await
//         .unwrap();
//     // assert pagination
//     assert_eq!(paged_groups.total, 2);
//     assert_eq!(paged_groups.limit, 10);
//     assert_eq!(paged_groups.offset, 0);
//
//     // assert objects
//     assert_eq!(paged_groups.objects.get(0).unwrap().order, 0);
//     assert_eq!(paged_groups.objects.get(0).unwrap().name, "150");
//
//     assert_eq!(paged_groups.objects.get(1).unwrap().order, 1);
//     assert_eq!(paged_groups.objects.get(1).unwrap().name, "200");
// }
