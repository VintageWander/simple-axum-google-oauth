use dotenvy::var;
use oauth2::{ClientId, ClientSecret, RedirectUrl};

pub fn port() -> u16 {
    var("PORT")
        .expect("PORT is not set")
        .parse()
        .expect("PORT is not a number")
}

pub fn google_client_id() -> ClientId {
    let client_id = var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID is not set");
    ClientId::new(client_id)
}

pub fn google_client_secret() -> ClientSecret {
    let client_secret = var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET is not set");
    ClientSecret::new(client_secret)
}

pub fn google_redirect_url() -> RedirectUrl {
    let redirect_url = var("GOOGLE_REDIRECT_URL").expect("GOOGLE_REDIRECT_URL is not set");
    RedirectUrl::new(redirect_url).expect("GOOGLE_REDIRECT_URL is not valid")
}

pub fn access_token_secret() -> String {
    var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET is not set")
}

pub fn refresh_token_secret() -> String {
    var("REFRESH_TOKEN_SECRET").expect("REFRESH_TOKEN_SECRET is not set")
}

pub fn check_env() {
    port();
    google_client_id();
    google_client_secret();
    google_redirect_url();
    access_token_secret();
    refresh_token_secret();
}
