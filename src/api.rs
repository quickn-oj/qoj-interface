use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    pub page: usize,
    pub offset: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtHeader<'a> {
    // JWT Token
    // Must be 'JWT ${token}' form
    pub authoriztaion: &'a str,
}

// QOJ standard level
#[derive(Serialize, Deserialize, Debug)]
pub struct Standard<'a> {
    pub level: u8,
    pub version: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyHeader;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiHeader<'a> {
    pub name: &'a str,
    pub url: &'a str,
    pub version: &'a str,
    pub support_standard: Standard<'a>,
}