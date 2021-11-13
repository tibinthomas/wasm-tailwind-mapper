use std::collections::HashMap;


pub trait Framework {
    fn default_css() -> Vec<HashMap<String, String>>;
    fn framework_name() -> str;
    fn supported_version() -> Vec<&'static str>;
    fn get() -> Option<String>;
}