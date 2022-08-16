mod pokemon;
use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use pokemon::{load_urls_in_memory, pokemon_download};
use serde::{Deserialize, Serialize};
use actix_files as fs;
use actix_files::NamedFile;

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

    if args.json | args.save_images {
        pokemon_download(&args).await;
    }

    let urls = load_urls_in_memory().await;

    println!("Server running on {}", args.addr);
    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(urls.clone()))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(index)
            .service(web::scope("/api/v1").service(get_pokemons))
    })
    .bind(args.addr)?
    .run()
    .await?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct PokemonResponse {
    urls: Vec<String>,
    count: usize,
    num_of_pages: usize,
}

#[derive(Serialize, Deserialize)]
struct PokemonQuery {
    page: Option<i32>,
    page_size: Option<i32>,
}

#[get("/pokemon")]
async fn get_pokemons(query: web::Query<PokemonQuery>, urls: web::Data<Vec<String>>) -> impl Responder {
    let page = query.page.unwrap_or_else(|| 1);
    let page_size = query.page_size.unwrap_or_else(|| 10);
    let skip = (page - 1) * page_size;
    let urls: Vec<String> = urls.into_inner().to_vec();
    let count = urls.len();
    let urls = urls
        .into_iter()
        .skip(skip as usize)
        .take(page_size as usize)
        .collect::<Vec<String>>();

    let resp = PokemonResponse {
        urls,
        count,
        num_of_pages: count / page_size as usize,
    };

    let json = serde_json::to_string_pretty(&resp).unwrap();
    HttpResponse::Ok().body(json)
}

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}