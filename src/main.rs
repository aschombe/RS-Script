// #[cfg(not(windows))]
// mod compile;
// #[cfg(not(windows))]
// use compile::Compiler;

// mod interp;
// use interp::error_handler::ErrorHandler;

// use std::{env::args, path::PathBuf};

// fn main() {
//     let mut args: std::env::Args = args();
//     let program_name: String = args.next().unwrap();

//     let mut path: Option<PathBuf> = None;
//     let mut compile: bool = false;
//     let mut executable_name: Option<String> = None;
//     let current_dir: Option<PathBuf>;

//     current_dir = Some(std::env::current_dir().unwrap());

//     for arg in args {
//         if arg == "-c" {
//             compile = true;
//             // #[cfg(windows)]
//             // {
//             //     println!("Cannot compile on Windows");
//             //     return;
//             // }
//         } else {
//             path = Some(PathBuf::from(arg.clone()));
//             executable_name = Some(arg.clone());

//             // grab the name of the file
//             executable_name = Some(
//                 executable_name
//                     .unwrap()
//                     .split('/')
//                     .last()
//                     .unwrap()
//                     .to_string(),
//             );

//             // remove the extension
//             executable_name = Some(
//                 executable_name
//                     .unwrap()
//                     .split('.')
//                     .next()
//                     .unwrap()
//                     .to_string(),
//             );
//         }
//     }
//     if let Some(path) = path {
//         if let Some(extension) = path.extension() {
//             if extension == "rss" {
//                 if compile {
//                     // compile the file
//                     #[cfg(not(windows))]
//                     {
//                         let compiler: Compiler = Compiler::new(path);
//                         compiler.compile();
//                         return;
//                     }
//                     println!("Cannot compile on Windows");
//                 } else {
//                     // interpret the file
//                     let mut interpreter: interp::Interpreter = interp::Interpreter::new();
//                     let _res: Result<(), ErrorHandler> = interpreter.interp(path);
//                 }
//             } else if extension == "ll" {
//                 #[cfg(windows)]
//                 {
//                     println!("Cannot compile on Windows");
//                     return;
//                 }
//                 // invoke clang to compile the llvm file
//                 let mut cmd: std::process::Command = std::process::Command::new("clang");
//                 cmd.arg("-o");
//                 cmd.arg(format!(
//                     "{}/{}",
//                     current_dir.as_ref().unwrap().to_str().unwrap(),
//                     executable_name.as_ref().unwrap()
//                 ));
//                 cmd.arg(path);
//                 let res: std::process::Output = cmd.output().unwrap();

//                 // run the newly compiled executable
//                 if res.status.success() {
//                     let mut cmd: std::process::Command = std::process::Command::new(format!(
//                         "{}/{}",
//                         current_dir.unwrap().to_str().unwrap(),
//                         executable_name.unwrap()
//                     ));
//                     let res: std::process::Output = cmd.output().unwrap();
//                     println!("{}", String::from_utf8_lossy(&res.stdout));
//                 } else {
//                     println!("{}", String::from_utf8_lossy(&res.stderr));
//                 }
//             } else {
//                 println!("Invalid file extension");
//             }
//         } else {
//             println!("No file extension");
//         }
//     } else {
//         println!("Usage: {} <file> [-c]", program_name);
//     }
// }

// use rss::error_handler::ErrorHandler;

use rss::error_handler::ErrorHandler;

static CODE: &str = r#"
// let x:float = (5.0 + 1.0) * 2.0;
// x = 3.0;
// del x;
// let y:float = 1.0 + x;

// func myst(a:float, b:float):float {
//     let x:int = 1;
//     return x + c;
// }
// myst(1.0, 2.0);

// let x:float = myst(1.0, 2.0);

func factorial(n:int):int {
    if (n == 0) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

// if (1 != 1) {
//     let x:int = 1;
//     let a:float = 2;
// } elif (1 == 1) {
//     let y:int = 2;
//     let b:float = 3;
// } else {
//     let z:int = 3;
//     let c:float = 4;
// }

// let x:int = 0;
// for (i; j < 10; k = i + 1) { 
//     1 + 1;
// }

// let x:int = 5;
// while (x > 0) {
//     x = x - 1;
// }
"#;

fn main() {
    // tokenize the code
    let tokens: Vec<String> = rss::tokenizer::tokenize(CODE);
    println!("Tokenized: {:?}", tokens);
    
    if tokens.is_empty() {
        println!("{}", ErrorHandler::NoProgram);
        return;
    }

    println!("Parsed:");
    let mut parser: rss::parser::Parser = rss::parser::Parser::new(&tokens);
    match parser.parse() {
        Ok(ast) => println!("{:?}", ast),
        Err(err) => println!("{}", err),
    }
}