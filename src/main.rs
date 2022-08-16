mod pokemon;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
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
    #[clap(long, value_parser, default_value = "127.0.0.1:8080")]
    addr: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    println!("App settings: {:#?}", args);
    pokemon_download(&args).await;

    println!("Server running on {}", args.addr);
    HttpServer::new(|| App::new().service(web::scope("/api/v1").service(index)))
        .bind(args.addr)?
        .run()
        .await?;

    Ok(())
}

#[get("/pokemon")]
async fn index() -> impl Responder {
    "hello, world"
}
