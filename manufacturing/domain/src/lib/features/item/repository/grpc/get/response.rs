use tonic::Response;

use super::{get, proto};

impl From<get::Response> for Response<proto::GetItemResponse> {
    fn from(value: get::Response) -> Self {
        let item = value.dissolve();

        Self::new(proto::GetItemResponse { item: item.into() })
    }
}

impl TryFrom<Response<proto::GetItemResponse>> for get::Response {
    type Error = get::Error;

    fn try_from(value: Response<proto::GetItemResponse>) -> Result<Self, Self::Error> {
        let value = value.into_inner().item.try_into()?;

        Ok(Self::new(value))
    }
}
