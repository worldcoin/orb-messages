#![allow(clippy::all, clippy::pedantic)]

use thiserror::Error;

pub mod mcu_main;
pub mod mcu_sec;

impl core::fmt::Display for mcu_main::ack::ErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Success => "operation succeeded",
            Self::Version => "unsupported version",
            Self::Range => "argument is out of range",
            Self::InProgress => "operation is already in progress",
            Self::Fail => "operation failed",
            Self::OverTemperature => "operation could not be completed because of overheating",
            Self::OperationNotSupported => "operation is not supported",
            Self::InvalidState => "message can not be processed because of the current state",
        })
    }
}

impl core::fmt::Display for mcu_sec::ack::ErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Success => "operation succeeded",
            Self::Version => "unsupported version",
            Self::Range => "argument is out of range",
            Self::InProgress => "operation is already in progress",
            Self::Fail => "operation failed",
            Self::OverTemperature => "operation could not be completed because of overheating",
            Self::OperationNotSupported => "operation is not supported",
            Self::InvalidState => "message can not be processed because of the current state",
        })
    }
}

/// Common acknowledgement error codes for _any_ microcontroller.
#[derive(Error, Debug)]
pub enum CommonAckError {
    #[error("operation succeeded")]
    Success,
    #[error("unsupported version")]
    Version,
    #[error("argument is out of range")]
    Range,
    #[error("operation is already in progress")]
    InProgress,
    #[error("operation failed")]
    Fail,
    #[error("operation could not be completed because of overheating")]
    OverTemperature,
    #[error("operation is not supported")]
    OperationNotSupported,
    #[error("message can not be processed because of the current state")]
    InvalidState,
}

impl From<i32> for CommonAckError {
    fn from(value: i32) -> Self {
        match value {
            0 => CommonAckError::Success,
            1 => CommonAckError::Version,
            2 => CommonAckError::Range,
            3 => CommonAckError::InProgress,
            4 => CommonAckError::Fail,
            5 => CommonAckError::OverTemperature,
            6 => CommonAckError::OperationNotSupported,
            7 => CommonAckError::InvalidState,
            _ => {
                panic!("Unknown error code: {}", value)
            }
        }
    }
}

/// Converts a `mcu_main::ack::ErrorCode` to a `CommonAckError`.
impl From<mcu_main::ack::ErrorCode> for CommonAckError {
    fn from(value: mcu_main::ack::ErrorCode) -> Self {
        use mcu_main::ack::ErrorCode as E;
        match value {
            E::Success => Self::Success,
            E::Version => Self::Version,
            E::Range => Self::Range,
            E::InProgress => Self::InProgress,
            E::Fail => Self::Fail,
            E::OverTemperature => Self::OverTemperature,
            E::OperationNotSupported => Self::OperationNotSupported,
            E::InvalidState => Self::InvalidState,
        }
    }
}

/// Converts a `mcu_sec::ack::ErrorCode` to a `CommonAckError`.
impl From<mcu_sec::ack::ErrorCode> for CommonAckError {
    fn from(value: mcu_sec::ack::ErrorCode) -> Self {
        use mcu_sec::ack::ErrorCode as E;
        match value {
            E::Success => Self::Success,
            E::Version => Self::Version,
            E::Range => Self::Range,
            E::InProgress => Self::InProgress,
            E::Fail => Self::Fail,
            E::OverTemperature => Self::OverTemperature,
            E::OperationNotSupported => Self::OperationNotSupported,
            E::InvalidState => Self::InvalidState,
        }
    }
}
