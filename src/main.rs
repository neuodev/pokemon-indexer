mod pokemon;
use colored::Colorize;
use clap::Parser;

const NUM_OF_PAGES: u8 = 252;
const PAGES_TO_PROCESS: u8 = 5;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value_t = 5)]
    pages: u8,
    #[clap(short, long, value_parser, default_value_t = false)]
    json: bool,
    #[clap(short, long, value_parser, default_value_t = true)]
    save_images: bool
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let total_iters =  NUM_OF_PAGES / PAGES_TO_PROCESS;
    println!("Total iterations: {}", total_iters);

    for i in 1..=total_iters {        
        let offset = ((i - 1) * PAGES_TO_PROCESS) + 1;
        let mut handlers = vec![];
        println!("[chunk]: {}", format!("{}", i).bold().underline().yellow());
        for page in offset..offset + PAGES_TO_PROCESS {
            let handler = tokio::spawn(async move {
                match pokemon::fetch_page(page).await {
                    Ok(result) => {
                        println!("[page]: {}", format!("{}", page ).bold().underline().cyan());
                        let urls = pokemon::extract_img_urls(result.as_str());
                        pokemon::save_page_images(urls, page).await;
                        println!("[done]: {}", format!("{}", page).bold().underline().green());
                    },
                    Err(e) => {
                        println!("{}", format!("Error processing page '{}'", page).bold().underline().red());
                        println!("{e}");
                    }
                }
            });

            handlers.push(handler);
        } 

        for handler in handlers {
            handler.await.unwrap();
        }
    }


    Ok(())
}
