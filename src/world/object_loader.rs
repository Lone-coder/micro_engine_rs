//  testing phase rn
use super::super::serde_json;
//use super::super::serde::{Serializer,Deserializer};
use super::super::game_object;
use std::fs::File;
use std::io::Read;




// Testing -Debug phase
// Everything is unwraped because it is for testing now
// More clarifications sto be done
// more error handling to be done
pub fn loader()->Vec<(String,i32,i32)>{

    let mut p:String=String::new();

    File::open("map/map_data.json").unwrap()
                                   .read_to_string(&mut p);

    let v:serde_json::Value=serde_json::from_str(&p).unwrap();

    println!("{:?},{:?},{:?}",v["level_name"],v["type"],v["data"][1]);

    let objects=v["data"].as_array().unwrap();

    let val:Vec<(String,i32,i32)>=objects.iter().map(|_object|{
        (_object["type"].as_str().unwrap().to_string(),_object["x"].as_i64().unwrap() as i32,_object["y"].as_i64().unwrap() as i32)
    }).collect::<Vec<(String,i32,i32)>>();

    println!("val is {:?}",val );

    val


}
