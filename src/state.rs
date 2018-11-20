use Value;
use Operation;
use operators;
use operators::Operator;

#[derive(Serialize, Debug, Clone)]
pub struct InternalState {
  pub register: Option<Value>,
	pub input_tape: Vec<Value>,
	pub output_tape: Vec<Value>,
	pub memory: Vec<Option<Value>>,
	pub instruction_counter: usize,
	_executed_instructions: u32
}


macro_rules! apply_operation {
	($self: ident, $operator:expr) => ({
		let op = $operator;
    match op.apply_to($self) {
			Ok(()) => {
				if !op.changes_instruction_counter() {
					$self.instruction_counter += 1;
				}
				$self.increase_executed_instructions();
				Ok(())
			},
			Err(reason) => {
				Err(reason)
			}
		}
	})
}

impl InternalState {
	pub fn new(register: Option<Value>, counter: usize) -> InternalState {
		InternalState {
			register,
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(),
			instruction_counter: counter,
			_executed_instructions: 0
		}
	}

	pub fn with_input_tape(mut self, new_input_tape: Vec<Value>) -> Self {
		self.input_tape = new_input_tape;
		self
	}

	pub fn with_memory(mut self, new_memory: Vec<Option<Value>>) -> Self {
		self.memory = new_memory;
		self
	}

    pub fn executed_instructions(&self) -> u32 {
		self._executed_instructions
	}

	pub fn increase_executed_instructions(&mut self) {
		self._executed_instructions += 1;
	}

	pub fn apply(&mut self, op: Operation) -> Result<(), String> {
		match op {
			Operation::Add{cell: _cell} => {
				apply_operation!(self, operators::add::AddOp{cell: _cell})
			},
			Operation::Sub{cell: _cell} => {
				apply_operation!(self, operators::sub::SubOp{cell: _cell})
			},
			Operation::Inbox => {
				apply_operation!(self, operators::inbox::InboxOp{})
			},
			Operation::Outbox => {
				apply_operation!(self, operators::outbox::OutboxOp{})
			},
			Operation::CopyFrom{cell: _cell} => {
				apply_operation!(self, operators::copyfrom::CopyFromOp{cell: _cell})
			},
			Operation::CopyTo{cell: _cell} => {
				apply_operation!(self, operators::copyto::CopyToOp{cell: _cell})
			},
			Operation::Label => {
				apply_operation!(self, operators::jump::LabelOp)
			},
			Operation::Jump{next_operation: _next_op} => {
				apply_operation!(self, operators::jump::JumpOp{next_operation: _next_op})
			},
			Operation::JumpEqualsZero{next_operation: _next_op} => {
				apply_operation!(self, operators::jump::JumpEqualsZeroOp{next_operation: _next_op})
			}
			Operation::JumpNegative{next_operation: _next_op} => {
				apply_operation!(self, operators::jump::JumpNegativeOp{next_operation: _next_op})
			}
			Operation::BumpPlus{cell: _cell} => {
				apply_operation!(self, operators::bump::BumpPlusOp{cell: _cell})
			}
			Operation::BumpMinus{cell: _cell} => {
				apply_operation!(self, operators::bump::BumpMinusOp{cell: _cell})
			}
		}
	}
}

#[cfg(test)]
mod test {
	use state::InternalState;
	use Value;
	use Operation;

	#[test]
	fn executed_counter_at_start() {
		let state = InternalState::new(None, 0);

		assert_eq!(state.executed_instructions(), 0);
	}

	#[test]
	fn executed_counter_is_increased() {
		let mut state = InternalState::new(None, 0)
			.with_input_tape(vec!(Value::Number{value: 1}));

		let _ = state.apply(Operation::Inbox);

		assert_eq!(state.executed_instructions(), 1);
	}
}