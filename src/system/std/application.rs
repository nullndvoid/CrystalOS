use alloc::{boxed::Box, string::String, vec::Vec};
use async_trait::async_trait;

#[async_trait]
pub trait Application {
    fn new() -> Self;

    async fn run(&mut self, _: Vec<String>) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    UnknownCommand(String),
    CommandFailed(String),
    ApplicationError(String),
    EmptyCommand,
}

pub enum Exit {
    None,
    Exit,
    ExitWithError(Error),
}
