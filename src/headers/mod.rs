pub mod headers_config;

use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Headers {
    pub headers: HashMap<String, String>,
}
