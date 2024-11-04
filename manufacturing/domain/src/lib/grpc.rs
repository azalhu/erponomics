#![allow(clippy::all, clippy::pedantic, clippy::nursery)]
pub mod proto {
    pub mod primary_type {
        tonic::include_proto!("erponomics.manufacturing.domain.primary_type");
    }
    pub mod item {
        tonic::include_proto!("erponomics.manufacturing.domain.item");
        pub mod repository {
            tonic::include_proto!("erponomics.manufacturing.domain.item.repository");
        }
    }
}
