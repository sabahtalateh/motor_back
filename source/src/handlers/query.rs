use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::Result;
use async_graphql::*;
use chrono::Utc;
use shaku::HasComponent;

use crate::config::ConfigIf;
use crate::container::Container;
use crate::handlers::groups::UserGroup;
use crate::handlers::stack::StackItem;
use crate::handlers::Paging;
use crate::repos::Id;
use crate::services::auth::AuthServiceIf;
use crate::services::groups::{GroupsServiceIf, Set};
use crate::services::stack::StackServiceIf;
use crate::services::PageInfo;
use crate::utils::ExtendType;

pub struct Query;

#[Object]
impl Query {
    pub async fn api_version<'a>(&'a self, ctx: &'a Context<'_>) -> &'a str {
        let config: &dyn ConfigIf = ctx.data_unchecked::<Container>().resolve_ref();
        config.api_version()
    }

    pub async fn recent_sets(&self, ctx: &Context<'_>, access: String) -> Result<Vec<Set>> {
        let ctr: &Container = ctx.data_unchecked::<Container>();
        let auth: &dyn AuthServiceIf = ctr.resolve_ref();
        let user = auth.validate_access(&access, Utc::now()).await?;

        let groups: &dyn GroupsServiceIf = ctr.resolve_ref();

        Ok(vec![Set {
            id: Id::from_str("123"),
            name: "123".to_string(),
        }])
    }

    pub async fn list_groups(
        &self,
        ctx: &Context<'_>,
        access: String,
        group_set: Option<String>,
        paging: Option<Paging>,
    ) -> Result<Connection<usize, UserGroup, PageInfo, EmptyFields>> {
        let ctr: &Container = ctx.data_unchecked::<Container>();
        let groups: &dyn GroupsServiceIf = ctr.resolve_ref();
        let auth: &dyn AuthServiceIf = ctr.resolve_ref();
        let user = auth.validate_access(&access, Utc::now()).await?;

        query(None, None, None, None, |_, _, _, _| async move {
            let sl = groups
                .list_groups(user, group_set.into(), paging)
                .await
                .extend_type()?;

            let mut connection = Connection::with_additional_fields(false, false, sl.page_info);
            connection.append(
                sl.objects
                    .into_iter()
                    .map(|item| Edge::new(item.order as usize, item)),
            );
            Ok(connection)
        })
        .await
    }

    // pub async fn my_stack(&self, ctx: &Context<'_>, access: String) -> Result<Vec<StackItem>> {
    //     let ctr: &Container = ctx.data_unchecked::<Container>();
    //     let auth: &dyn AuthServiceIf = ctr.resolve_ref();
    //     let user = auth
    //         .validate_access(&access, &Utc::now())
    //         .await
    //         .extend_type()?;
    //
    //     let stack_service: &dyn StackServiceIf = ctx.data_unchecked::<Container>().resolve_ref();
    //     Ok(stack_service
    //         .my_stack(user)
    //         .await
    //         .into_iter()
    //         .map(|i| i.into())
    //         .collect())
    // }
    //
    // pub async fn my_groups(
    //     &self,
    //     ctx: &Context<'_>,
    //     access: String,
    //     paging: Paging,
    // ) -> GraphQLResult<PagedResponse<UserGroup>> {
    //     let auth: &dyn AuthServiceIf = ctx.data_unchecked::<Container>().resolve_ref();
    //     let user = auth.validate_access(&access, Utc::now()).await?;
    //
    //     unimplemented!()
    //
    //     // let groups: &dyn GroupsServiceIf = ctx.ctr.resolve_ref();
    //     // groups.list(&user, &paging).await
    // }

    // async fn numbers(
    //     &self,
    //     _ctx: &Context<'_>,
    //     after: Option<String>,
    //     before: Option<String>,
    //     first: Option<i32>,
    //     last: Option<i32>,
    // ) -> Result<Connection<usize, KK, PP, Diff>> {
    //     query(
    //         after,
    //         before,
    //         first,
    //         last,
    //         |after, before, first, last| async move {
    //             let mut start = after.map(|after| after + 1).unwrap_or(0);
    //             let mut end = before.unwrap_or(10000);
    //             if let Some(first) = first {
    //                 end = (start + first).min(end);
    //             }
    //             if let Some(last) = last {
    //                 start = if last > end - start { end } else { end - last };
    //             }
    //             let mut connection =
    //                 Connection::with_additional_fields(start > 0, end < 10000, PP { pp: 22 });
    //             connection.append((start..end).into_iter().map(|n| {
    //                 Edge::with_additional_fields(
    //                     n,
    //                     KK { kk: n as i32 },
    //                     Diff {
    //                         diff: (10000 - n) as i32,
    //                     },
    //                 )
    //             }));
    //             Ok(connection)
    //         },
    //     )
    //     .await
    // }

    // pub async fn my_groups33(
    //     &self,
    //     ctx: &Context<'_>,
    //     access: String,
    //     paging: Paging,
    // ) -> GraphQLResult<PagedResponse<i32>> {
    //     let auth: &dyn AuthServiceIf = ctx.data_unchecked::<Container>().resolve_ref();
    //     let user = auth.validate_access(&access, Utc::now()).await?;
    //
    //     unimplemented!()
    //
    //     // let groups: &dyn GroupsServiceIf = ctx.ctr.resolve_ref();
    //     // groups.list(&user, &paging).await
    // }
}
