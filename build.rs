use std::path::Path;

fn main() {
    // NOTE: We cannot use `CARGO_MANIFEST_DIR`, because protoc doesn't work well with
    // absolute paths.
    let is_public = env!("CARGO_PKG_NAME") == "orb-mcu-messaging";
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

    let main_proto = messages_dir.join("mcu_messaging_main.proto");
    let sec_proto = messages_dir.join("mcu_messaging_sec.proto");
    let sec_priv_proto = priv_dir.join("mcu_messaging_sec_priv.proto");

    // Codegen with protoc.
    if let Err(err) = || -> std::io::Result<()> {
        prost_build::Config::default()
            .default_package_filename("mcu_messaging_main")
            .compile_protos(&[main_proto], &[messages_dir])?;

        prost_build::Config::default()
            .default_package_filename("mcu_messaging_sec")
            .compile_protos(&[sec_proto], &[messages_dir, &priv_dir])?;

        prost_build::Config::default()
            .default_package_filename("mcu_messaging_sec_priv")
            .compile_protos(&[sec_priv_proto], &[priv_dir])?;

        Ok(())
    }() {
        panic!("{err}");
    }
}
