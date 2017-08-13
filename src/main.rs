// values
#[derive(Debug, Clone, Copy)]
struct Value {
	value: u32,
	is_char: bool
}
fn create_number(number: u32) -> Value {
	Value{value: number, is_char: false}
}
fn create_character(character: char) -> Value {
	Value{value: character.to_digit(36).unwrap() - 10, is_char: true}
}

// structure to be modified by `Operator`s
#[derive(Debug)]
struct InternalState {
  register: Option<Value>,
	input_tape: Vec<Value>,
	output_tape: Vec<Value>,
	memory: Vec<Option<Value>>,
	instruction_counter: u32
}

// define the Operator trait: every Operator
// can modify the internal state
trait Operator: Sized {
	// rust compiler issue 35203.
  fn apply_to(self, /*mut*/ s: InternalState) -> InternalState;
	fn changes_instruction_counter(&self) -> bool;
}
#[derive(Debug)]
enum Operation {
	InboxOp,
	OutboxOp,
	AddOp{cell: usize},
	CopyToOp{cell: usize}
}

impl InternalState {
	fn apply(mut self, op: Operation) -> InternalState {
		match op {
			Operation::AddOp{cell: _cell} => {
				let op = AddOp{cell: _cell};
				// why is it moved?
				let x = op.changes_instruction_counter();
				self = op.apply_to(self);
				if ! x {
					self.instruction_counter += 1;
				}
			},
			Operation::InboxOp => {
				let op = InboxOp{};
				let x = op.changes_instruction_counter();
				self = op.apply_to(self);
				if ! x {
					self.instruction_counter += 1;
				}
			},
			Operation::OutboxOp => {
				let op = OutboxOp{};
				let x = op.changes_instruction_counter();
				self = op.apply_to(self);
				if ! x {
					self.instruction_counter += 1;
				}
			},
			Operation::CopyToOp{cell: _cell} => {
				let op = CopyToOp{cell: _cell};
				let x = op.changes_instruction_counter();
				self = op.apply_to(self);
				if ! x {
					self.instruction_counter += 1;
				}
			}
		};
		self
	}
}

// implement an operator
struct AddOp {
	cell: usize
}
impl Operator for AddOp {
  fn changes_instruction_counter(&self) -> bool {
		false
	}

  fn apply_to(self, mut s: InternalState) -> InternalState {
		let x = s.memory[self.cell].clone();
		match x {
			Some(ref v) => {
				match s.register {
					Some(old_register) => {
						s.register = Some(create_number(old_register.value + v.value))
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


struct InboxOp {}
impl Operator for InboxOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(self, mut s: InternalState) -> InternalState {
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

struct CopyToOp {
	cell: usize
}
impl Operator for CopyToOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(self, mut s: InternalState) -> InternalState {
		if let Some(value) = s.register {
			s.memory[self.cell] = Some(value);
		}
		else {
			panic!("register holds no value. could not copy a None value to {}", self.cell)
		}
		s
	}
}

struct OutboxOp {
}
impl Operator for OutboxOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(self, mut s: InternalState) -> InternalState {
		if let Some(value) = s.register {
			s.output_tape.push(value);
		}
		else {
			panic!("you cannot put nothing in the output queue!")
		}
		s
	}
}

fn main() {
    // create the state to be modified
    let mut internal_state = InternalState{
			register: None,
			input_tape: vec!(
				create_number(8)
			),
			output_tape: vec!(),
			instruction_counter: 0,
			memory: vec!(None, None, None, None, None)
		};

    let code : Vec<Operation> = vec!(
			Operation::InboxOp{},
			Operation::CopyToOp{cell: 0},
			Operation::AddOp{cell: 0},
			Operation::CopyToOp{cell: 1},
			Operation::AddOp{cell: 1},
			Operation::CopyToOp{cell: 2},
			Operation::AddOp{cell: 2},
			Operation::OutboxOp{}
		);

    // create the operator and call Operator::applyTo
		for _operation in code {
			println!("applying operation {:?}", _operation);
			internal_state = internal_state.apply(_operation);
		}

    // print internal state
//    println!("{:?}", internal_state.register);
//		println!("{:?}", internal_state.memory);
		println!("{:?}", internal_state);
}
