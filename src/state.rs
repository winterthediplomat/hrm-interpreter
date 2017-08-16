use Value;
use Operation;
use operators;
use operators::Operator;

#[derive(Debug)]
pub struct InternalState {
  pub register: Option<Value>,
	pub input_tape: Vec<Value>,
	pub output_tape: Vec<Value>,
	pub memory: Vec<Option<Value>>,
	pub instruction_counter: usize
}


macro_rules! apply_operation {
	($self: ident, $operator:expr) => ({
		let op = $operator;
		$self = op.apply_to($self);
		if !op.changes_instruction_counter() {
			$self.instruction_counter += 1;
		}
	})
}

impl InternalState {
	pub fn apply(mut self, op: Operation) -> InternalState {
		match op {
			Operation::Add{cell: _cell} => {
				apply_operation!(self, operators::add::AddOp{cell: _cell});
			},
			Operation::Inbox => {
				apply_operation!(self, operators::inbox::InboxOp{});
			},
			Operation::Outbox => {
				apply_operation!(self, operators::outbox::OutboxOp{});
			},
			Operation::CopyFrom{cell: _cell} => {
				apply_operation!(self, operators::copyfrom::CopyFromOp{cell: _cell});
			}
			Operation::CopyTo{cell: _cell} => {
				apply_operation!(self, operators::copyto::CopyToOp{cell: _cell});
			}
		};
		self
	}
}
