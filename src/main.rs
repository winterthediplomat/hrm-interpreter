// values
extern crate hrm_interpreter;

use hrm_interpreter::state;
use hrm_interpreter::Operation;
use hrm_interpreter::Value;

fn main() {
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

    // create the operator and call Operator::applyTo
		for _operation in code {
			println!("applying operation {:?}", _operation);
			internal_state = internal_state.apply(_operation);
		}

    // print internal state
		println!("{:?}", internal_state);
}
