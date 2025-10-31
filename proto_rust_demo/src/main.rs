use grpc_demo::run_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting gRPC server...");
    run_server().await
}
