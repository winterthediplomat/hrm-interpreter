use operators::Operator;
use state::InternalState;

pub struct OutboxOp {
}
impl Operator for OutboxOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(&self, s: &mut InternalState) -> Result<(), String> {
		if let Some(value) = s.register {
			s.output_tape.push(value);
			Ok(())
		}
		else {
			Err(format!("you cannot put nothing in the output queue!"))
		}
	}
}


