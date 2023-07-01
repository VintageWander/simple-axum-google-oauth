use dotenvy::var;

pub fn port() -> u16 {
    var("PORT")
        .expect("PORT is not set")
        .parse()
        .expect("PORT is not a number")
}

pub fn google_client_id() -> String {
    var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID is not set")
}

pub fn google_client_secret() -> String {
    var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET is not set")
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
    access_token_secret();
    refresh_token_secret();
}
