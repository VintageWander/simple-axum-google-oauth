use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, TokenUrl,
};

use crate::config::{google_client_id, google_client_secret};

pub fn get_client(hostname: String) -> BasicClient {
    let google_client_id = ClientId::new(google_client_id());
    let google_client_secret = ClientSecret::new(google_client_secret());

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Unable to create auth url");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Unable to create token url");

    let protocol = if hostname.starts_with("localhost")
        || hostname.starts_with("127.0.0.1")
        || hostname.starts_with("0.0.0.0")
    {
        "http"
    } else {
        "https"
    };

    let redirect_url = RedirectUrl::new(format!("{protocol}://{hostname}/callback"))
        .expect("Unable to create redirect url");

    let revocation_url = RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
        .expect("Unable to create revocation url");

    BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_url)
    .set_revocation_uri(revocation_url)
}
