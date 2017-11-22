extern crate clap;

extern crate hrm_interpreter;
use hrm_interpreter::json::{read_file, read_config};
use clap::{Arg, App};

fn main() {
    let app_data = App::new("hrm-interpreter")
                  .version("0.1")
		  .arg(Arg::with_name("code")
		       .short("c")
		       .long("code")
		       .value_name("CODE")
		       .takes_value(true))
		  .arg(Arg::with_name("input")
		       .short("i")
		       .long("input")
		       .value_name("INPUT")
		       .takes_value(true));

    let matches = app_data.get_matches();
    let srcpath = matches.value_of("code").unwrap();
    let inputpath = matches.value_of("input").unwrap();

    let code = read_file(String::from(srcpath));
    // create the state to be modified
    let mut internal_state = read_config(String::from(inputpath));

		loop {
			if internal_state.instruction_counter < code.len() {
				let _operation = code[internal_state.instruction_counter];
				println!("applying operation {:?}", _operation);

				let result = internal_state.apply(_operation);
				if result.is_err() {
					let reason = result.err().unwrap();
					println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
					println!("Error: {}", reason);
					println!("Dumping current internal state:");
					println!("{:?}", internal_state);
					break;
				}
			}
			else {
				break;
			}
		}

		// print internal state
		println!("{:?}", internal_state);
}
