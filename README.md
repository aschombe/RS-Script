# Rust Explorer

## Description
This is an interpreter (soon to have compiler feature) written in rust. Open to feature suggestions. Check [main.md](docs/main.md) for more information.

## Table of Contents
- [Rust Explorer](#rust-explorer)
  - [Table of Contents](#table-of-contents)
  - [Documentation](#documentation)
  - [Usage](#usage)
  - [Features](#features)
  - [Todo](#todo)
  - [Notes](#notes)

## Documentation
- Check out the [documentation](docs/main.md) for more information on the features of the language.

## Usage
To run the project, you need to have Rust installed. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install). After installing Rust, you can generate the `rss` tool by running the following in the project directory:
```bash
cargo run
```

Run the command line tool by running the following command:
```bash
./rss <path-to-file> <flag>
```
The file should have the extension `.rss` or `rssc`. You can find some example files in the `examples` directory.
More about the rss tool can be found in the [rss.md](docs/rss.md) file.

## Features
- [x] Print Statements
- [x] Comments (single line comments)
- [x] Arithmetic Operations
- [x] Math Functions
- [x] Comparison Operations
- [x] And, Or, Not
- [x] Nested Expressions
- [x] Variables
- [x] Floating point numbers
- [x] For loops
- [x] Control Flow
- [x] Add boolean keywords
- [x] Expansive Examples in the examples directory
- [x] Debug function (prints the current state of the interpreter (variables and functions))
- [x] Exit function (exits the interpreter)
- [x] Code can span multiple lines
- [x] String type for variables
- [x] String literals
- [x] String functions


## Todo
- [ ] Fix the way the interpreter and compiler receives the file path
- [ ] Fix print statements (the output is going before the line being interpreted)
- [ ] Create a bytecode for the language, and make a virtual machine to run the bytecode
 
## Notes
- Empty parenthesis '()' are evaluated to None
<!-- - Functions: (func "name" (placeholder arg1 arg2 ... argn) (body)), the placeholder can be anything but (), but it must be there (or everything breaks) (fix this eventually)
- Recursive Functions: Base case must be prepended by the "base" keyword. Look in the examples directory for an example of a recursive function -->
