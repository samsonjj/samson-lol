#[macro_use]
extern crate rocket;

mod utils;

use rocket::response::Redirect;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search?<q>")]
async fn search(q: String) -> Redirect {
    let command = Command(q);
    let (action, rest) = command.get_action_and_rest();

    let redirect_url = match action {
        "gh" => utils::github::construct_github_url(rest).await,
        "tw" => utils::twitter::construct_twitter_url(rest),
        _ => utils::google::construct_google_search_url(&command.get_raw()),
    };

    Redirect::to(redirect_url)
}

use rocket::tokio::time::{sleep, Duration};

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

struct Command(String);

impl Command {
    fn get_first_space_index(&self) -> usize {
        self.0.find(" ").unwrap_or(0)
    }
    pub fn get_raw(&self) -> &str {
        &self.0
    }
    pub fn get_action(&self) -> &str {
        &self.0.split_whitespace().next().unwrap_or("")
    }
    pub fn get_args(&self) -> Vec<&str> {
        self.0.split_whitespace().skip(1).collect()
    }
    pub fn get_rest(&self) -> &str {
        let space_index = self.get_first_space_index();
        if space_index >= self.0.len() - 1 {
            ""
        } else {
            &self.0[(self.get_first_space_index() + 1)..]
        }
    }
    pub fn get_action_and_rest(&self) -> (&str, &str) {
        let space_index = self.get_first_space_index();
        let action = &self.0[0..space_index];
        let rest = if space_index >= self.0.len() - 1 {
            ""
        } else {
            &self.0[(self.get_first_space_index() + 1)..]
        };
        (action, rest)
    }
    pub fn get_action_and_args(&self) -> (&str, Vec<&str>) {
        let mut iter = self.0.split_whitespace();
        let action = iter.next().unwrap_or("");
        let rest = iter.collect();
        (action, rest)
    }
}

fn get_config() -> rocket::Config {
    rocket::Config {
        address: "0.0.0.0".parse().unwrap(),
        ..rocket::Config::default()
    }
}

#[launch]
fn rocket() -> _ {
    env_logger::init();

    let config = get_config();
    rocket::custom(&config).mount("/", routes![delay, index, search])
}
