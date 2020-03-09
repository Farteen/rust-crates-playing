use mongodb::{Client, options::ClientOptions, Cursor};

extern crate bson;
extern crate serde;
use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct PhotoItem {
    #[serde(rename="_id")]
    pub id: bson::oid::ObjectId,
    #[serde(rename="following")]
    pub following_count: i32,
}


// fn handle_cursor(cursor: Cursor) {
//     for item in cursor {
//         let unwrapped_item = item.unwrap();
//         let photo_item: PhotoItem = bson::from_bson(bson::Bson::Document(item));
//         println!("{:?}", photo_item);
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;

// Manually set an option.
    client_options.app_name = Some("My App".to_string());

// Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

// List the names of the databases in that deployment.
    for db_name in client.list_database_names(None)? {
        println!("{}", db_name);
    Ok(())
}
