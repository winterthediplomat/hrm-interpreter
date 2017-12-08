use operators::Operator;
use state::InternalState;
use Location;
use Value;

pub struct CopyToOp {
	pub cell: Location
}
impl Operator for CopyToOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(&self, s: &mut InternalState) -> Result<(), String> {
		let memory_position = match self.cell {
			Location::Cell(mempos) => Ok(mempos),
			Location::Address(addressed_cell) => match s.memory[addressed_cell] {
				None => Err(format!("cannot read the value of the addressed cell")),
				Some(Value::Character{value: _}) => Err(format!("char is not a valid address")),
				Some(Value::Number{value: mempos}) => Ok(mempos as usize)
			}
		};
		if let Err(error_reason) = memory_position {
			return Err(error_reason);
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


