use csv::{Writer,Reader};
use std::sync::{Arc};
use tokio::sync::Mutex;
use std::{collections::HashMap, io::Read};
use std::error::Error;

use crate::hash::hash_256_;


pub fn load_csv_base(path : &str) -> Result</* passwrd and username*/ HashMap<String,String>, Box<dyn Error>>{
    let mut rdr = Reader::from_path(path)?;
    let mut users = HashMap::new();
    
    for res in rdr.records(){
        let record = res?;
        users.insert(record[0].to_string(), record[1].to_string());
    }
    Ok(users)
}



pub async fn check_user_csv_(user :&str, passwd : &str, csv_data : Arc<Mutex<HashMap<String,String>>>){
    let cnt_pass = hash_256_(passwd);
    let users = csv_data.lock().await;
    match users.get(user) {
        Some(hash_user_pass) if *hash_user_pass == cnt_pass => {
            println!("Auth supported for user : {}", user);
        }
        _ => {
            println!("Auth not supported for user : {}", user);
        }
    }
}



let var = String::from("houssam");
var
