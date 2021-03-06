fn build_grpc() {
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["proto/exchange/matchengine.proto"],
            &["proto/exchange", "proto/third_party/googleapis"],
        )
        .unwrap();
}

fn main() {
    build_grpc()
}
