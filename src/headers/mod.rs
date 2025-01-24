pub mod headers;

use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Headers {
    headers: HashMap<String, String>,
}

