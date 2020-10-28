use crate::{setup_with_default_user, trunc_collection};
use bson::oid::ObjectId;
use motor_back::container::Container;
use motor_back::db::DBIf;
use motor_back::errors::AppError;
use motor_back::handlers::stack::{
    NewBlock, NewMark, NewStackItem, StackItemChangeSet, UpdateBlock,
};
use motor_back::logger::AppLoggerIf;
use motor_back::repos::db as ddbb;
use motor_back::repos::marks::InsertMark;
use motor_back::repos::users::User;
use motor_back::repos::Id;
use motor_back::services::stack::{StackItem, StackService, StackServiceIf};
use shaku::HasComponent;
