pub struct Query {
    page: usize,
    offset: usize,
}

pub struct JwtHeader {
    // JWT Token
    // Must be 'JWT ${token}' form
    authoriztaion: String,
}

// QOJ standard level
pub struct Standard {
    level: u8,
    version: String,
}

pub struct ApiHeader {
    name: String,
    url: String,
    version: String,
    support_standard: Standard,
}