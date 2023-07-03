use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::CookieJar;
use oauth2::{reqwest::http_client, AuthorizationCode, CsrfToken, PkceCodeVerifier, TokenResponse};
use serde::Deserialize;
use tokio::task::spawn_blocking;

use crate::{
    model::{user_select, GoogleUserProfile},
    prisma::{oauth_state_storage, user},
    utils::{
        cookie::{make_access_cookie, make_refresh_cookie},
        encode::{encode_access_token, encode_refresh_token},
        google::get_client,
        rand_pw::random_password,
    },
    GlobalState, WebResult,
};

pub fn oauth_callback() -> Router<GlobalState> {
    #[derive(Deserialize)]
    struct OAuthCallbackQuery {
        pub state: String,
        pub code: String,
    }
    async fn oauth_callback_handler(
        State(GlobalState { db }): State<GlobalState>,
        cookies: CookieJar,
        Query(OAuthCallbackQuery { state, code }): Query<OAuthCallbackQuery>,
        Query(query): Query<HashMap<String, String>>,
    ) -> WebResult {
        dbg!(query);
        let state = CsrfToken::new(state);
        let code = AuthorizationCode::new(code);

        let deleted_oauth_state = db
            .oauth_state_storage()
            .delete(oauth_state_storage::csrf_state::equals(
                state.secret().to_owned(),
            ))
            .exec()
            .await?;

        let pkce_code_verifier = PkceCodeVerifier::new(deleted_oauth_state.pkce_code_verifier);
        let return_url = deleted_oauth_state.return_url;

        let client = get_client();

        let token_response = spawn_blocking(move || {
            client
                .exchange_code(code)
                .set_pkce_verifier(pkce_code_verifier)
                .request(http_client)
        })
        .await??;

        let access_token = token_response.access_token().secret();

        // Get user info from google
        let url =
            format!("https://www.googleapis.com/oauth2/v2/userinfo?access_token={access_token}");

        let body: GoogleUserProfile = reqwest::get(url).await?.json().await?;

        dbg!(&body);

        let GoogleUserProfile { email, .. } = body;

        let find_user = db
            .user()
            .find_unique(user::email::equals(email.clone()))
            .select(user_select::select())
            .exec()
            .await?;

        // let mut is_created = false;

        let user = match find_user {
            Some(user) => user,
            None => {
                let trimmed_email = email.split_once('@').unwrap().0.to_string();

                // is_created = true;

                db.user()
                    .create(trimmed_email, email, random_password(), vec![])
                    .select(user_select::select())
                    .exec()
                    .await?
            }
        };

        let access_token = encode_access_token(&user)?;
        let refresh_token = encode_refresh_token(&user)?;

        let access_cookie = make_access_cookie(access_token);
        let refresh_cookie = make_refresh_cookie(refresh_token.clone());

        db.user()
            .update(
                user::id::equals(user.id),
                vec![user::refresh_token::set(refresh_token)],
            )
            .select(user_select::select())
            .exec()
            .await?;

        let response = (
            cookies.add(access_cookie).add(refresh_cookie),
            Redirect::to(&return_url),
        );

        Ok(response.into_response())
    }
    Router::new().route("/callback", get(oauth_callback_handler))
}
