use uuid::Uuid;
use mime::Mime;
use iso639_1::Iso639_1;
use chrono::prelude::*;

pub struct Explain {
    language: Iso639_1,
    mime_type: Option<Mime>,
    text: String,
}

pub struct Sample {
    input: String,
    output: String,
}

pub struct Problem {
    id: Uuid,
    title: String,
    explains: Vec<Explain>,
    samples: Vec<Sample>,
    tags: Vec<Uuid>, // Tag UUID
    languages: Vec<Uuid>, // Language UUID
    created_at: Option<DateTime<Utc>>,
}