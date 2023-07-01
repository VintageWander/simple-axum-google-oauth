use axum::{extract::State, routing::get, Router};

use crate::{model::LoggedInUser, response::Web, GlobalState, WebResult};

pub fn profile() -> Router<GlobalState> {
    async fn profile_handler(
        State(GlobalState { .. }): State<GlobalState>,
        LoggedInUser(user): LoggedInUser,
    ) -> WebResult {
        Ok(Web::ok("Here's your user profile", user))
    }
    Router::new().route("/profile", get(profile_handler))
}
