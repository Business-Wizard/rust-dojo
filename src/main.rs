use rust_dojo;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    rust_dojo::run().await
}
