use operators::Operator;
use state;
use Value;

#[derive(Debug, Clone, Copy)]
pub struct LabelOp;

impl Operator for LabelOp {
    fn changes_instruction_counter(&self) -> bool { false }

    fn apply_to(&self, _: &mut state::InternalState) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JumpOp {
    pub next_operation: usize
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
    pub next_operation: usize
}

impl Operator for JumpEqualsZeroOp {
    fn changes_instruction_counter(&self) -> bool { true }

    fn apply_to(&self, s: &mut state::InternalState) -> Result<(), String> {
        match s.register {
            None => Err(String::from("register holds no value! cannot compare it to zero!")),
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

#[derive(Debug, Clone, Copy)]
pub struct JumpNegativeOp {
    pub next_operation: usize
}

impl Operator for JumpNegativeOp {
    fn changes_instruction_counter(&self) -> bool { true }

    fn apply_to(&self, s: &mut state::InternalState) -> Result<(), String> {
        match s.register {
            None => Err(String::from("register holds no value! cannot see if it is negative!")),
            Some(Value::Character{value: _}) => Err(String::from("cannot compare a character to zero!")),
            Some(Value::Number{value: _v}) if _v < 0 => {
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
    use operators::jump::LabelOp;
    use operators::jump::JumpEqualsZeroOp;
    use operators::jump::JumpNegativeOp;
    use state::InternalState;
    use Value;

    #[test]
    fn label_does_not_change_instruction_counter() {
        let mut _state = InternalState::new(Some(Value::Number{value: 0}), 0);
        let _op = LabelOp;

        _op.apply_to(&mut _state).unwrap();

        assert!(_state.instruction_counter == 0);
    }

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

    #[test]
    fn jneg_register_is_zero() {
        let mut _state = InternalState::new(Some(Value::Number{value: 0}), 0);
        let _op = JumpNegativeOp{next_operation: 15};

        _op.apply_to(&mut _state).unwrap();

        assert!(_state.instruction_counter == 1);
    }

    #[test]
    fn jneg_register_is_negative() {
        let mut _state = InternalState::new(Some(Value::Number{value: -2}), 0);
        let _op = JumpNegativeOp{next_operation: 15};

        _op.apply_to(&mut _state).unwrap();

        assert!(_state.instruction_counter == 15);
    }

    #[test]
    fn jneg_register_is_positive() {
        let mut _state = InternalState::new(Some(Value::Number{value: 5}), 0);
        let _op = JumpEqualsZeroOp{next_operation: 15};

        _op.apply_to(&mut _state).unwrap();

        assert!(_state.instruction_counter == 1);
    }

    #[test]
    fn jneg_no_register_value() {
        let mut _state = InternalState::new(None, 0);
        let _op = JumpEqualsZeroOp{next_operation: 15};

        assert!(_op.apply_to(&mut _state).is_err());
    }
}
