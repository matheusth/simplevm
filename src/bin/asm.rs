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
    for line in buff.lines() {
        for t in line.unwrap().split(' ').filter(|x| !x.is_empty()) {
            let b = u8::from_str_radix(t, 16)
                .map_err(|x| format!("parse int: {}", x))
                .unwrap();
            output.push(b);
        }
    }
    stdout().write_all(&output).unwrap();
}
