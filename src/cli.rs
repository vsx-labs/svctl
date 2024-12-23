#[derive(Debug)]
pub struct DefaultArgs {
    pub cluster: String,
    pub verbose: bool,
}

impl DefaultArgs {
    pub fn new() -> Self {
        DefaultArgs {
            verbose: false,
            cluster: "testnet".to_string(),
        }
    }
}

impl Default for DefaultArgs {
    fn default() -> Self {
        Self::new()
    }
}
