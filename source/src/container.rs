use crate::config::{Config, ConfigIf};
use crate::db::{DBParameters, DB};
use crate::logger::build_app_logger;
use crate::logger::{AppLogger, AppLoggerParameters};
use crate::mongo;
use crate::repos::users::UserRepo;
use crate::services::auth::AuthService;
use crate::services::check::{CheckService, CheckServiceParameters};
use mongodb::{Client, Database};
use shaku::{module, Component, HasProvider, Interface, Module, Provider};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

module! {
    pub Container {
        components = [
            // base
            DB,
            Config,
            AppLogger,

            // repo
            UserRepo,

            // service
            AuthService,
            CheckService,
        ],
        providers = [
        ]
    }
}


