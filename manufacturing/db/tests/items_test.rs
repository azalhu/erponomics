use std::str::FromStr;

use manufacturing::{
    item::repository::{create, delete, get},
    proto::item::repository::{
        item_command_service_client::ItemCommandServiceClient,
        item_query_service_client::ItemQueryServiceClient, CreateItemRequest, DeleteItemRequest,
        GetItemRequest,
    },
    Code, Description, Id, Item, Name, Timestamp,
};
use tonic::Request;

#[tokio::test]
async fn it_creates_new_item() -> Result<(), Box<dyn std::error::Error>> {
    let mut command_client = ItemCommandServiceClient::connect("http://localhost:8084").await?;
    let mut query_client = ItemQueryServiceClient::connect("http://localhost:8084").await?;

    let id = Id::default();
    let code = Code::from_str("P10000")?;
    let name = Name::from_str("Pedal")?;
    let description = Description::from_str("Pedal for bicycle")?;
    let created_at = Timestamp::now();
    let item = Item::from((
        id.clone(),
        code.clone(),
        name.clone(),
        description.clone(),
        created_at.clone(),
    ));
    let request = create::Request::new(item);
    let request: Request<CreateItemRequest> = request.into();

    command_client.create_item(request).await?;

    let request = get::Request::new(id.clone());
    let request: Request<GetItemRequest> = request.into();

    let response = query_client.get_item(request).await?;
    let response: get::Response = response.try_into()?;
    let item = response.item();

    assert_eq!(id, item.id().clone());
    assert_eq!(code, item.code().clone());
    assert_eq!(name, item.name().clone());
    assert_eq!(created_at, item.created_at().clone());

    let request = delete::Request::new(id.clone());
    let request: Request<DeleteItemRequest> = request.into();

    command_client.delete_item(request).await?;

    let request = get::Request::new(id.clone());
    let request: Request<GetItemRequest> = request.into();

    let response = query_client.get_item(request).await.unwrap_err();
    let error: get::Error = response.into();

    assert!(matches!(error, get::Error::NotFound));

    Ok(())
}
