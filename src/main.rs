use clap::{App, Arg};
use google_cloud_spanner::client::Client;
use nene::generator::TableGenerator;
use nene::repository::TableRepository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database: &str = "projects/local-project/instances/test-instance/databases/local-database";

    std::env::set_var("SPANNER_EMULATOR_HOST", "localhost:9010");
    //let database = std::env::var("SPANNER_DSN");
    let matches = App::new("Spanner ORM Generator")
        .arg(
            Arg::with_name("input_dir")
                .short("i")
                .long("input_dir")
                .help("template directory")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output_dir")
                .short("o")
                .long("output_dir")
                .help("output directory")
                .takes_value(true),
        )
        .get_matches();
    let input = matches.value_of("input_dir");
    let output = matches.value_of("output_dir").unwrap_or("./gen");

    let client = Client::new(database).await?;
    let repository = TableRepository::new(client);
    let generator = TableGenerator::new(repository);

    if input.is_some() {
        generator.generate(input.unwrap(), output).await
    }else {
        generator.generate_default(output).await
    }
}
