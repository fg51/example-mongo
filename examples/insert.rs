use anyhow::Result;

use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{options::ClientOptions, Client};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Book {
    #[serde(rename = "_id", skip_serializing)]
    id: Option<ObjectId>,
    title: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("test");
    let collection = db.collection::<Book>("books");

    let books = vec![
        Book {
            id: None,
            title: "1984".to_string(),
            author: "George Orwell".to_string(),
        },
        Book {
            id: None,
            title: "Animal Farm".to_string(),
            author: "George Orwell".to_string(),
        },
        Book {
            id: None,
            title: "The Great Gatsby".to_string(),
            author: "F. Scott Fitzgerald".to_string(),
        },
    ];

    collection.insert_many(books, None).await?;
    Ok(())

}
