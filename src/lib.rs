#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use json::dump_state;

#[derive(Serialize, Debug, Clone, Copy)]
pub enum Value {
	Number{value: i32},
	Character{value: char}
}

#[derive(Debug, Clone, Copy)]
pub enum Location {
	Cell(usize),
	Address(usize)
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
	Inbox,
	Outbox,
	Add{cell: Location},
	Sub{cell: Location},
	CopyFrom{cell: Location},
	CopyTo{cell: Location},
	Label,
	Jump{next_operation: usize},
	JumpEqualsZero{next_operation: usize},
	JumpNegative{next_operation: usize},
	BumpPlus{cell: Location},
	BumpMinus{cell: Location},
}

pub mod json;
pub mod memory;
pub mod operators;
pub mod state;

pub struct CodeIterator<'a> {
	pub state: &'a mut state::InternalState,
	pub operations: Vec<Operation>,
	has_errored: bool,
	dump_file_path: &'a str
}

impl<'a> CodeIterator<'a> {
	pub fn new(_state: &'a mut state::InternalState, _operations: Vec<Operation>, dump_file_path: &'a str) -> Self {
		CodeIterator{state: _state, operations: _operations, has_errored: false, dump_file_path}
	}
}

impl<'a> Iterator for CodeIterator<'a> {
	type Item = Result<state::InternalState, String>;

	fn next(&mut self) -> Option<Self::Item> {
		let srcpath = self.dump_file_path;
		if self.has_errored {
			None
		}
		else if self.state.instruction_counter < self.operations.len() {
			let _operation = self.operations[self.state.instruction_counter];

			let result = self.state.apply(_operation);

			if result.is_err() {
				if let Operation::Inbox =  _operation {
					self.has_errored = false;
					dump_state(&self.state, srcpath, &result.err().unwrap());
					return None;
				}
				else {
					dump_state(&self.state, srcpath, &result.err().unwrap());
					self.has_errored = true;
				}
			}
			dump_state(&self.state, srcpath, &String::new());

			Some(Ok(self.state.clone()))
		}
		else {
			dump_state(&self.state, srcpath, &String::new());
			None
		}
	}
}
