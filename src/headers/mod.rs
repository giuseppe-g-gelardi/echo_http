pub mod headers;

use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Headers<'a> {
    headers: HashMap<&'a str, &'a str>,
}
