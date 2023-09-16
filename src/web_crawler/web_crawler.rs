use regex::Regex;
use reqwest::{Client, Url};
use std::collections::HashSet;

use super::link_filters::{is_same_domain, is_valid_link, is_wanted_file, is_wanted_locale};

pub struct WebCrawler {
    pub domain: String,
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

    // extract the domain from the URL
    let domain = match url.domain() {
        Some(domain) => domain,
        None => panic!("URL '{}' does not have a host", start_url),
    };

    // create a new vector to hold the URLs to visit and add the start URL
    let mut to_visit = Vec::new();
    let mut visited = HashSet::new();
    visited.insert(start_url.clone());
    to_visit.push(start_url);

    // create a new WebCrawler
    return WebCrawler {
        domain: domain.to_string(),
        http_client: Client::new(),
        visited,
        to_visit,
    };
}

impl WebCrawler {
    pub async fn start(&mut self) {
        while let Some(url) = self.to_visit.pop() {
            println!("Visiting {}", url);
            println!("");

            let response = self.http_client.get(url.clone()).send().await;

            match response {
                Ok(response) => {
                    let body = response.text().await;
                    match body {
                        Ok(body) => {
                            let parsed_url = Url::parse(&url).unwrap();
                            let host = parsed_url.host_str().unwrap();
                            self.scrape_page_for_links(body, host);
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

    fn scrape_page_for_links(&mut self, body: String, host: &str) {
        // scrape the page for links
        let re_url = Regex::new(r#"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)"#).unwrap();

        // Turn to vec
        let mut links = re_url
            .captures_iter(&body)
            .map(|capture| capture[0].to_string())
            .collect::<Vec<String>>();

        // scrape the page for relative links
        let re_relative_url = Regex::new(r#"["']\/([^>"']+)["']"#).unwrap();
        let relative_links = re_relative_url
            .captures_iter(&body)
            .map(|capture| {
                if capture[1].starts_with('/') {
                    return format!("https://{}{}", host, &capture[1]);
                }
                return format!("https://{}/{}", host, &capture[1]);
            })
            .collect::<Vec<String>>();

        // combine the links and relative_links
        links.extend(relative_links);

        // print out any matching links
        links.iter().for_each(|link| {
            if (link.contains("greenhouse")) {
                println!("talloh link: {}", link);
            }
        });

        let mut filtered_links = links
            .iter()
            .filter(|link| {
                is_valid_link(link)
                    && is_wanted_locale(link)
                    && is_same_domain(link, &self.domain)
                    && is_wanted_file(link)
            })
            .map(|link| {
                let mut url = Url::parse(link).unwrap();
                url.set_fragment(None);
                return url.to_string();
            })
            .collect::<Vec<String>>();

        filtered_links.sort_by(|a, b| b.cmp(a));

        for link in filtered_links {
            if self.visited.contains(&link) {
                continue;
            }
            self.visited.insert(link.clone());
            self.to_visit.push(link.to_string());
            println!("Found link: {}", link);
        }
    }
}
