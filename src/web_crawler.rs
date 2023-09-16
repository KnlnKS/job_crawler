use reqwest::{Client, Url};
use std::collections::HashSet;

pub struct WebCrawler {
    pub host: String,
    pub http_client: Client,
    pub visited: HashSet<String>,
    pub to_visit: Vec<String>,
}

pub fn new_web_crawler(start_url: String) -> WebCrawler {
    // parse the URL
    let url = match Url::parse(&start_url) {
        Ok(url) => url,
        Err(err) => panic!("Error: {}", err),
    };

    // extract the host from the URL
    let host = match url.host_str() {
        Some(host) => host,
        None => panic!("URL '{}' does not have a host", start_url),
    };

    // create a new vector to hold the URLs to visit and add the start URL
    let mut to_visit = Vec::new();
    to_visit.push(start_url);

    // create a new WebCrawler
    return WebCrawler {
        host: host.to_string(),
        http_client: Client::new(),
        visited: HashSet::new(),
        to_visit,
    };
}

impl WebCrawler {
    pub async fn start(&mut self) {
        while let Some(url) = self.to_visit.pop() {
            if self.visited.contains(&url) {
                continue;
            }

            self.visited.insert(url.clone());
            println!("Visiting {}", url);

            let response = self.http_client.get(url).send().await;

            let j = match response {
                Ok(response) => {
                    let body = response.text().await;
                    match body {
                        Ok(body) => {
                            self.scrape_page_for_links(body);
                        }
                        Err(err) => {
                            println!("Error: {}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            };
        }
    }
}
