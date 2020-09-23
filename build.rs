fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &[
            "proto/unitoken/node/grpc/accounts_api.proto",
            "proto/unitoken/node/grpc/assets_api.proto",
            "proto/unitoken/node/grpc/blockchain_api.proto",
            "proto/unitoken/node/grpc/blocks_api.proto",
            "proto/unitoken/node/grpc/transactions_api.proto",
            "proto/unitoken/events/events.proto",
        ],
        &["proto"],
    )?;

    Ok(())
}
