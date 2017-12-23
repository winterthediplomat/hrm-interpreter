extern crate clap;

extern crate hrm_interpreter;
use hrm_interpreter::json::{read_file, read_config, dump_state};
use hrm_interpreter::CodeIterator;
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

    let mut errored = false;
    let mut reason = String::new();
    {
        let code_execution = CodeIterator::new(&mut internal_state, code);

        for operation_result in code_execution {
            if operation_result.is_err() {
                errored = true;
                reason = operation_result.err().unwrap();
                break;
            }
        }
    }

    if errored {
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        println!("Error: {}", reason);
        println!("Dumping current internal state:");
        println!("{:?}", internal_state);
    }
    else {
        println!("{:?}", internal_state);
    }

    dump_state(&internal_state, srcpath, reason);
}
