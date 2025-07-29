use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config
}
