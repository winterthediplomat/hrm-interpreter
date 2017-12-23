HRM Interpreter
===============

Simple interpreter for the assembler language described in Human Resource Machine.

This interpreter is intended to be used to verify [hrm-compiler](https://github.com/alfateam123/hrm-compiler),
especially optimizations (unreachable/not executed code removal, jump optimizations...)

## Todo List

* [x] implement operations
* [x] implement json-formatted code
* [x] implement json-formatted input
* [x] output interpreter state as json at the end of execution
* [ ] insert debugging hooks (brakepoints, tracepoints)
* [ ] (maybe) execute source code directly

## How can I run my code with your interpreter?

1. Install [hrm-compiler](https://github.com/alfateam123/hrm-compiler)
2. Run `hrmc <mysourcefile.hrm>` - it will generate the json-formatted version of the code executed by `hrm-interpreter`
3. Run `cargo run -- --code <mysourcefile.json> --input <myinputfile.json>`
