fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile_protos(
        &["code.proto", "description.proto", "id.proto", "name.proto"],
        &["proto/primary_type"],
    )?;
    tonic_build::configure().compile_protos(
        &[
            "command.proto",
            "result.proto",
            "status.proto",
            "type.proto",
        ],
        &["proto/sync", "proto/primary_type"],
    )?;
    tonic_build::configure()
        .compile_protos(&["item.proto"], &["proto/item", "proto/primary_type"])?;
    tonic_build::configure().compile_protos(
        &["create_item_request.proto", "delete_item_request.proto"],
        &["proto/item/command", "proto/item", "proto/primary_type"],
    )?;
    tonic_build::configure().compile_protos(
        &["get_item_request.proto", "get_item_response.proto"],
        &["proto/item/query", "proto/item", "proto/primary_type"],
    )?;
    tonic_build::configure().compile_protos(
        &[
            "create_item_request.proto",
            "delete_item_request.proto",
            "get_item_request.proto",
            "get_item_response.proto",
            "item_command_service.proto",
            "item_query_service.proto",
        ],
        &["proto/item/repository", "proto/item", "proto/primary_type"],
    )?;
    tonic_build::configure().compile_protos(
        &["item_sync_command.proto", "item_sync_result.proto"],
        &["proto/item/sync", "proto/sync", "proto/primary_type"],
    )?;

    Ok(())
}
