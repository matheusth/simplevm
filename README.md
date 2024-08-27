# Simple VM

A simple 8-bit VM implementation for studying rust, it has two binaries `vm`, `asm`, where the first is the vm and the second an assembler.

# Assembler

The assembler creates an binary executable by the VM from a human-readle file.

# Binaries

## ASM

the assembler that creates a file accepted by the VM.
```
$ cargo run --bin asm <input> <output>
```

### Parameters

| Name   | Required | Default                              | Description                        |
| :----: | :------: | :----------------------------------: | :--------------------------------: |
| input  | yes      | -                                    | the file to be assembled           |
| output | no       | input without the extension + '.bin' | the file to output the program to. |


## VM

command to execute the vm on a compiled program.
```
$ cargo run --bin vm <program>
```

### Registers

 - A
 - B
 - C
 - M
 - SP
 - PC
 - BP
 - Flags

### Parameters

| Name    |Required  |Default  | Description                           |
| :-----: | :------: | :-----: | :-----------------------------------: |
| program | yes      | -       | the program to be executed by the vm. |

### Instructions

| Command     | Parameters                 | Description                                                                          |
| :---------: | :------------------------: | :----------------------------------------------------------------------------------: |
| Push        | U16 Interger               | Push a u16 to the stack.                                                             |
| PopRegister | U8 Integer                 | Pop the top of the stack to the register with the selected index.                    |
| AddStack    | -                          | Add the two most top numbers on the stack and push the result to the stack.          |
| AddRegister | Register1, Register2       | Add Register2 to Register1, see the [register section](#Registers) for register names.|
| Signal      | U8 Signal                  | Send a signal to the VM.                                                             |

### Signals

- `0xf0`, Halt Signal
