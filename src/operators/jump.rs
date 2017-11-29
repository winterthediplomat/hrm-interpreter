use operators::Operator;
use state;

#[derive(Debug, Clone, Copy)]
pub struct LabelOp;

impl Operator for LabelOp {
    fn changes_instruction_counter(&self) -> bool { false }

    fn apply_to(&self, s: &mut state::InternalState) -> Result<(), String> {
        s.instruction_counter += 1;
	Ok(())
    }
}
