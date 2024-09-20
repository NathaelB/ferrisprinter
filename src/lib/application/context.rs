pub struct AppContext {}

impl AppContext {
    pub fn new() -> Self {
        AppContext {}
    }
}

impl Default for AppContext {
    fn default() -> Self {
        AppContext::new()
    }
}
