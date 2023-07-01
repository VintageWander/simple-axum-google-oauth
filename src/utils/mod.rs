pub mod cookie;
pub mod decode;
pub mod encode;
pub mod google;
pub mod rand_pw;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    name: String,
    exp: usize,
}

pub enum TokenType {
    Access,
    Refresh,
}
