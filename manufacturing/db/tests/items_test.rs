use db::inbound::grpc::item::{item_client::ItemClient, ItemRequest};
use tonic::Request;

#[tokio::test]
async fn it_creates_new_item() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ItemClient::connect("http://localhost:1339").await?;

    let request = Request::new(ItemRequest {
        number: "P10000".to_owned(),
        name: "Pedal".to_owned(),
    });

    let response = client.create_item(request).await?.into_inner();

    assert_eq!(response.id, "0");
    assert_eq!(response.number, "P10000");
    assert_eq!(response.name, "Pedal");

    Ok(())
}
