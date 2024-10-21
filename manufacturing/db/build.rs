fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/items.proto")?;

    tonic_build::configure().compile_protos(
        &[
            "item.proto",
            "item_id.proto",
            "item_number.proto",
            "item_service.proto",
            "create_item_request.proto",
            "create_item_response.proto",
            "get_item_request.proto",
            "get_item_response.proto",
        ],
        &["proto/item"],
    )?;

    Ok(())
}
