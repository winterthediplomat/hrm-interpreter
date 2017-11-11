#[macro_use]
extern crate serde_derive;

extern crate hrm_interpreter;
extern crate serde;
extern crate serde_json;
use hrm_interpreter::state;
use hrm_interpreter::Operation;
use hrm_interpreter::Value;

//use serde_json::{Value, Error};
use std::fs::File;
use std::io::prelude::*;
use serde_json::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct JsonOperation {
    operation: String,
    operand: Option<String>
}

fn readIt() {
    let mut file = File::open("/home/winterthediplomat/projects/hrm-compiler/examples/script10.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    
    let x: Result<Vec<JsonOperation>, _> = serde_json::from_str(&contents);
    println!("{:?}", x);
}

fn main() {
    readIt();

    // create the state to be modified
    let mut internal_state = state::InternalState{
			register: None,
			input_tape: vec!(
				Value::Number{value: 8}
			),
			output_tape: vec!(),
			instruction_counter: 0,
			memory: vec!(None, None, None, None, None)
		};

    let code : Vec<Operation> = vec!(
			Operation::Inbox{},
			Operation::CopyTo{cell: 0},
			Operation::Add{cell: 0},
			Operation::CopyTo{cell: 1},
			Operation::Add{cell: 1},
			Operation::CopyTo{cell: 2},
			Operation::Add{cell: 2},
			Operation::Outbox{}
		);

		loop {
			if internal_state.instruction_counter < code.len() {
				let _operation = code[internal_state.instruction_counter];
				println!("applying operation {:?}", _operation);

				let result = internal_state.apply(_operation);
				if result.is_err() {
					let reason = result.err().unwrap();
					println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
					println!("Error: {}", reason);
					println!("Dumping current internal state:");
					println!("{:?}", internal_state);
					break;
				}
			}
			else {
				break;
			}
		}

		// print internal state
		println!("{:?}", internal_state);
}
