pub struct Config {
    pub database_url: String,
    pub server_addr: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: "routes.db".to_string(),
            server_addr: "127.0.0.1:8080".to_string(),
        }
    }
}