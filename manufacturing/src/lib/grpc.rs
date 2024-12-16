pub mod item;
pub mod status;
pub mod sync;
pub mod timestamp;

pub mod proto {
    #![allow(clippy::all, clippy::pedantic, clippy::nursery)]
    pub mod google {
        pub mod api {
            tonic::include_proto!("google.api");
        }
        pub mod longrunning {
            tonic::include_proto!("google.longrunning");
        }
        pub mod rpc {
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
