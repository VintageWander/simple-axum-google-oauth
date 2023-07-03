use oauth2::{basic::BasicClient, AuthUrl, RevocationUrl, TokenUrl};

use crate::config::{google_client_id, google_client_secret, google_redirect_url};

pub fn get_client() -> BasicClient {
    let google_client_id = google_client_id();
    let google_client_secret = google_client_secret();

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())
        .expect("Unable to create auth url");
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
        .expect("Unable to create token url");

    let redirect_url = google_redirect_url();

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
