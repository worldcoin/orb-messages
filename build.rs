#![forbid(unsafe_code)]

use std::path::Path;

fn main() {
    // NOTE: We cannot use `CARGO_MANIFEST_DIR`, because protoc doesn't work well with
    // absolute paths.
    #[allow(clippy::eq_op)]
    let is_public =
        env!("CARGO_PKG_REPOSITORY") == "https://github.com/worldcoin/orb-messages";
    let (messages_dir, priv_dir) = if is_public {
        println!("cargo:warning=Be aware that private definitions are stubbed out when building the public crate.");

        (Path::new("messages"), Path::new("messages/private_stub"))
    } else {
        (Path::new("public/messages"), Path::new("private"))
    };

    // rebuild if any of this changes
    println!("cargo:rerun-if-env-changed=PROTOC");
    println!("cargo:rerun-if-env-changed=PROTOC_INCLUDE");
    println!("cargo:rerun-if-changed={}", messages_dir.display());
    println!("cargo:rerun-if-changed={}", priv_dir.display());

    // These protos and any others that are imported by them will get compiled
    if let Err(err) = || -> std::io::Result<()> {
        prost_build::Config::default().compile_protos(
            &[messages_dir.join("orb.proto")],
            &[messages_dir, priv_dir],
        )?;

        Ok(())
    }() {
        panic!("{err}");
    }
}
