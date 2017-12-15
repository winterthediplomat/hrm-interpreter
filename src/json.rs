use std::fs::File;
use std::io::prelude::*;
use serde_json;

use Operation;
use Value;
use Location;
use state::InternalState;


// JSON data format for json-ified source code
#[derive(Debug, Deserialize, Clone)]
pub enum JsonOperand {
    Label(String),
    Address(u32),
    Cell(u32)
}

#[derive(Debug, Deserialize, Clone)]
pub struct JsonOperation {
    operation: String,
    operand: Option<JsonOperand>
}

// JSON data format for input files:
// using ::Value directly forces users to write horrible things,
// such as "input_tape": [{"Number": {"value": 3}}].
// Users can now write "input_tape: [3] and be happy.
#[derive(Deserialize, Clone)]
#[serde(untagged)]
enum JsonValue {
    Number(i32),
    Character(char)
}

#[derive(Deserialize, Clone)]
struct Config {
    input_tape: Vec<JsonValue>,
    memory: Vec<Option<JsonValue>>
}

impl JsonOperation {
    fn new(operation: String, operand: Option<JsonOperand>) -> Self {
        Self {
            operation: operation,
            operand: operand
        }
    }
}

fn to_operator(json_op: JsonOperation, labels_mapping: &Vec<(String, usize)>) -> Operation {
    if json_op.operation == String::from("inbox"){ return Operation::Inbox{}; }
    else if json_op.operation == String::from("add") {
        let cell_to_add = match json_op.operand.unwrap() {
            JsonOperand::Label(_) => panic!("only Address or Cell are valid operand for 'add'"),
            JsonOperand::Address(cell) => Location::Address(cell as usize),
            JsonOperand::Cell(cell) => Location::Cell(cell as usize)
        };
        return Operation::Add{cell: cell_to_add};
    }
    else if json_op.operation == String::from("copyfrom") {
        let cell = match json_op.operand.unwrap() {
            JsonOperand::Label(_) => panic!("only Address or Cell are valid operand for 'copyfrom'"),
            JsonOperand::Address(cell) => Location::Address(cell as usize),
            JsonOperand::Cell(cell) => Location::Cell(cell as usize)
        };
        return Operation::CopyFrom{cell: cell};
    }
    else if json_op.operation == String::from("copyto") {
        let cell = match json_op.operand.unwrap() {
            JsonOperand::Label(_) => panic!("only Address or Cell are valid operand for 'copyto'"),
            JsonOperand::Address(cell) => Location::Address(cell as usize),
            JsonOperand::Cell(cell) => Location::Cell(cell as usize)
        };
        return Operation::CopyTo{cell: cell};
    }
    else if json_op.operation == String::from("label") {
        return Operation::Label{};
    }
    else if json_op.operation == String::from("jmp") {
        if let JsonOperand::Label(label_name) = json_op.operand.unwrap() {
            let next_position = position_from_label(&label_name, &labels_mapping).unwrap();
            return Operation::Jump{next_operation: next_position};
        }
        else {
            panic!("only Labels are valid operands for jmp");
        }
    }
    else if json_op.operation == String::from("jez") {
        if let JsonOperand::Label(label_name) = json_op.operand.unwrap() {
            let next_position = position_from_label(&label_name, &labels_mapping).unwrap();
            return Operation::JumpEqualsZero{next_operation: next_position};
        }
        else {
            panic!("only Labels are valid operands for jmp");
        }
    }
    else if json_op.operation == String::from("outbox") { return Operation::Outbox{}; }
    else { panic!(format!("unrecognized operation {}", json_op.operation)) }
}

fn labels_to_positions(source_code: &Vec<JsonOperation>) -> Vec<(String, usize)> {
    let mut labels : Vec<(String, usize)> = vec!();

    let mut index = 0;
    for operation in source_code {
        if operation.operation == String::from("label") {
            if let JsonOperand::Label(label_name) = operation.clone().operand.unwrap() {
                labels.push((label_name, index));
            }
        }
        index += 1;
    }

   return labels;
}

fn position_from_label(label: &String, mapping: &Vec<(String, usize)>) -> Option<usize> {
    mapping.iter().filter_map(|pair| {
        let &(ref candidate_label, position) = pair;
        if label == candidate_label {
            return Some(position);
        }
        else {
            return None
        }
    }).last()
}

