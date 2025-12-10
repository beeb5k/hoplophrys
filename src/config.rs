/// Configuration settings for the application.
///
/// Stores runtime configuration values such as directory paths
/// and other application-wide settings.
pub struct Config {
    /// The directory path where application data is stored.
    data_dir: String,
}

impl Config {
    /// Creates a new `Config` instance with default values.
    ///
    /// The default data directory is set to `"hoplo_data"`.
    pub(super) fn new() -> Self {
        Self { data_dir: String::from("hoplo_data") }
    }

    /// Returns the data directory path.
    pub fn data_dir(&self) -> &str {
        &self.data_dir
    }
}
