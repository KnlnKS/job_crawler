use reqwest::Url;

pub fn extract_lever(url: String) -> Option<String> {
    let url = "https://jobs.lever.co/palantir/377641a4-7cc6-4336-bfdf-cc6607cbbce8";
    let parsed_url = Url::parse(&url);
    let parsed_url = match parsed_url {
        Ok(parsed_url) => parsed_url,
        Err(_) => return None,
    };

    if parsed_url.host_str() != Some("jobs.lever.co") {
        return None;
    }

    let mut path_segments = match parsed_url.path_segments() {
        Some(path_segments) => path_segments,
        None => return None,
    };

    let company = match path_segments.nth(0) {
        Some(company) => company,
        None => return None,
    };

    return Some(format!(
        "https://api.lever.co/v0/postings/{}?mode=json",
        company
    ));
}
