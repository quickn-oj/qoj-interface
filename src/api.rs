use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    page: usize,
    offset: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtHeader<'a> {
    // JWT Token
    // Must be 'JWT ${token}' form
    authoriztaion: &'a str,
}

// QOJ standard level
#[derive(Serialize, Deserialize, Debug)]
pub struct Standard<'a> {
    level: u8,
    version: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiHeader<'a> {
    name: &'a str,
    url: &'a str,
    version: &'a str,
    support_standard: Standard<'a>,
}