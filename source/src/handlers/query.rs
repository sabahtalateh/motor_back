use actix_web::dev::Service;
use async_graphql::*;
use async_graphql::connection::*;
use async_graphql::Result;
use chrono::Utc;
use shaku::HasComponent;

use crate::config::ConfigIf;
use crate::container::Container;
use crate::handlers::stack::StackItem;
use crate::services::auth::AuthServiceIf;
use crate::services::stack::StackServiceIf;
use crate::utils::ExtendType;

#[derive(SimpleObject)]
struct Diff {
    diff: i32,
}

#[derive(SimpleObject)]
struct KK {
    kk: i32,
}

#[derive(SimpleObject)]
struct PP {
    pp: i32,
}

pub struct Query;

#[Object]
impl Query {
    pub async fn api_version(&self, ctx: &Context<'_>) -> String {
        let config: &dyn ConfigIf = ctx.data_unchecked::<Container>().resolve_ref();
        config.api_version()
    }

    pub async fn my_stack(&self, ctx: &Context<'_>, access: String) -> Result<Vec<StackItem>> {
        let ctr: &Container = ctx.data_unchecked::<Container>();
        let auth: &dyn AuthServiceIf = ctr.resolve_ref();
        let user = auth
            .validate_access(&access, Utc::now())
            .await
            .extend_type()?;

        let stack_service: &dyn StackServiceIf = ctx.data_unchecked::<Container>().resolve_ref();
        Ok(stack_service
            .my_stack(user)
            .await
            .into_iter()
            .map(|i| i.into())
            .collect())
    }
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

    async fn numbers(
        &self,
        _ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, KK, PP, Diff>> {
        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = after.map(|after| after + 1).unwrap_or(0);
                let mut end = before.unwrap_or(10000);
                if let Some(first) = first {
                    end = (start + first).min(end);
                }
                if let Some(last) = last {
                    start = if last > end - start { end } else { end - last };
                }
                let mut connection =
                    Connection::with_additional_fields(start > 0, end < 10000, PP { pp: 22 });
                connection.append((start..end).into_iter().map(|n| {
                    Edge::with_additional_fields(
                        n,
                        KK { kk: n as i32 },
                        Diff {
                            diff: (10000 - n) as i32,
                        },
                    )
                }));
                Ok(connection)
            },
        )
        .await
    }

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
