use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("manufacturing_descriptor.bin"))
        .compile_protos(
            &[
                "code.proto",
                "description.proto",
                "id.proto",
                "name.proto",
                "command.proto",
                "result.proto",
                "status.proto",
                "type.proto",
                "item.proto",
                "create_item_request.proto",
                "delete_item_request.proto",
                "item_command_service.proto",
                "get_item_request.proto",
                "get_item_response.proto",
                "item_query_service.proto",
                "repository_create_item_request.proto",
                "repository_delete_item_request.proto",
                "repository_get_item_request.proto",
                "repository_get_item_response.proto",
                "repository_item_command_service.proto",
                "repository_item_query_service.proto",
                "item_sync_command.proto",
                "item_sync_result.proto",
            ],
            &[
                "proto/primary_type",
                "proto/sync",
                "proto/item",
                "proto/item/command",
                "proto/item/query",
                "proto/item/repository",
                "proto/item/sync",
            ],
        )?;

    Ok(())
}
