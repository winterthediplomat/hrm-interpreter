use std::fs::File;
use std::io::prelude::*;
use serde_json;

use Operation;
use Value;
use state::InternalState;

// TODO(alfateam123): create ad-hoc type for operands in order to
// refactor `to_operator`. Fix it when addresses are introduced.
#[derive(Debug, Deserialize, Clone)]
pub struct JsonOperation {
    operation: String,
    operand: Option<String>
}

// JSON data format:
// using ::Value directly forces users to write horrible things,
// such as "input_tape": [{"Number": {"value": 3}}].
// Users can now write "input_tape: [3] and be happy.
#[derive(Deserialize, Clone)]
#[serde(untagged)]
enum JsonValue {
    Number(u32),
    Character(char)
}

#[derive(Deserialize, Clone)]
struct Config {
    input_tape: Vec<JsonValue>,
    memory: Vec<Option<JsonValue>>
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
    
    let source_code: Result<Vec<JsonOperation>, _> = serde_json::from_str(&contents);
    let mut res: Vec<Operation> = vec!(); 
    for json_op in source_code.unwrap() {
	res.push(to_operator(json_op.clone()));
    }

    return res;
}

pub fn read_config(path: String) -> InternalState  {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let input_config: Config = serde_json::from_str(&contents).unwrap();
    return InternalState{
        register: None,
        input_tape: input_config.input_tape.into_iter().map(|input| match input {
	    JsonValue::Number(num_) => Value::Number{value: num_},
	    JsonValue::Character(char_) => Value::Character{value: char_}
        }).collect(),
        output_tape: vec!(),
        instruction_counter: 0,
        memory: input_config.memory.into_iter().map(|memory_value| match memory_value {
	    Some(JsonValue::Number(num_)) => Some(Value::Number{value: num_}),
	    Some(JsonValue::Character(char_)) => Some(Value::Character{value: char_}),
	    None => None
        }).collect()
    };
}
