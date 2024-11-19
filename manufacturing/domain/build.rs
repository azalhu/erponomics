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
                "item_repository_command_service.proto",
                "item_repository_query_service.proto",
                "production_order.proto",
                "production_order_command_service.proto",
                "production_order_query_service.proto",
                "production_order_repository_command_service.proto",
                "production_order_repository_query_service.proto",
            ],
            &[
                "../v1",
                "../v1/sync",
                "../v1/item",
                "../v1/item/repository",
                "../v1/production_order",
                "../v1/production_order/repository",
            ],
        )?;

    Ok(())
}
