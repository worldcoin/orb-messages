fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=./mcu_messaging_common.options");
    println!("cargo:rerun-if-changed=./mcu_messaging_common.proto");
    println!("cargo:rerun-if-changed=./mcu_messaging_main.options");
    println!("cargo:rerun-if-changed=./mcu_messaging_main.proto");
    println!("cargo:rerun-if-changed=./mcu_messaging_sec.options");
    println!("cargo:rerun-if-changed=./mcu_messaging_sec.proto");

    // remove the symlink if it already exists, to make sure it gets updated
    std::fs::remove_file("./mcu_messaging_sec_priv.proto").unwrap_or(());

    if std::path::Path::new("private").exists() {
        println!("cargo:rerun-if-changed=private/mcu_messaging_sec_priv.options'");
        println!("cargo:rerun-if-changed=private/mcu_messaging_sec_priv.proto");

        std::os::unix::fs::symlink(
            "private/mcu_messaging_sec_priv.proto",
            "mcu_messaging_sec_priv.proto",
        )
            .unwrap();
    } else {
        println!("cargo:rerun-if-changed=./mcu_messaging_sec_obfuscated.proto");

        std::os::unix::fs::symlink(
            "./mcu_messaging_sec_obfuscated.proto",
            "./mcu_messaging_sec_priv.proto",
        )
            .unwrap();
        println!("cargo:warning=`private` directory not found, using obfuscated protobuf definitions");
    }

    prost_build::Config::default()
        .default_package_filename("mcu_messaging_main")
        .compile_protos(&["./mcu_messaging_main.proto"], &["./"])?;

    prost_build::Config::default()
        .default_package_filename("mcu_messaging_sec")
        .compile_protos(&["./mcu_messaging_sec.proto"], &["./"])?;

    prost_build::Config::default()
        .default_package_filename("mcu_messaging_sec_priv")
        .compile_protos(&["./mcu_messaging_sec_priv.proto"], &["./"])?;

    Ok(())
}
