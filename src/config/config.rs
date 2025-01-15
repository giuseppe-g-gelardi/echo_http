use super::Config;

impl Default for Config {
    fn default() -> Self {
        Config {
            base_url: None,
            timeout: None,
            headers: None,
        }
    }
}
