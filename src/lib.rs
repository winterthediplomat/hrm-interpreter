
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

pub mod operators {
	use state;
	// define the Operator trait: every Operator
	// can modify the internal state
	pub trait Operator {
	// rust compiler issue 35203.
		fn apply_to(self, /*mut*/ s: state::InternalState) -> state::InternalState;
		fn changes_instruction_counter(&self) -> bool;
	}

	pub mod add;
	pub mod inbox;
	pub mod outbox;
	pub mod copyto;
}

pub mod state {
	use Value;
	use Operation;
	use operators::*;

#[derive(Debug)]
pub struct InternalState {
  pub register: Option<Value>,
	pub input_tape: Vec<Value>,
	pub output_tape: Vec<Value>,
	pub memory: Vec<Option<Value>>,
	pub instruction_counter: u32
}


macro_rules! apply_operation {
	($self: ident, $operator:expr) => ({
		$self = $operator.apply_to($self);
		if !$operator.changes_instruction_counter() {
			$self.instruction_counter += 1;
		}
	})
}

impl InternalState {
	pub fn apply(mut self, op: Operation) -> InternalState {
		match op {
			Operation::Add{cell: _cell} => {
				apply_operation!(self, add::AddOp{cell: _cell});
			},
			Operation::Inbox => {
				apply_operation!(self, inbox::InboxOp{});
			},
			Operation::Outbox => {
				apply_operation!(self, outbox::OutboxOp{});
			},
			Operation::CopyTo{cell: _cell} => {
				apply_operation!(self, copyto::CopyToOp{cell: _cell});
			}
		};
		self
	}
}


}
