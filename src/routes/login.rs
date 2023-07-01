use axum::{
    extract::{Host, Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use oauth2::{CsrfToken, PkceCodeChallenge, Scope};
use serde::Deserialize;

use crate::{utils::google::get_client, GlobalState, WebResult};

pub fn login() -> Router<GlobalState> {
    #[derive(Deserialize)]
    struct LoginQuery {
        pub return_url: Option<String>,
    }
    async fn login_handler(
        State(GlobalState { db }): State<GlobalState>,
        Host(hostname): Host,
        Query(LoginQuery { return_url }): Query<LoginQuery>,
    ) -> WebResult {
        let return_url = return_url.unwrap_or_else(|| "/".into());

        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let oauth_client = get_client(hostname);

        let (authorized_url, csrf_state) = oauth_client
            .authorize_url(CsrfToken::new_random)
            .add_scopes([
                Scope::new("https://www.googleapis.com/auth/userinfo.email".into()),
                Scope::new("https://www.googleapis.com/auth/userinfo.profile".into()),
            ])
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        db.oauth_state_storage()
            .create(
                csrf_state.secret().clone(),
                pkce_code_verifier.secret().clone(),
                return_url,
                vec![],
            )
            .exec()
            .await?;

        Ok(Redirect::to(authorized_url.as_str()).into_response())
    }
    Router::new().route("/login", get(login_handler))
}
