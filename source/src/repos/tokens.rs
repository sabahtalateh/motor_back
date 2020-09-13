use shaku::Interface;
use async_trait::async_trait;

#[async_trait]
pub trait TokensRepoIf: Interface {

}


pub struct TokensRepo {

}

#[async_trait]
impl TokensRepoIf for TokensRepo {

}
