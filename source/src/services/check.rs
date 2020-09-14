use crate::errors::AppError;
use crate::repos::users::UsersRepoIf;
use crate::utils::AppResult;
use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;

#[async_trait]
pub trait CheckServiceIf: Interface {
    fn strong_password(&self, password: &str) -> AppResult<()>;
    async fn username_exists(&self, username: &str) -> bool;
}

#[derive(Component)]
#[shaku(interface = CheckServiceIf)]
pub struct CheckService {
    #[shaku(inject)]
    user_repo: Arc<dyn UsersRepoIf>,
    pwd_min_len: u32,
}

#[async_trait]
impl CheckServiceIf for CheckService {
    fn strong_password(&self, password: &str) -> AppResult<()> {
        if password.len() >= self.pwd_min_len as usize {
            Ok(())
        } else {
            Err(AppError::check(
                format!(
                    "password length should be at least `{}` characters",
                    self.pwd_min_len
                )
                .as_str(),
            ))
        }
    }

    async fn username_exists(&self, username: &str) -> bool {
        match self.user_repo.find_by_username(username).await {
            Some(_) => true,
            None => false,
        }
    }
}
