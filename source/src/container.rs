use shaku::module;

use crate::config::Config;
use crate::db::DB;
use crate::logger::AppLogger;

use crate::repos::stack::StackRepo;
use crate::repos::tokens::TokensRepo;
use crate::repos::users::UsersRepo;

use crate::services::auth::AuthService;
use crate::services::check::CheckService;
use crate::services::stack::StackService;

module! {
    pub Container {
        components = [
            // basic
            DB,
            Config,
            AppLogger,

            // repo
            StackRepo,
            TokensRepo,
            UsersRepo,

            // service
            AuthService,
            CheckService,
            StackService,
        ],
        providers = [
        ]
    }
}
