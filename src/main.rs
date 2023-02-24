use clap::{App, Arg};
use google_cloud_default::WithAuthExt;
use google_cloud_spanner::client::{Client, ClientConfig};
use nene::generator::TableGenerator;
use nene::repository::TableRepository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let database = std::env::var("SPANNER_DSN")?;
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
        .arg(
            Arg::with_name("json")
                .short("j")
                .long("json flag")
                .help("json support")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("default")
                .short("d")
                .long("default flag")
                .help("default trait support")
                .takes_value(false),
        )
        .get_matches();
    let input = matches.value_of("input_dir");
    let output = matches.value_of("output_dir").unwrap_or("./gen");
    let json = matches.is_present("json");
    let default = matches.is_present("default");

    let config = ClientConfig::default().with_auth().await?;
    let client = Client::new(database, config).await?;
    let repository = TableRepository::new(client, json, default);
    let generator = TableGenerator::new(repository);

    if input.is_some() {
        log::info!("generate from custom template output dir is {}", output);
        generator.generate(input.unwrap(), output).await
    } else {
        log::info!("generate from default template output dir is {}", output);
        generator.generate_default(output).await
    }
}
