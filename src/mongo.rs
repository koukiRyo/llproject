use mongodb::{options::ClientOptions , Client};

pub struct ConnectionResult {
    pub client : mongodb::Client,
    pub client_options : ClientOptions,
}

#[tokio::main]
pub async fn connect_to_mongo() -> ConnectionResult {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.expect("connection_option error");
    let client = Client::with_options(client_options.clone()).expect("connection error");

    ConnectionResult{client , client_options}
}


