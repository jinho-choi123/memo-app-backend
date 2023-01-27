use mongodb::{Client, options::ClientOptions};
use dotenv::dotenv;
use std::env;

pub async fn connect() -> Client {
    let mongoUri = env::var("MONGO_URL").unwrap();
    let clientOptions = ClientOptions::parse(mongoUri).await.unwrap();

    let client = Client::with_options(clientOptions).unwrap();

    return client
}

#[cfg(test)]
mod tests {
    use mongodb::{Client, options::ClientOptions};
    use dotenv::dotenv;
    use crate::utils::check_dotenv::check_dotenv;
    use std::env;

    #[tokio::test()]
    async fn test_db_connection() {
        check_dotenv();
        let mongoUri = env::var("MONGO_URL").unwrap();
        let clientOptions = ClientOptions::parse(mongoUri).await.unwrap();

        let client = Client::with_options(clientOptions).unwrap();

        client.database("memo");
        return ()
    }
}