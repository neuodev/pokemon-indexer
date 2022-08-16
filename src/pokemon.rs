use colored::Colorize;
use select::document::Document;
use select::predicate::Class;
use std::path::Path;
use tokio::fs::{self, File};
use tokio::io::{self, AsyncWriteExt};

const NUM_OF_PAGES: u8 = 252;
const OUTPUT_JSON_FILE: &str = "./output/urls.json";
const OUTPUT_DIR: &str = "./output";

/// Fetch the HTML per page
/// There is about 252 page on this website https://pkmncards.com/page/1/?s with over more than 15,000 image
pub async fn fetch_page(num: u8) -> Result<String, reqwest::Error> {
    let response = reqwest::get(format!("https://pkmncards.com/page/{}/?s", num)).await?;
    Ok(response.text().await?)
}

/// Parse HTML response and extract `src` attr from all images
pub fn extract_img_urls(html: &str) -> Vec<String> {
    let doc = Document::from(html);
    let image_elements = doc.find(Class("card-image"));
    let urls = image_elements
        .into_iter()
        .filter_map(|n| n.attr("src"))
        .map(|src| src.to_string())
        .collect::<Vec<String>>();
    urls
}

pub async fn save_page_images(urls: Vec<String>, page: u8) {
    fs::create_dir_all(format!("./output/{page}").as_str())
        .await
        .unwrap();

    let mut handlers = vec![];
    for (idx, url) in urls.into_iter().enumerate() {
        let handler = tokio::spawn(async move {
            if let Ok(resp) = reqwest::get(url).await {
                println!("[image fetched] [page]: {}, [image]: {}", page, idx);
                if let Ok(bytes) = resp.bytes().await {
                    let mut out = File::create(format!("./output/{}/{}.png", page, idx + 1))
                        .await
                        .unwrap(); // Todo: remove unwrap
                    let mut resp_slice: &[u8] = bytes.as_ref();
                    io::copy(&mut resp_slice, &mut out)
                        .await
                        .expect("failed to copy content");
                } else {
                    println!("Unable to read bytes for {idx} on page {page}");
                }
            } else {
                println!(
                    "[error]: [page]: {}, [image]: {}",
                    format!("{}", page).bold().underline().red(),
                    format!("{}", idx).bold().underline().red()
                );
            }
        });

        handlers.push(handler)
    }

    for h in handlers {
        h.await.unwrap();
    }
}

// Save all urls for all 15,000 images into a json
async fn save_urls(urls: Vec<String>) {
    // Create the output dir if is not exsit
    let path = Path::new(OUTPUT_DIR);
    if !path.exists() {
        fs::create_dir(OUTPUT_DIR)
            .await
            .expect("Failed to create the output dir.");
    }

    let mut file = File::create(OUTPUT_JSON_FILE)
        .await
        .expect("Unable to create output json file");
    let json = serde_json::to_string(&urls).expect("Invalid json");
    file.write_all(json.as_bytes())
        .await
        .expect("Failed to write to the ouptut json file");
}

pub async fn pokemon_download(args: &crate::Args) {
    let total_iters = NUM_OF_PAGES / args.pages;
    println!("Total iterations: {}", total_iters);

    let mut all_urls = vec![];
    let should_save_images = args.save_images;

    for i in 1..=total_iters {
        let offset = ((i - 1) * args.pages) + 1;
        let mut handlers = vec![];
        println!("[chunk]: {}", format!("{}", i).bold().underline().yellow());
        for page in offset..offset + args.pages {
            let handler = tokio::spawn(async move {
                match fetch_page(page).await {
                    Ok(result) => {
                        println!("[page]: {}", format!("{}", page).bold().underline().cyan());
                        let urls = extract_img_urls(result.as_str());
                        if should_save_images == true {
                            save_page_images(urls.clone(), page).await;
                        }
                        println!("[done]: {}", format!("{}", page).bold().underline().green());
                        return urls;
                    }
                    Err(e) => {
                        println!(
                            "{}",
                            format!("Error processing page '{}'", page)
                                .bold()
                                .underline()
                                .red()
                        );
                        println!("{e}");
                        return vec![];
                    }
                }
            });

            handlers.push(handler);
        }

        for handler in handlers {
            let mut urls = handler.await.unwrap();
            all_urls.append(&mut urls);
        }
    }

    if args.json == true {
        save_urls(all_urls).await;
    }
}

pub async fn load_urls_in_memory() -> Vec<String> {
    let path = Path::new(OUTPUT_JSON_FILE);

    if !path.exists() {
        panic!("{} is missing", OUTPUT_JSON_FILE);
    }

    let json_str = fs::read_to_string(path)
        .await
        .expect("Unable to read urls.json file");
    let urls: Vec<String> = serde_json::from_str(&json_str).expect("Failed to parse urls");

    urls
}
