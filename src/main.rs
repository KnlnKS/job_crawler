mod web_crawler;
mod locales;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut crawler = web_crawler::new_web_crawler("https://www.rust-lang.org/".to_string());
    crawler.start().await;
}
