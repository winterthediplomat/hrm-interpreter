use super::operators::Operator;
use state::InternalState;

pub struct CopyToOp {
	pub cell: usize
}
impl Operator for CopyToOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(&self, mut s: InternalState) -> InternalState {
		if let Some(value) = s.register {
			s.memory[self.cell] = Some(value);
		}
		else {
			panic!("register holds no value. could not copy a None value to {}", self.cell)
		}
		s
	}
}


