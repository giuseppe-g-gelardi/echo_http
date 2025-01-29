pub mod headers;

use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Headers<'a> {
    pub headers: HashMap<&'a str, &'a str>,
}
