use zero_to_prod;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    zero_to_prod::run().await
}
