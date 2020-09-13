use crate::config::Config;
use crate::db::DB;
use crate::logger::AppLogger;
use crate::repos::users::UsersRepo;
use crate::repos::tokens::TokensRepo;
use crate::services::auth::AuthService;
use crate::services::check::CheckService;
use shaku::module;

module! {
    pub Container {
        components = [
            // base
            DB,
            Config,
            AppLogger,

            // repo
            UsersRepo,
            TokensRepo,

            // service
            AuthService,
            CheckService,
        ],
        providers = [
        ]
    }
}
