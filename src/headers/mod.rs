pub mod headers_config;

use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Headers<'a> {
    pub headers: HashMap<&'a str, &'a str>,
}



