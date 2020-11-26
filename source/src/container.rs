use shaku::module;

use crate::config::Config;
use crate::db::DB;
use crate::logger::AppLogger;
use crate::repos::blocks::BlocksRepo;
use crate::repos::default_group_sets::DefaultGroupSetsRepo;
use crate::repos::group_sets::GroupSetsRepo;
use crate::repos::groups::GroupsRepo;
use crate::repos::groups_ordering::GroupsOrderingRepo;
use crate::repos::marks::MarksRepo;
use crate::repos::sets::SetsRepo;
use crate::repos::stack::StackRepo;
use crate::repos::stack_history::StackHistoryRepo;
use crate::repos::tokens::TokensRepo;
use crate::repos::users::UsersRepo;
use crate::services::auth::AuthService;
use crate::services::groups::GroupsService;
use crate::services::stack::StackService;

module! {
    pub Container {
        components = [
            // things
            DB,
            Config,
            AppLogger,

            // repo
            BlocksRepo,
            DefaultGroupSetsRepo,
            GroupsRepo,
            GroupsOrderingRepo,
            GroupSetsRepo,
            MarksRepo,
            SetsRepo,
            StackRepo,
            StackHistoryRepo,
            TokensRepo,
            UsersRepo,

            // service
            AuthService,
            GroupsService,
            StackService,
        ],
        providers = [
        ]
    }
}
