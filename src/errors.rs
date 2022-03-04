pub type Result<T, E = Box<dyn std::error::Error + Send + Sync + 'static>> =
    core::result::Result<T, E>;
