use std::fs::File;
use std::io::prelude::*;
use Operation;
use serde_json;

#[derive(Debug, Deserialize, Clone)]
pub struct JsonOperation {
    operation: String,
    operand: Option<String>
}

fn to_operator(json_op: JsonOperation) -> Operation {
    if json_op.operation == String::from("inbox"){ return Operation::Inbox{}; }
    else if json_op.operation == String::from("add") {
        return Operation::Add{cell: json_op.operand.unwrap().parse::<usize>().unwrap()};
    }
    else if json_op.operation == String::from("copyfrom") {
        return Operation::CopyFrom{cell: json_op.operand.unwrap().parse::<usize>().unwrap()};
    }
    else if json_op.operation == String::from("copyto") {
        return Operation::CopyTo{cell: json_op.operand.unwrap().parse::<usize>().unwrap()};
    }
    else { return Operation::Outbox{}; }
}

pub fn read_file(srcpath: String) -> Vec<Operation> {
    let mut file = File::open(srcpath).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    
    let x: Result<Vec<JsonOperation>, _> = serde_json::from_str(&contents);
    let mut res: Vec<Operation> = vec!(); 
    for json_op in x.unwrap() {
	res.push(to_operator(json_op.clone()));
    }

    return res;
}
