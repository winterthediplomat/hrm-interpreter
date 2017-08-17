use operators::Operator;
use state::InternalState;

pub struct CopyToOp {
	pub cell: usize
}
impl Operator for CopyToOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(&self, s: &mut InternalState) -> Result<(), String> {
		if let Some(value) = s.register {
			s.memory[self.cell] = Some(value);
			Ok(())
		}
		else {
			Err(format!("register holds no value. could not copy a None value to {}", self.cell))
		}
	}
}


