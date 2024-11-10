#![allow(clippy::all, clippy::pedantic, clippy::nursery)]
pub mod proto {
    pub mod primary_type {
        tonic::include_proto!("erponomics.manufacturing.domain.primary_type");
    }
    pub mod sync {
        tonic::include_proto!("erponomics.manufacturing.domain.sync");
    }
    pub mod item {
        tonic::include_proto!("erponomics.manufacturing.domain.item");
        pub mod command {
            tonic::include_proto!("erponomics.manufacturing.domain.item.command");
        }
        pub mod query {
            tonic::include_proto!("erponomics.manufacturing.domain.item.query");
        }
        pub mod repository {
            tonic::include_proto!("erponomics.manufacturing.domain.item.repository");
        }
        pub mod sync {
            tonic::include_proto!("erponomics.manufacturing.domain.item.sync");
        }
    }
}
