use operators::Operator;
use state::InternalState;

pub struct OutboxOp {
}
impl Operator for OutboxOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(&self, mut s: InternalState) -> Result<InternalState, String> {
		if let Some(value) = s.register {
			s.output_tape.push(value);
			Ok(s)
		}
		else {
			Err(format!("you cannot put nothing in the output queue!"))
		}
	}
}


