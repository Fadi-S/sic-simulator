use std::fs::File;
use std::io;
use std::io::Read;

mod code;

fn main() -> Result<(), io::Error>{
    let mut code = String::new();
    File::open("code.sic")?.read_to_string(&mut code)?;

    let mut code = code::Code::compile(&mut code);

    code.execute();

    code.print();

    Ok(())
}