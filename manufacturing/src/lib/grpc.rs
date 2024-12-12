#![allow(clippy::all, clippy::pedantic, clippy::nursery)]
pub mod base;
pub mod core;

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
                tonic::include_proto!("erponomics.manufacturing.v1");
            }
        }
    }
}
