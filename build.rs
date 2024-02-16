use std::path::{Path, PathBuf};

const PRIVATE_PATH_ENV: &str = "ORB_MCU_MESSAGING_PRIVATE_PATH";

fn main() {
    // NOTE: We cannot use `CARGO_MANIFEST_DIR`, because protoc doesn't work well with
    // absolute paths.
    let messages_dir = Path::new("messages");

    // rebuild if any of this changes
    println!("cargo:rerun-if-env-changed=PROTOC");
    println!("cargo:rerun-if-env-changed=PROTOC_INCLUDE");
    println!("cargo:rerun-if-env-changed={PRIVATE_PATH_ENV}");
    println!("cargo:rerun-if-changed={}", messages_dir.display());

    let sec_priv_dir = get_sec_priv_dir(messages_dir);

    let main_proto = messages_dir.join("mcu_messaging_main.proto");
    let sec_proto = messages_dir.join("mcu_messaging_sec.proto");
    let sec_priv_proto = sec_priv_dir.join("mcu_messaging_sec_priv.proto");

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
        panic!("error while running protoc. Hint: Did you specify {PRIVATE_PATH_ENV} correctly?\n{err}");
    }
}

fn get_sec_priv_dir(messages_dir: &Path) -> PathBuf {
    // Choose either `private` or `private_stub`
    let sec_priv_dir = if cfg!(feature = "private") {
        let Ok(priv_dir) = std::env::var(PRIVATE_PATH_ENV) else {
            panic!(
                "The `private` feature of this crate requires you to provide the \
                {PRIVATE_PATH_ENV} env var to the private protobufs."
            );
        };
        println!("cargo:rerun-if-changed={priv_dir}");
        println!("cargo:warning=using private protobufs from `{priv_dir}`");
        PathBuf::from(priv_dir)
    } else {
        println!(
            "cargo:warning=`private` cargo feature not required, using stubs for \
            private protobufs instead."
        );
        messages_dir.join("private_stub")
    };
    assert!(
        !sec_priv_dir.to_str().unwrap().contains("~"),
        "`~` must be expanded by your shell before used in the build script. See \
        https://stackoverflow.com/a/5748307"
    );
    assert!(
        sec_priv_dir.exists() && sec_priv_dir.is_dir(),
        "expected {PRIVATE_PATH_ENV}={sec_priv_dir:?} dir to exist."
    );

    sec_priv_dir
}
