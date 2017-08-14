use operators::Operator;
use Value;
use state;

// structure to be modified by `Operator`s
// implement an operator
pub struct AddOp {
	pub cell: usize
}
impl Operator for AddOp {
  fn changes_instruction_counter(&self) -> bool {
		false
	}

  fn apply_to(&self, mut s: state::InternalState) -> state::InternalState {
		let x = s.memory[self.cell].clone();
		match x {
			Some(ref v) => {
				match s.register {
					Some(old_register) => {
						let value_to_add = match v {
							&Value::Number{value: _v} => _v,
							&Value::Character{value: _} => panic!("argh")
						};
						let old_register_value = match old_register {
							Value::Number{value: _v} => _v,
							Value::Character{value: _} => panic!("argh2")
						};
						s.register = Some(Value::Number{value: old_register_value + value_to_add});
					}
					_ => {
						panic!("No value in register Employee, cannot add.");
					}
				}
			}
			_ => {
				panic!("No value at cell {}", self.cell);
			}
		}

		return s;
  }
}


