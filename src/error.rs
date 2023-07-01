use thiserror::Error;

use axum::response::{IntoResponse, Response};
use prisma_client_rust::{
    prisma_errors::query_engine::{
        ConstraintViolation, RecordNotFound, TableDoesNotExist, UniqueKeyViolation,
    },
    QueryError,
};

use crate::response::Web;

pub fn match_query_error(error: QueryError) -> Response {
    if error.is_prisma_error::<UniqueKeyViolation>() {
        Web::conflict(
            "Conflict data",
            "The provided data is already exists, please try another",
        )
    } else if error.is_prisma_error::<ConstraintViolation>() {
        Web::bad_request(
            "Constraint violated",
            "A constraint in the database has been violated",
        )
    } else if error.is_prisma_error::<RecordNotFound>() {
        Web::not_found(
            "Not found",
            "The information provided could not be found in the database",
        )
    } else if error.is_prisma_error::<TableDoesNotExist>() {
        Web::internal_error(
            "Table does not exists",
            "The database has not yet been initialized",
        )
    } else {
        Web::internal_error("Unknown error", error)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Query error")]
    Query(#[from] QueryError),

    #[error("Joining thread error")]
    Join(#[from] tokio::task::JoinError),

    #[error("Send request failed")]
    SendRequest(#[from] reqwest::Error),

    #[error("Exchange code failed")]
    ExchangeCode(
        #[from]
        oauth2::RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        >,
    ),

    #[error("JWT error")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Not found")]
    NotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Query(error) => match_query_error(error),
            Error::Join(e) => Web::internal_error("Server system having problems", e),
            Error::SendRequest(e) => Web::internal_error("Send request to OAuth2 server error", e),
            Error::ExchangeCode(e) => Web::internal_error("Exchange code failed", e),
            Error::Jwt(e) => Web::bad_request("Invalid token", e),
            Error::Unauthorized => {
                Web::unauthorized("Unauthorized", "You need to login, or create a new account")
            }
            Error::NotFound => Web::not_found(
                "Not found",
                "The information that you provided could not be found in the database",
            ),
        }
    }
}
