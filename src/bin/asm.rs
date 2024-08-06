use simplevm::Op;
use std::{
    env,
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
    path::Path,
};

fn main() {
    // ./asm file.asm
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }
    let file = File::open(Path::new(&args[1])).unwrap();
    let buff = BufReader::new(file);
    let mut output: Vec<u8> = Vec::new();
    let encoded_instructions: Vec<u16> = buff
        .lines()
        .map(|e| e.unwrap())
        .map(|l| Op::try_from(l).unwrap())
        .map(|op| std::convert::TryInto::<u16>::try_into(op).unwrap())
        .collect();
    for ins in encoded_instructions {
        output.push((ins & 0xFF) as u8);
        output.push((ins >> 8) as u8);
    }
    stdout().write_all(&output).unwrap();
}
