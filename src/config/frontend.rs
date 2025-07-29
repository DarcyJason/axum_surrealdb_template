#[derive(Debug, Clone)]
pub struct FrontendConfig {
    pub frontend_url: String,
}

impl Default for FrontendConfig {
    fn default() -> Self {
        FrontendConfig {
            frontend_url: std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set")
        }
    }
}

impl FrontendConfig {
    pub fn new() -> Self {
        Self::default()
    }
}