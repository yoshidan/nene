use clap::{App, Arg};
use google_cloud_spanner::client::Client;
use nene::generator::TableGenerator;
use nene::repository::TableRepository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let database = std::env::var("SPANNER_DSN");
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
        log::info!("generate from custom template output dir is {}", output);
        generator.generate(input.unwrap(), output).await
    }else {
        log::info!("generate from default template output dir is {}", output);
        generator.generate_default(output).await
    }
}
