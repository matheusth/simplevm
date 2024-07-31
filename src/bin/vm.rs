use simplevm::Machine;

fn main() -> Result<(), &'static str> {
    let mut vm = Machine::new();
    vm.step()
}
