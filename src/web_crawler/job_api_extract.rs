use reqwest::Url;

pub fn extract_job_api(url: String) -> Option<String> {
    if url.contains("lever") {
        return extract_lever(url);
    } else if url.contains("greenhouse") {
        return extract_greenhouse(url);
    } else {
        return None;
    }
}

fn extract_lever(url: String) -> Option<String> {
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

fn extract_greenhouse(url: String) -> Option<String> {
    if url.contains("greenhouse.io/embed/") {
        let parsed_url = Url::parse(&url);
        let parsed_url = match parsed_url {
            Ok(parsed_url) => parsed_url,
            Err(_) => return None,
        };

        for (key, value) in parsed_url.query_pairs() {
            if key == "for" {
                return Some(format!(
                    "https://boards-api.greenhouse.io/v1/boards/{}/jobs",
                    value
                ));
            }
        }
    } else if url.contains("greenhouse.io") {
        let parsed_url = Url::parse(&url);
        let parsed_url = match parsed_url {
            Ok(parsed_url) => parsed_url,
            Err(_) => return None,
        };

        let mut path_segments = match parsed_url.path_segments() {
            Some(path_segments) => path_segments,
            None => return None,
        };

        let company = match path_segments.nth(0) {
            Some(company) => company,
            None => return None,
        };

        return Some(format!(
            "https://boards-api.greenhouse.io/v1/boards/{}/jobs",
            company
        ));
    }
    return None;
}
