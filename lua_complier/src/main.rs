use std::env;
use std::fs::File;


mod value;
mod byte_code;
mod lex;
mod parse;
mod vm;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2{
        println!("usage : {} script",args[0]);
        return;
    }

    let file = File::open(&args[1]).unwrap();
    let proto = parse::ParseProto::load(file);
    vm::ExeState::new().execute(&proto);
}
