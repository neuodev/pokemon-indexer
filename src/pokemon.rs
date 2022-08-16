use colored::Colorize;
use select::document::Document;
use select::predicate::{Class};
// use std::io;
// use std::fs::File;
use tokio::fs::{File, self};
use tokio::io;

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
    let urls = image_elements.into_iter().filter_map(|n| n.attr("src")).map(|src| src.to_string()).collect::<Vec<String>>();
    urls
}

pub async fn save_page_images(urls: Vec<String>, page: u8) {
    fs::create_dir_all(format!("./output/{page}").as_str()).await.unwrap();

    let mut handlers = vec![];
    for (idx, url) in urls.into_iter().enumerate() {
        let handler = tokio::spawn(async move {
            if let Ok(resp) = reqwest::get(url).await {
                println!("[image fetched] [page]: {}, [image]: {}", page, idx);
                if let Ok(bytes) = resp.bytes().await {
                    let mut out = File::create(format!("./output/{}/{}.png", page, idx + 1)).await.unwrap(); // Todo: remove unwrap
                    let mut resp_slice: &[u8] = bytes.as_ref();
                    io::copy(&mut resp_slice, &mut out).await.expect("failed to copy content");
                } else {
                    println!("Unable to read bytes for {idx} on page {page}");
                }
            } else {
                println!("[error]: [page]: {}, [image]: {}", format!("{}", page).bold().underline().red(), format!("{}", idx).bold().underline().red());
            }
        });

        handlers.push(handler)
    }

    for h in handlers {
        h.await.unwrap();
    }
}

