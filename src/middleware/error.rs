// #[macro_use]
// extern crate thiserror;
use async_graphql::Error;
use async_graphql::ErrorExtensions;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserCustomResponseError {
    #[error("Could not find resource")]
    NotFound,

    #[error("Server Error")]
    ServerError,

    #[error("Not Allowed")]
    NotAllowed,
}

impl ErrorExtensions for UserCustomResponseError {
    fn extend(&self) -> Error {
        Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            UserCustomResponseError::NotFound => e.set("code", "NOT_FOUND"),
            UserCustomResponseError::ServerError => e.set("code", "Internal_Server_Error"),
            UserCustomResponseError::NotAllowed => e.set("code", "Not_Allowed"),
        })
    }
}
