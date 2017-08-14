use operators::Operator;
use state::InternalState;

pub struct InboxOp {}
impl Operator for InboxOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(&self, mut s: InternalState) -> InternalState {
		match s.input_tape.pop() {
			Some(input) => {
				s.register = Some(input);
			},
			_ => {
				panic!("input is empty!");
			}
		}
		return s;
	}
}


