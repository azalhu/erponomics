use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let mut config = prost_build::Config::new();
    config.enable_type_names();

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("manufacturing_descriptor.bin"))
        .compile_protos_with_config(
            config,
            &["item.proto"],
            &["../erponomics/manufacturing/v1", "../googleapis"],
        )?;

    Ok(())
}
