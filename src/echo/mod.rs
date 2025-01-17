pub mod echo;
pub mod echo_internal;

use crate::Config;

pub struct Echo {
    pub config: Config,
    client: reqwest::Client,
}

