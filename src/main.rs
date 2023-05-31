use std::collections::HashMap;

use cities::print_state;
mod cities;

#[tokio::main]
async fn main() {
    let united_states: HashMap<String, Vec<String>> = cities::get_cities().await;
    print_state(&united_states);
}
