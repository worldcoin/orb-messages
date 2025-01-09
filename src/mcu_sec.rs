#![allow(clippy::all, clippy::pedantic)]

include!(concat!(env!("OUT_DIR"), "/mcu_messaging_sec.rs"));

pub mod private {
    include!(concat!(env!("OUT_DIR"), "/private.rs"));
}
