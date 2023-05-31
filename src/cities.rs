use reqwest;
use std::fs;
use std::collections::HashMap;

pub async fn get_cities() {
    let result = reqwest::get("https://gist.githubusercontent.com/ahmu83/38865147cf3727d221941a2ef8c22a77/raw/c647f74643c0b3f8407c28ddbb599e9f594365ca/US_States_and_Cities.json")
        .await
        .unwrap()
        .text()
        .await;
    let value = result.expect("Cannot get the Cities list.");
    let states: HashMap<String,Vec<String>> = serde_json::from_str(&value).expect("Failed to convert string to json object.");
    print_state(&states);
}

pub async fn get_cities_from_file() {
    let value = fs::read_to_string("cities.json").expect("Failed to read file");
    let states: HashMap<String,Vec<String>> = serde_json::from_str(&value).expect("Failed to convert string to json object.");
    print_state(&states);

}

 fn print_state(states: &HashMap<String, Vec<String>>){
   for state in states.iter() {
        println!("{:#?}", state)
    }
}
