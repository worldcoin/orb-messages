use std::path::Path;

fn main() {
    // NOTE: We cannot use `CARGO_MANIFEST_DIR`, because protoc doesn't work well with
    // absolute paths.
    let messages_dir = Path::new("messages");
    let priv_dir = messages_dir.join("private");
    let stub_dir = messages_dir.join("private_stub");

    // Choose either `private` or `private_stub`
    let sec_priv_dir = {
        if Path::new(&priv_dir).exists() {
            priv_dir
        } else {
            println!("cargo:warning=No `private` directory found, using `private_stub` instead");
            if !Path::new(&stub_dir).exists() {
                panic!("both private and stubbed directories were missing!");
            }
            stub_dir
        }
    };
    let main_proto = messages_dir.join("mcu_messaging_main.proto");
    let sec_proto = messages_dir.join("mcu_messaging_sec.proto");
    let sec_priv_proto = sec_priv_dir.join("mcu_messaging_sec_priv.proto");

    // rebuild if any of this changes
    println!("cargo:rerun-if-env-changed=PROTOC");
    println!("cargo:rerun-if-env-changed=PROTOC_INCLUDE");
    println!("cargo:rerun-if-changed={}", messages_dir.display());

    // Codegen with protoc.
    if let Err(err) = || -> std::io::Result<()> {
        prost_build::Config::default()
            .default_package_filename("mcu_messaging_main")
            .compile_protos(&[main_proto], &[messages_dir])?;

        prost_build::Config::default()
            .default_package_filename("mcu_messaging_sec")
            .compile_protos(&[sec_proto], &[messages_dir, &sec_priv_dir])?;

        prost_build::Config::default()
            .default_package_filename("mcu_messaging_sec_priv")
            .compile_protos(&[sec_priv_proto], &[sec_priv_dir])?;

        Ok(())
    }() {
        panic!("{err}");
    }
}
