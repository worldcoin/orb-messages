use std::fmt;

/// Error returned when `zenoh_suffix_key()` is called on a payload not marked with `@zenoh`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotAZenohPayload;

impl fmt::Display for NotAZenohPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "payload is not marked with @zenoh in the proto file")
    }
}

impl std::error::Error for NotAZenohPayload {}

/// Trait for types that have an associated Zenoh suffix key.
///
/// The suffix is used to construct the full Zenoh key path for
/// MCU-to-Jetson messages. The format is `{source}/{payload_name}` where:
/// - `source` is either `main` or `sec` depending on which MCU sent the message
/// - `payload_name` is the snake_case name of the payload type
///
/// ## Opting In
///
/// Only payload fields marked with `// @zenoh` in the proto file will have
/// zenoh suffix keys. Add the marker to any field you want to publish:
///
/// ```protobuf
/// orb.mcu.Versions versions = 6; // @zenoh
/// ```
///
/// Calling `zenoh_suffix_key()` on an unmarked payload returns `Err(NotAZenohPayload)`.
pub trait ZenohKey {
    /// Returns the Zenoh suffix key for this type.
    ///
    /// # Errors
    ///
    /// Returns `Err(NotAZenohPayload)` if called on a payload not marked with
    /// `@zenoh` in the proto file.
    ///
    /// # Example
    /// ```ignore
    /// use orb_messages::{main::mcu_to_jetson::Payload, ZenohKey};
    ///
    /// // Only works for @zenoh-marked payloads
    /// let payload = Payload::Versions(Default::default());
    /// assert_eq!(payload.zenoh_suffix_key().unwrap(), "main/versions");
    /// ```
    fn zenoh_suffix_key(&self) -> Result<&'static str, NotAZenohPayload>;
}

// Auto-generated implementations are included from build.rs output
include!(concat!(env!("OUT_DIR"), "/zenoh_keys.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::main::mcu_to_jetson::Payload as MainPayload;
    use crate::sec::sec_to_jetson::Payload as SecPayload;

    #[test]
    fn test_main_payload_zenoh_keys() {
        // Only @zenoh-marked payloads have zenoh keys
        let versions = MainPayload::Versions(Default::default());
        assert_eq!(versions.zenoh_suffix_key().unwrap(), "main/versions");

        let hardware = MainPayload::Hardware(Default::default());
        assert_eq!(hardware.zenoh_suffix_key().unwrap(), "main/hardware");
    }

    #[test]
    fn test_sec_payload_zenoh_keys() {
        // Only @zenoh-marked payloads have zenoh keys
        let versions = SecPayload::Versions(Default::default());
        assert_eq!(versions.zenoh_suffix_key().unwrap(), "sec/versions");

        let tamper = SecPayload::Tamper(Default::default());
        assert_eq!(tamper.zenoh_suffix_key().unwrap(), "sec/tamper");
    }

    #[test]
    fn test_unmarked_payload_returns_error() {
        // PowerButton is not marked with @zenoh, so this returns an error
        let power_button = MainPayload::PowerButton(Default::default());
        assert_eq!(power_button.zenoh_suffix_key(), Err(NotAZenohPayload));
    }
}
