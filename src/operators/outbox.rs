use operators::Operator;
use state::InternalState;

pub struct OutboxOp {
}
impl Operator for OutboxOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(&self, mut s: InternalState) -> InternalState {
		if let Some(value) = s.register {
			s.output_tape.push(value);
		}
		else {
			panic!("you cannot put nothing in the output queue!")
		}
		s
	}
}


