pub mod unitoken {
    tonic::include_proto!("unitoken");

    pub mod events {
        tonic::include_proto!("unitoken.events");
    }

    pub mod node {
        pub mod grpc {
            tonic::include_proto!("unitoken.node.grpc");
        }
    }
}
