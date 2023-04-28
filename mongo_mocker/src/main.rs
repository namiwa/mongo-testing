use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let client_uri = "mongodb://localhost:27017"; 
   // A Client is needed to connect to MongoDB:
   let options = ClientOptions::parse(&client_uri).await?;
   let client = Client::with_options(options)?;

   // Print the databases in our MongoDB cluster:
   println!("Databases:");
   for name in client.list_database_names(None, None).await? {
      println!("- {}", name);
   }

   Ok(())
}

