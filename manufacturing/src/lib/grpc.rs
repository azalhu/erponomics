#![allow(clippy::all, clippy::pedantic, clippy::nursery)]
pub mod proto {
    pub(crate) mod google {
        pub(crate) mod api {
            tonic::include_proto!("google.api");
        }
        pub(crate) mod longrunning {
            tonic::include_proto!("google.longrunning");
        }
        pub(crate) mod rpc {
            tonic::include_proto!("google.rpc");
        }
    }
    pub mod erponomics {
        pub mod manufacturing {
            pub mod v1 {
                pub mod item {
                    tonic::include_proto!("erponomics.manufacturing.v1.item");
                }
                pub mod production_order {
                    tonic::include_proto!("erponomics.manufacturing.v1.production_order");
                }
            }
        }
    }
}
