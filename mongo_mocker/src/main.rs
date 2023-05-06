use clap::Parser;
use mongodb::{Client, options::{ClientOptions, EstimatedDocumentCountOptions} };
use std::error::Error;
use serde::{Deserialize, Serialize};
use tokio;

// safe ignore error since it compiles normally (proc-macro not expanded)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  mongo_uri: String,
  target_collection: String,
  collection_size: u64
}


#[derive(Clone, Debug, Deserialize, Serialize)]
struct TestDocument {
  title: String
}

async fn run_db_mock(args: Args) -> Result<(), Box<dyn Error>> {
   let Args { mongo_uri, target_collection, collection_size } = args; 
   println!("parsed arguements {}, {}, {}", mongo_uri, target_collection, collection_size);
   // A Client is needed to connect to MongoDB:
   let options = ClientOptions::parse(&mongo_uri).await?;
   let client = Client::with_options(options)?;

    // 2. create dummy docs with random words 
    // 3. insert until set size is reached 
   let db = client.database("test");
   let col = db.collection::<TestDocument>(&target_collection);
   let count = col.estimated_document_count(EstimatedDocumentCountOptions::default()).await?;
   println!("estimated number of documents {}", count);
   // TODO: 1. check current test db cols 
   Ok(())
}

fn main() {
    let args = Args::parse();
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let res = run_db_mock(args).await;
            res.unwrap_or_default()
        });
}

