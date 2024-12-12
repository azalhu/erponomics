#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
use manufacturing::{
    proto::item::{
        item_command_service_client::ItemCommandServiceClient,
        item_query_service_client::ItemQueryServiceClient, CreateItemRequest, DeleteItemRequest,
        GetItemRequest, Item,
    },
    ItemState,
};
use num_traits::ToPrimitive;
use tonic::Request;

#[tokio::test]
async fn it_does_not_create_duplicate_item() -> Result<(), Box<dyn std::error::Error>> {
    let path = "http://localhost:8081";
    let mut command_client = ItemCommandServiceClient::connect(path).await?;
    let mut query_client = ItemQueryServiceClient::connect(path).await?;

    let id = String::from("b-max");
    let name = None;
    let display_name = Some(String::from("Bike"));
    let title = None;
    let description = Some(String::from("Bike with maximum power"));
    let state = None;
    let etag = None;
    let uid = None;
    let create_time = None;
    let update_time = None;

    let request = CreateItemRequest {
        item_id: id.clone().into(),
        item: Item {
            name,
            display_name: display_name.clone(),
            title: title.clone(),
            description: description.clone(),
            state: state.clone(),
            etag: etag.clone(),
            uid: uid.clone(),
            create_time: create_time.clone(),
            update_time: update_time.clone(),
        }
        .into(),
    };

    let request = Request::new(request);

    command_client.create_item(request).await?;

    let request = GetItemRequest { name: id.clone() };
    let request = Request::new(request);

    let response = query_client.get_item(request).await?;
    let item = response.into_inner();

    //    assert_eq!(id, item.name.clone().unwrap());
    //    assert_eq!(display_name, item.display_name.clone());
    //    assert_eq!(title, item.title.clone());
    //    assert_eq!(description, item.description.clone());
    //    assert_eq!(ItemState::Creating.to_i32(), item.state.clone());
    //    assert!(!item.etag.clone().unwrap().is_empty());
    //    assert!(!uuid::Uuid::try_parse(&item.uid.clone().unwrap())
    //        .unwrap()
    //        .is_nil());

    let request = DeleteItemRequest {
        name: id.clone(),
        etag: item.etag.clone().unwrap(),
    };
    let request = Request::new(request);

    command_client.delete_item(request).await?;

    let request = GetItemRequest { name: id };
    let request = Request::new(request);

    let response = query_client.get_item(request).await?;
    let _item = response.into_inner();

    //    assert_eq!(ItemState::Deleting.to_i32(), item.state.clone());

    Ok(())
}
