use crate::models::models::Customer;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::fs::File;
use serde_json::from_reader;

// in memory data store of customer data
pub type Db = Arc<Mutex<Vec<Customer>>>;

// initializes data store
// returns the db type defined above that either contains customer data
// or is empty
pub fn init_db() -> Db {
    let file = File::open("./data/customers.json");
    match file {
        Ok(json) => {
            let customers = from_reader(json).unwrap();
            Arc::new(Mutex::new(customers))
        },
        Err(_) => {
            Arc::new(Mutex::new(Vec::new()))
        }
    }
}
