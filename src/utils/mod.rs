use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest::StatusCode;

pub fn get_command_from_query_string(query_string: &str) -> &str {
    query_string.split_whitespace().next().unwrap_or("")
}

pub mod github;
pub mod google;
pub mod twitter;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn encode_url(s: &str) -> String {
    utf8_percent_encode(s, FRAGMENT).to_string()
}

use log::{info};

pub async fn ping_url(url: &str) -> Result<(), StatusCode> {
    info!("pinging {}", url);
    
    let client = reqwest::Client::new();
    let response = client.head(url).send().await.unwrap();
    match response.status() {
        StatusCode::OK => {
            Ok(())
        },
        x => {
            Err(x)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
        let actual = get_command_from_query_string("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_command_from_query_string_whitespace() {
        let actual = get_command_from_query_string("tw @fbOpenSource");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
}
