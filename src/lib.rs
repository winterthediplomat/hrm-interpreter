#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Debug, Clone, Copy)]
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
	CopyFrom{cell: Location},
	CopyTo{cell: Location},
	Label,
	Jump{next_operation: usize},
	JumpEqualsZero{next_operation: usize}
}

pub mod json;
pub mod operators;
pub mod state;

pub struct CodeIterator<'a> {
	pub state: &'a mut state::InternalState,
	pub operations: Vec<Operation>,
	has_errored: bool
}

impl<'a> CodeIterator<'a> {
	pub fn new(_state: &'a mut state::InternalState, _operations: Vec<Operation>) -> Self {
		CodeIterator{state: _state, operations: _operations, has_errored: false}
	}
}

impl<'a> Iterator for CodeIterator<'a> {
	type Item = Result<(), String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.has_errored {
			None
		}
		else if self.state.instruction_counter < self.operations.len() {
			let _operation = self.operations[self.state.instruction_counter];
			println!("applying operation {:?}", _operation);

			let result = self.state.apply(_operation);

			if result.is_err() {
				if let Operation::Inbox =  _operation {
					self.has_errored = false;
					return None;
				}
				else {
					self.has_errored = true;
				}
			}

			Some(result)
		}
		else {
			None
		}
	}
}
