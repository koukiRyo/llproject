use std::env;
pub mod mongo;
pub mod words;
use mongo::ConnectionResult; 
use crate::words::Words;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Path to the program: {}",args[0]);

    if args.len() > 1 {
        for argument in &args[1..] {
            println!("{}",argument);
        }
    }

    let ConnectionResult{ client: client , client_options: mut client_options } = mongo::connect_to_mongo();

    let db = client.database("llproject");
    let collection: mongodb::Collection<Words> = db.collection("word");

    
}


