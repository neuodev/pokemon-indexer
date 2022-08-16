mod pokemon;
use clap::Parser;
use pokemon::pokemon_download;

#[derive(Parser, Debug)]
#[clap(allow_negative_numbers = false)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser, default_value_t = 5)]
    pages: u8,
    #[clap(short, long, value_parser, default_value_t = false)]
    json: bool,
    #[clap(short, long, value_parser, default_value_t = false)]
    save_images: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    println!("App settings: {:#?}", args);
    pokemon_download(&args).await;

    Ok(())
}
