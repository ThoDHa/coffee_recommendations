mod cities;

#[tokio::main]
async fn main() {
    cities::get_cities().await
}

