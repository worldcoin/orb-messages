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
