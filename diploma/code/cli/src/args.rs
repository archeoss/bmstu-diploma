use error_stack::{Result, ResultExt};
use serde::{de::DeserializeOwned, Deserialize};
use serde_with::{serde_as, DisplayFromStr};
use std::{fs::File, io::BufReader, net::SocketAddr, path::PathBuf, time::Duration};
use thiserror::Error;

/// Server Configuration passed on initialization
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// Server address <host:port>
    pub address: SocketAddr,

    /// Enable Default Cors configuration
    #[serde(default = "Config::default_cors")]
    pub cors_allow_all: bool,

    /// [`Logger`](LoggerConfig) Configuration
    #[serde(default)]
    pub logger: LoggerConfig,
}

/// Logger Configuration passed on initialization
#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LoggerConfig {
    /// Rolling file logger config
    #[serde(default)]
    pub file: Option<FileLogger>,

    /// Stdout logger config
    #[serde(default)]
    pub stdout: Option<StdoutLogger>,

    /// Tracing Level
    #[serde(default = "LoggerConfig::level_default")]
    #[serde_as(as = "DisplayFromStr")]
    pub trace_level: tracing::Level,
}

/// File Logger Configuration for writing logs to files
#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FileLogger {
    /// Enable log output to file
    pub enabled: bool,

    /// File to save logs
    pub log_file: Option<PathBuf>,

    /// Number of log files
    #[serde(default = "FileLogger::default_log_amount")]
    pub log_amount: usize,

    /// Max size of a single log file, in bytes
    #[serde(default = "FileLogger::default_log_size")]
    pub log_size: usize,
}

/// Stdout Logger Configuration for printing logs into stdout
#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StdoutLogger {
    /// Enable log output to stdout
    pub enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: SocketAddr::from(([0, 0, 0, 0], 7000)),
            cors_allow_all: Self::default_cors(),
            logger: LoggerConfig::default(),
        }
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            file: None,
            stdout: None,
            trace_level: Self::level_default(),
        }
    }
}

impl Config {
    #[must_use]
    pub const fn default_cors() -> bool {
        false
    }

    #[must_use]
    pub const fn default_timeout() -> Duration {
        Duration::from_millis(5000)
    }
}

impl LoggerConfig {
    #[must_use]
    pub const fn level_default() -> tracing::Level {
        tracing::Level::TRACE
    }
}

impl FileLogger {
    #[must_use]
    pub const fn default_enabled() -> bool {
        false
    }

    #[must_use]
    pub const fn default_log_amount() -> usize {
        5
    }

    #[must_use]
    pub const fn default_log_size() -> usize {
        10usize.pow(6)
    }
}

impl StdoutLogger {
    #[must_use]
    pub const fn default_enabled() -> bool {
        false
    }
}

impl Default for FileLogger {
    fn default() -> Self {
        Self {
            log_file: None,
            log_amount: Self::default_log_amount(),
            log_size: Self::default_log_size(),
            enabled: Self::default_enabled(),
        }
    }
}

impl Default for StdoutLogger {
    fn default() -> Self {
        Self {
            enabled: Self::default_enabled(),
        }
    }
}

pub trait FromFile {
    /// Parses the file spcified in `path`
    ///
    /// # Errors
    ///
    /// The fucntion will fail if either it couldn't open config file
    /// or failed to parse given file
    fn from_file(path: PathBuf) -> Result<Self, Error>
    where
        Self: Sized + DeserializeOwned,
    {
        let file = File::open(path).change_context(Error::FromFile)?;
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).change_context(Error::FromFile)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("configuration error: couldn't read from file")]
    FromFile,
}

impl FromFile for Config {}
