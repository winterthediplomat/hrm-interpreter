extern crate hrm_interpreter;
use hrm_interpreter::state;
use hrm_interpreter::Value;
use hrm_interpreter::json::read_file;

fn main() {
    let code = read_file();

    // create the state to be modified
    let mut internal_state = state::InternalState{
			register: None,
			input_tape: vec!(
				Value::Number{value: 1}
			),
			output_tape: vec!(),
			instruction_counter: 0,
			memory: vec!(None, None, None, None, None)
		};

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
