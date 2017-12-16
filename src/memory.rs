use Location;
use Value;
use state::InternalState;

#[derive(Debug)]
pub enum Error {
    PointerCellContainsChar,
    NoValue(Location)
}

pub fn extract_memory_position(cell: Location, s: &InternalState) -> Result<usize, Error> {
    match cell {
        Location::Cell(mempos) => Ok(mempos),
        Location::Address(mempos) => {
            let value_from_memory = s.memory[mempos].clone();
            match value_from_memory {
                Some(Value::Number{value: pointed_cell}) => Ok(pointed_cell as usize),
                Some(Value::Character{value: _}) => Err(Error::PointerCellContainsChar),
                None => Err(Error::NoValue(Location::Cell(mempos)))
            }
        }
    }
}

pub fn explain(error: Error) -> String {
    match error {
        Error::NoValue(Location::Cell(_cell)) =>
            format!("There is no value at cell {:?}", _cell),
        Error::NoValue(Location::Address(_cell)) =>
            format!("There is no value at cell {:?}", _cell),
        Error::PointerCellContainsChar =>
            String::from("The selected cell should contain a number, not a char"),
    }
}
