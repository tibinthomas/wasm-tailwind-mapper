use std::collections::HashMap;


pub trait Framework<'a> {
    fn default_css() -> HashMap<&'a str, &'a str>;
    fn framework_name() -> &'static str;
    fn supported_version() -> Vec<&'static str>;
    fn get() -> Option<String>;
}