use std::{fs::File, io::Read};
use std::env;
use std::io::BufReader;
use std::path::Path;
use simplevm::{Machine, Register};

fn main() -> Result<(), String> {
    let mut vm = Machine::new();
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).unwrap();
    let mut buff = BufReader::new(file);
    let mut program: Vec<u8> = Vec::new();

    let _ = buff.read_to_end(&mut program);
    vm.memory.load_into(&program, 0);
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;

    println!("A = {}", vm.get_register(Register::A));
    Ok(())
}
