use std::fs::File;
use std::{env, io};
use std::io::Read;

mod code;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    let mut code = String::new();
    File::open(&args[1])?.read_to_string(&mut code)?;

    let mut code = code::Code::compile(&mut code);

    code.execute();

    code.print();

    Ok(())
}