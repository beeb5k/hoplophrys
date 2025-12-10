use crate::config::Config;

mod config;

/// The main application struct for Hoplophrys.
///
/// This struct holds the application configuration and serves as the
/// primary entry point for interacting with the library.
pub struct Hoplophrys {
    /// The application configuration.
    config: Config,
}

impl Default for Hoplophrys {
    fn default() -> Self {
        Self::new()
    }
}

impl Hoplophrys {
    /// Creates a new `Hoplophrys` instance with default configuration.
    ///
    /// # Returns
    ///
    /// A new `Hoplophrys` instance initialized with default settings.
    pub fn new() -> Self {
        Self { config: Config::new() }
    }

    /// Returns a reference to the application configuration.
    ///
    /// # Returns
    ///
    /// A reference to the [`Config`] struct.
    pub fn config(&self) -> &Config {
        &self.config
    }
}
