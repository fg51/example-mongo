use anyhow::Result;

use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{options::ClientOptions, Client};
use mongodb::options::FindOptions;
use futures::stream::TryStreamExt;

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

    let filter = doc! {"author": "George Orwell"};
    let find_options = FindOptions::builder().sort(doc! {"title": -1}).build();

    let mut cursor = collection.find(filter, find_options).await?;

    while let Some(book) = cursor.try_next().await? {
        println!("title: {}", book.title);
    }
    Ok(())
}

