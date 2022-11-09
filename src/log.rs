use clap::{Parser, ValueEnum};
use serde::Deserialize;
use strum_macros::{AsRefStr, EnumIter, EnumString};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Deserialize, AsRefStr)]
pub enum LogLevel {
    /// A level lower than all log levels.
    OFF,
    /// Corresponds to the `Error` log level.
    ERROR,
    /// Corresponds to the `Warn` log level.
    WARN,
    /// Corresponds to the `Info` log level.
    INFO,
    /// Corresponds to the `Debug` log level.
    DEBUG,
    /// Corresponds to the `Trace` log level.
    TRACE,
}
