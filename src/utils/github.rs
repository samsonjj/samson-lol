use super::encode_url;

pub async fn construct_github_url(rest: &str) -> String {
    if rest.len() == 0 {
        return String::from("https://github.com");
    }
    match &rest[0..1] {
        "@" => construct_github_profile_url(&rest[1..]),
        _ => {
            let page_url = construct_github_page_url(rest);
            println!("pinging {page_url}");
            if let Ok(_) = super::ping_url(&page_url).await {
                page_url
            } else {
                construct_github_search_url(rest)
            }
        }
    }
}

pub fn construct_github_profile_url(profile: &str) -> String {
    format!("https://github.com/{}", encode_url(profile))
}

pub fn construct_github_search_url(query: &str) -> String {
    format!("https://github.com/search?q={}", encode_url(query))
}

pub fn construct_github_page_url(query: &str) -> String {
    format!("https://github.com/{}", encode_url(query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_github_search_url() {
        assert_eq!(
            construct_github_search_url("hello world"),
            "https://github.com/search?q=hello%20world"
        )
    }

    #[test]
    fn test_construct_github_profile_url() {
        assert_eq!(
            construct_github_profile_url("hello world"),
            "https://github.com/hello%20world"
        )
    }

    #[test]
    fn test_construct_gihtub_url() {
        assert_eq!(
            construct_github_url("@hello world"),
            "https://github.com/hello%20world"
        );
        assert_eq!(
            construct_github_url("hello world"),
            "https://github.com/search?q=hello%20world"
        );
        assert_eq!(construct_github_url(""), "https://github.com");
    }
}
