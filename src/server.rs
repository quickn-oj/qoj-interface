use crate::extension::Extension;

pub struct Server {
    app_name: String,
    description: Option<String>,
    extensions: Vec<Extension>,
}