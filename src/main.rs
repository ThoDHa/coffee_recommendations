use std::collections::HashMap;
mod cities;
mod reddit;

#[tokio::main]
async fn main() {
    let united_states: HashMap<String, Vec<String>> = cities::get_cities_from_file().await;
    let subreddits: [String; 1] = ["r/coffee".to_string()];

    for subreddit in subreddits.iter() {
        for state in united_states.iter() {
            for city in state.1.iter() {
                reddit::get_search(subreddit.to_string(), city.to_string(), state.0.to_string());
            }
        }
    }
}
