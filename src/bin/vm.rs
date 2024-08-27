use simplevm::{Machine};
use simplevm::register::Register;
use std::env;
use std::io::BufReader;
use std::path::Path;
use std::{fs::File, io::Read};

fn signal_halt(vm: &mut Machine) -> Result<(), String> {
    vm.halt = true;
    Ok(())
}

fn main() -> Result<(), String> {
    let mut vm = Machine::new();
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).unwrap();
    let mut buff = BufReader::new(file);
    let mut program: Vec<u8> = Vec::new();

    vm.define_handler(0xf0, signal_halt);
    let _ = buff.read_to_end(&mut program);
    vm.memory.load_into(&program, 0);
    while !vm.halt {
        vm.step()?;
    }
    println!("A = {}", vm.get_register(Register::A));
    println!("B = {}", vm.get_register(Register::B));
    println!("C = {}", vm.get_register(Register::C));
    Ok(())
}
