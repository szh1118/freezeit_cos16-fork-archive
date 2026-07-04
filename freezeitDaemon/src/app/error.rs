use std::{fmt, io};

#[derive(Debug)]
pub enum DaemonError {
    Config(String),
    Io(io::Error),
    Protocol(String),
    System(String),
}

impl DaemonError {
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config(message.into())
    }

    pub fn protocol(message: impl Into<String>) -> Self {
        Self::Protocol(message.into())
    }

    pub fn system(message: impl Into<String>) -> Self {
        Self::System(message.into())
    }
}

impl fmt::Display for DaemonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(message) => write!(formatter, "config error: {message}"),
            Self::Io(error) => write!(formatter, "io error: {error}"),
            Self::Protocol(message) => write!(formatter, "protocol error: {message}"),
            Self::System(message) => write!(formatter, "system error: {message}"),
        }
    }
}

impl std::error::Error for DaemonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for DaemonError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