pub fn read_file(srcpath: String) -> Vec<Operation> {
    let mut file = File::open(srcpath).unwrap();
    let mut contents = String::new();
    let file_read_ok = file.read_to_string(&mut contents);
    if file_read_ok.is_err() {
        panic!("could not read the file!");
    }

    let source_code: Vec<JsonOperation> = serde_json::from_str(&contents).unwrap();
    let position_for_label = labels_to_positions(&source_code);
    let mut res: Vec<Operation> = vec!();
    for json_op in source_code {
        res.push(to_operator(json_op.clone(), &position_for_label));
    }

    return res;
}

pub fn read_config(path: String) -> InternalState  {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    let file_read_ok = file.read_to_string(&mut contents);
    if file_read_ok.is_err() {
        panic!("could not read the file!");
    }

    let input_config: Config = serde_json::from_str(&contents).unwrap();
    return InternalState{
        register: None,
        input_tape: input_config.input_tape.into_iter().map(|input| match input {
            JsonValue::Number(num_) => Value::Number{value: num_},
            JsonValue::Character(char_) => Value::Character{value: char_}
        }).collect(),
        output_tape: vec!(),
        instruction_counter: 0,
        memory: input_config.memory.into_iter().map(|memory_value| match memory_value {
            Some(JsonValue::Number(num_)) => Some(Value::Number{value: num_}),
            Some(JsonValue::Character(char_)) => Some(Value::Character{value: char_}),
            None => None
        }).collect()
    };
}

#[cfg(test)]
mod test {
    use Operation;
    use json::to_operator;
    use json::JsonOperation;
    use json::JsonOperand;
    use json::labels_to_positions;

    #[test]
    #[should_panic]
    fn to_operator_unknown() {
        let empty_labels_mapping = vec!();
        let src = JsonOperation{
            operation: String::from("fsdfsadfsadjsdf"),
            operand: None
        };

        to_operator(src, &empty_labels_mapping);
    }

    #[test]
    fn to_operator_label() {
        let empty_labels_mapping = vec!();
        let src = JsonOperation{
            operation: String::from("label"),
            operand: Some(JsonOperand::Label(String::from("mylabel")))
        };
        let result = to_operator(src, &empty_labels_mapping);

        assert!(match result {
            Operation::Label => true,
            _ => false
        });
    }

    #[test]
    fn to_operator_jump_label_found() {
        let mapping = vec!((String::from("myLabel"), 3));
        let operation = JsonOperation{
            operation: String::from("jmp"),
            operand: Some(JsonOperand::Label(String::from("myLabel")))
        };

        let result = to_operator(operation, &mapping);

        assert!(match result {
            Operation::Jump{next_operation: 3} => true,
            _ => false
        });
    }

    #[test]
    #[should_panic]
    fn to_operator_jump_label_not_found() {
        let mapping = vec!((String::from("myLabel"), 3));
        let operation = JsonOperation{
            operation: String::from("jmp"),
            operand: Some(JsonOperand::Label(String::from("fdfsdfsadj")))
        };

        to_operator(operation, &mapping);
    }

    #[test]
    fn labels_to_positions_empty_code() {
        let empty_vec = vec!();
        let result = labels_to_positions(&empty_vec);
        assert!(result.len() == 0);
    }

    #[test]
    fn labels_to_positions_no_labels() {
        let operations = vec!(
            JsonOperation{
                operation: String::from("copyto"),
                operand: Some(JsonOperand::Cell(2))
        });
        let result = labels_to_positions(&operations);
        assert!(result.len() == 0);
    }

    #[test]
    fn labels_to_positions_with_labels() {
        let operations = vec!(
            JsonOperation::new(String::from("label"), Some(JsonOperand::Label(String::from("firstlabel")))),
            JsonOperation::new(String::from("inbox"), None),
            JsonOperation::new(String::from("label"), Some(JsonOperand::Label(String::from("secondlabel")))),
            JsonOperation::new(String::from("jmp"), Some(JsonOperand::Label(String::from("firstlabel"))))
        );

        let result = labels_to_positions(&operations);

        assert!(result.len() == 2);
        assert!(result[0] == (String::from("firstlabel"), 0));
        assert!(result[1] == (String::from("secondlabel"), 2));
    }
}
