use std::fs::File;
use std::io::prelude::*;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct JsonOperation {
    operation: String,
    operand: Option<String>
}

pub fn read_file() {
    let mut file = File::open("/home/winterthediplomat/projects/hrm-compiler/examples/script10.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    
    let x: Result<Vec<JsonOperation>, _> = serde_json::from_str(&contents);
    println!("{:?}", x);
}
