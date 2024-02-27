#![allow(clippy::all, clippy::pedantic)]

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

/// Common acknowledgement error codes for any microcontroller.
pub enum CommonAckError {
    Success,
    Version,
    Range,
    InProgress,
    Fail,
    OverTemperature,
    OperationNotSupported,
    InvalidState,
}

/// Converts a `mcu_main::ack::ErrorCode` to a `CommonAckError`.
impl From<mcu_main::ack::ErrorCode> for CommonAckError {
    fn from(value: mcu_main::ack::ErrorCode) -> Self {
        match value {
            mcu_main::ack::ErrorCode::Success => {CommonAckError::Success}
            mcu_main::ack::ErrorCode::Version => {CommonAckError::Version}
            mcu_main::ack::ErrorCode::Range => {CommonAckError::Range}
            mcu_main::ack::ErrorCode::InProgress => {CommonAckError::InProgress}
            mcu_main::ack::ErrorCode::Fail => {CommonAckError::Fail}
            mcu_main::ack::ErrorCode::OverTemperature => {CommonAckError::OverTemperature}
            mcu_main::ack::ErrorCode::OperationNotSupported => {CommonAckError::OperationNotSupported}
            mcu_main::ack::ErrorCode::InvalidState => {CommonAckError::InvalidState}
        }
    }
}

impl From<i32> for CommonAckError {
    fn from(value: i32) -> Self {
        match value {
            0 => {CommonAckError::Success}
            1 => {CommonAckError::Version}
            2 => {CommonAckError::Range}
            3 => {CommonAckError::InProgress}
            4 => {CommonAckError::Fail}
            5 => {CommonAckError::OverTemperature}
            6 => {CommonAckError::OperationNotSupported}
            7 => {CommonAckError::InvalidState}
            _ => {panic!("Unknown error code: {}", value)}
        }
    }
}

impl From<mcu_sec::ack::ErrorCode> for CommonAckError {
    fn from(value: mcu_sec::ack::ErrorCode) -> Self {
        match value {
            mcu_sec::ack::ErrorCode::Success => {CommonAckError::Success}
            mcu_sec::ack::ErrorCode::Version => {CommonAckError::Version}
            mcu_sec::ack::ErrorCode::Range => {CommonAckError::Range}
            mcu_sec::ack::ErrorCode::InProgress => {CommonAckError::InProgress}
            mcu_sec::ack::ErrorCode::Fail => {CommonAckError::Fail}
            mcu_sec::ack::ErrorCode::OverTemperature => {CommonAckError::OverTemperature}
            mcu_sec::ack::ErrorCode::OperationNotSupported => {CommonAckError::OperationNotSupported}
            mcu_sec::ack::ErrorCode::InvalidState => {CommonAckError::InvalidState}
        }
    }
}
