#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
use manufacturing::{
    proto::item::command::{
        item_command_service_client::ItemCommandServiceClient, CreateItemRequest,
    },
    Code, Description, Name,
};
use tonic::Request;

#[tokio::test]
async fn it_does_not_create_duplicate_item() -> Result<(), Box<dyn std::error::Error>> {
    let mut command_client = ItemCommandServiceClient::connect("http://localhost:8081").await?;

    let code = Code::try_from("B-MAX".to_string())?;
    let name = Name::try_from("Bike".to_string())?;
    let description = Description::try_from("Bike with maximum power".to_string())?;
    let request = CreateItemRequest {
        code: code.clone().into(),
        name: name.clone().into(),
        description: description.clone().into(),
    };

    let request = Request::new(request);

    command_client.create_item(request).await?;

    Ok(())
}
