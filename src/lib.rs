#[derive(Debug, Clone, Copy)]
pub enum Value {
	Number{value: u32},
	Character{value: char}
}

#[derive(Debug)]
pub enum Operation {
	Inbox,
	Outbox,
	Add{cell: usize},
	CopyTo{cell: usize}
}

pub mod operators;
pub mod state;
