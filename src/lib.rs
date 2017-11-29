#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Debug, Clone, Copy)]
pub enum Value {
	Number{value: u32},
	Character{value: char}
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
	Inbox,
	Outbox,
	Add{cell: usize},
	CopyFrom{cell: usize},
	CopyTo{cell: usize},
	Label
}

pub mod json;
pub mod operators;
pub mod state;
