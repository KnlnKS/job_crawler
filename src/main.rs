mod web_crawler;

#[tokio::main]
async fn main() {
    let mut crawler = web_crawler::web_crawler::new_web_crawler("https://www.retool.com".to_string());
    crawler.start().await;
}
