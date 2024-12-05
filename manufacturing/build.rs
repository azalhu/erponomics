use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("manufacturing_descriptor.bin"))
        .compile_protos(
            &[
                "item.proto",
                "item_command_service.proto",
                "item_query_service.proto",
                "production_order.proto",
                "production_order_command_service.proto",
                "production_order_query_service.proto",
            ],
            &[
                "proto/v1",
                "proto/v1/item",
                "proto/v1/production_order",
                "../proto/googleapis",
            ],
        )?;

    Ok(())
}
