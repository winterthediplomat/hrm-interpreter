use operators::Operator;
use state;
use Value;

#[derive(Debug, Clone, Copy)]
pub struct LabelOp;

impl Operator for LabelOp {
    fn changes_instruction_counter(&self) -> bool { false }

    fn apply_to(&self, s: &mut state::InternalState) -> Result<(), String> {
        s.instruction_counter += 1;
	Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JumpOp {
    next_operation: usize
}

impl Operator for JumpOp {
    fn changes_instruction_counter(&self) -> bool { true }

    fn apply_to(&self, s: &mut state::InternalState) -> Result<(), String> {
        s.instruction_counter = self.next_operation;
	Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JumpEqualsZeroOp {
    next_operation: usize
}

impl Operator for JumpEqualsZeroOp {
    fn changes_instruction_counter(&self) -> bool { true }

    fn apply_to(&self, s: &mut state::InternalState) -> Result<(), String> {
        match s.register {
	    None => {
	        Err(String::from("register holds no value! cannot compare it to zero!"))
	    }
	    Some(Value::Number{value: 0}) => {
	        s.instruction_counter = self.next_operation;
		Ok(())
	    },
	    _ => {
	        s.instruction_counter += 1;
		Ok(())
	    }
	}
    }
}

#[cfg(test)]
mod test {
    use operators::Operator;
    use operators::jump::JumpEqualsZeroOp;
    use state::InternalState;
    use Value;

    #[test]
    fn jez_register_is_zero() {
        let mut _state = InternalState::new(Some(Value::Number{value: 0}), 0);
	let _op = JumpEqualsZeroOp{next_operation: 15};

        _op.apply_to(&mut _state).unwrap();

	assert!(_state.instruction_counter == 15);
    }

    #[test]
    fn jez_register_not_zero() {
        let mut _state = InternalState::new(Some(Value::Number{value: 5}), 0);
	let _op = JumpEqualsZeroOp{next_operation: 15};

        _op.apply_to(&mut _state).unwrap();

	assert!(_state.instruction_counter == 1);
    }

    #[test]
    fn jez_no_register_value() {
        let mut _state = InternalState::new(None, 0);
	let _op = JumpEqualsZeroOp{next_operation: 15};

        assert!(_op.apply_to(&mut _state).is_err());
    }
}
