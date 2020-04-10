
Not deleted because I might need to look this up later
Edit: And I had to! , thank you file !
                                    -cube







//  testing phase rn
use serde_json;
//use crate::serde::{Serializer,Deserializer};

use std::fs::File;
use std::io::Read;

// Testing -Debug phase
// Everything is unwraped because it is for testing now
// More clarifications to be done
// more error handling to be done
pub fn loader(f:String)-> Vec<(String,i32,i32)> {

    let mut p: String = String::new();

    File::open(f).unwrap()
            .read_to_string(&mut p);

    let v : serde_json::Value = serde_json::from_str(&p).unwrap();

    println!("{:?},{:?},{:?}",v["level_name"],v["type"],v["data"][1]);

    let objects = v["data"].as_array().unwrap();

    let val : Vec<(String,i32,i32)> = objects.iter().map(|_object| {
        (_object["type"].as_str().unwrap().to_string(), _object["x"].as_i64().unwrap() as i32,_object["y"].as_i64().unwrap() as i32)
    }).collect::<Vec<(String,i32,i32)>>();

    // For tests
    println!("val is {:?}",val );

    val


}
