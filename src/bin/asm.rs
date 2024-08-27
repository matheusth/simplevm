use simplevm::op::Op;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

fn main() {
    // ./asm file.asm
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        panic!(
            "usage: {} <input>\nor\n{} <input> <output>",
            args[0], args[0]
        );
    }
    let input_path = Path::new(&args[1]);
    let output_path: PathBuf = if args.len() == 3 {
        PathBuf::from(&args[2])
    } else {
        input_path.with_extension("bin")
    };
    let file = File::open(input_path).unwrap();
    let buff = BufReader::new(file);
    let mut output: Vec<u8> = Vec::new();
    let encoded_instructions: Vec<u16> = buff
        .lines()
        .map(|e| e.unwrap())
        .filter(|e| !e.starts_with(';'))
        .map(|l| Op::try_from(l).unwrap())
        .map(|op| std::convert::TryInto::<u16>::try_into(op).unwrap())
        .collect();
    for ins in encoded_instructions {
        output.push((ins & 0xFF) as u8);
        output.push((ins >> 8) as u8);
    }
    let mut output_file = File::create(&output_path).unwrap();
    output_file.write_all(&output).unwrap();
}
