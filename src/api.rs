use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    page: usize,
    offset: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtHeader {
    // JWT Token
    // Must be 'JWT ${token}' form
    authoriztaion: String,
}

// QOJ standard level
#[derive(Serialize, Deserialize, Debug)]
pub struct Standard {
    level: u8,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiHeader {
    name: String,
    url: String,
    version: String,
    support_standard: Standard,
}