use super::encode_url;

pub fn construct_twitter_url(rest: &str) -> String {
    if rest.len() == 0 {
        return String::from("https://twitter.com");
    }
    match &rest[0..1] {
        "@" => construct_twitter_profile_url(&rest[1..]),
        _ => construct_twitter_search_url(rest),
    }
}

pub fn construct_twitter_profile_url(profile: &str) -> String {
    format!("https://twitter.com/{}", encode_url(profile))
}

pub fn construct_twitter_search_url(query: &str) -> String {
    format!("https://twitter.com/search?q={}", encode_url(query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_twitter_search_url() {
        assert_eq!(
            construct_twitter_search_url("hello world"),
            "https://twitter.com/search?q=hello%20world"
        )
    }

    #[test]
    fn test_construct_twitter_profile_url() {
        assert_eq!(
            construct_twitter_profile_url("hello world"),
            "https://twitter.com/hello%20world"
        )
    }

    #[test]
    fn test_construct_twitter_url() {
        assert_eq!(
            construct_twitter_url("@hello world"),
            "https://twitter.com/hello%20world"
        );
        assert_eq!(
            construct_twitter_url("hello world"),
            "https://twitter.com/search?q=hello%20world"
        );
        assert_eq!(construct_twitter_url(""), "https://twitter.com");
    }
}
