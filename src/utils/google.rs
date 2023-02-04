use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use rocket::http::uri::Uri;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_google_search_url(q: &str) -> String {
    let query = utf8_percent_encode(q, FRAGMENT).to_string();
    println!("query is {query}");
    format!("https://google.com/search?q={}", query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_google_search_url() {
        let fake_query = "hello";

        let actual = construct_google_search_url(fake_query);
        let expected = "https://google.com/search?q=hello";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_construct_google_search_url_with_encoding() {
        let fake_query = "hello world";

        let actual = construct_google_search_url(fake_query);
        let expected = "https://google.com/search?q=hello%20world";

        assert_eq!(actual, expected);
    }
}
