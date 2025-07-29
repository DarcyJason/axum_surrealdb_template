#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub server_port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            server_port: std::env::var("SERVER_PORT")
                .expect("SERVER_PORT must be set").
                parse::<u16>()
                .expect("SERVER_PORT should be a u16 number"),
        }
    }
}

impl ServerConfig {
    pub fn new() -> Self {
        Self::default()
    }
}