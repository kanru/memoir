#[derive(Debug)]
pub struct Config {
    pub posts_prefix: String,
    pub layouts_prefix: String,
    pub site_prefix: String
}

impl Config {
    pub fn new() -> Config {
        Config {
            posts_prefix: "_posts".into(),
            layouts_prefix: "_layouts".into(),
            site_prefix: "_site".into()
        }
    }
}
