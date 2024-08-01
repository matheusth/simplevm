use simplevm::{Machine, Register};

fn main() -> Result<(), String> {
    let mut vm = Machine::new();
    /*
     * PUSH 10
     * PUSH 8
     * ADDSTACK
     * POPREGISTER A
     * POPREGISTER B
     * ADDREGISTER B A
     */

    //PUSH 10
    vm.memory.write(0, 0x1);
    vm.memory.write(1, 10);
    //PUSH 8
    vm.memory.write(2, 0x1);
    vm.memory.write(3, 8);
    //AddStask
    vm.memory.write(4, 0x3);
    //POPREGISTER A
    vm.memory.write(6, 0x2);
    vm.memory.write(7, 0x0);
    //PUSH 12
    vm.memory.write(8, 0x1);
    vm.memory.write(9, 12);
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;

    println!("A = {}", vm.get_register(Register::A));
    Ok(())
}
