use operators::Operator;
use state::InternalState;
use Location;
use memory;

pub struct CopyToOp {
	pub cell: Location
}
impl Operator for CopyToOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(&self, s: &mut InternalState) -> Result<(), String> {
		let memory_position = memory::extract_memory_position(self.cell, &s);
		if let Err(error) = memory_position {
			return Err(memory::explain(error));
		}

		let cell = memory_position.unwrap();
		if let Some(value) = s.register {
			s.memory[cell] = Some(value);
			Ok(())
		}
		else {
			Err(format!("register holds no value. could not copy a None value to {}", cell))
		}
	}
}


