use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{error::Error, prisma::user, utils::decode::decode_access_token, GlobalState};

#[derive(Serialize, Deserialize, Debug)]
pub struct GoogleUserProfile {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub locale: String,
}

user::select!(user_select {
    id username email created_at updated_at
});

pub type UserSelect = user_select::Data;

pub struct LoggedInUser(pub UserSelect);

#[async_trait]
impl FromRequestParts<GlobalState> for LoggedInUser {
    type Rejection = Error;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {
        let cookies = CookieJar::from_request_parts(parts, state).await.expect("This should not be happening, due to the fact that cookies are always sent along request");

        let access_token = cookies
            .get("accessToken")
            .ok_or_else(|| Error::Unauthorized)?
            .value()
            .to_string();

        let user_id = decode_access_token(access_token)?;
        let user = state
            .db
            .user()
            .find_unique(user::id::equals(user_id))
            .select(user_select::select())
            .exec()
            .await?
            .ok_or_else(|| Error::NotFound)?;

        Ok(LoggedInUser(user))
    }
}
