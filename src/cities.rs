use reqwest;
use std::fs::File;
use std::collections::HashMap;

pub async fn get_cities() -> HashMap<String, Vec<String>> {
    let result = reqwest::get("https://gist.githubusercontent.com/ahmu83/38865147cf3727d221941a2ef8c22a77/raw/c647f74643c0b3f8407c28ddbb599e9f594365ca/US_States_and_Cities.json")
        .await
        .unwrap()
        .text()
        .await;
    let value = result.expect("Cannot get the Cities list.");
    let states: HashMap<String,Vec<String>> = serde_json::from_str(&value).expect("Failed to convert string to json object.");
    return states;
}

pub async fn get_cities_from_file() -> HashMap<String, Vec<String>> {
    let value = File::open("cities.json").expect("Failed to read file");
    let states: HashMap<String,Vec<String>> = serde_json::from_reader(&value).expect("Failed to convert string to json object.");
    return states;
}

pub fn print_state(states: &HashMap<String, Vec<String>>){
   for state in states.iter() {
        println!("{:#?}", state)
    }
}
