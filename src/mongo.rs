use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> mongodb::Client {
        dotenv().ok();
        let mongo_url = env::var("MONGO_URL").expect("MONGODB_ADDRESS missing");
        let mut client_options = ClientOptions::parse(mongo_url).await.unwrap();
        client_options.app_name = Some("teleport".to_string());

        let client = Client::with_options(client_options).unwrap();

        client
    }
}
