use operators::Operator;
use state::InternalState;

pub struct InboxOp {}
impl Operator for InboxOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(&self, s: &mut InternalState) -> Result<(), String> {
		match s.input_tape.pop() {
			Some(input) => {
				s.register = Some(input);
				Ok(())
			},
			_ => {
				Err(String::from("The inbox is empty, you cannot pick a new value from it!"))
			}
		}
	}
}


