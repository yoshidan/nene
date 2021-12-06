use google_cloud_spanner::client::Client;
use nene::repository::TableRepository;
use nene::generator::{TableGenerator, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let database : &str =
        "projects/local-project/instances/test-instance/databases/local-database";

    std::env::set_var("SPANNER_EMULATOR_HOST", "localhost:9010");
    //let database = std::env::var("SPANNER_DSN");

    let client = Client::new(database).await?;
    let repository = TableRepository::new(client);
    let generator = TableGenerator::new(repository);
    generator.generate(Config {
        output_dir: "./src/gen".to_string(),
        input_dir: "./src/default".to_string()
    }).await
}
