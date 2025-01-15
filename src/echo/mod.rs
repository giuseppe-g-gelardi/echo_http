pub mod echo;

use crate::Config;

pub struct Echo {
    pub config: Config,
    client: reqwest::Client,
}

