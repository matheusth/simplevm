use crate::register::Register;

fn parse_instruction_arg(ins: u16) -> u8 {
    (ins >> 8) as u8
}

#[repr(u8)]
pub enum Op {
    Nop,
    Push(u16),
    PopRegister(Register),
    AddStack,
    AddRegister(Register, Register),
    Signal(u8),
}

impl Op {
    fn encode_r1(r: Register) -> u16 {
        ((r as u16) & 0x0f) << 8
    }
    fn encode_r2(r: Register) -> u16 {
        ((r as u16) & 0x0f) << 12
    }
    fn parse_numeric(s: &str) -> Result<u8, String> {
        if s.is_empty() {
            return Err("Empity string".to_string());
        }
        let (num, radix) = match s {
            s if s.starts_with('$') => (&s[1..], 16),
            s if s.starts_with('%') => (&s[1..], 2),
            _ => (s, 10),
        };
        u8::from_str_radix(num, radix).map_err(|x| format!("{}", x))
    }
}

impl TryFrom<u16> for Op {
    type Error = String;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let op = (value & 0xff) as u8;
        match op {
            0 => Ok(Op::Nop),
            1 => {
                let arg = parse_instruction_arg(value);
                Ok(Op::Push(arg as u16))
            }
            2 => {
                let arg = parse_instruction_arg(value);
                Ok(Op::PopRegister(Register::try_from(arg)?))
            }
            3 => Ok(Op::AddStack),
            4 => {
                let reg1 = Register::try_from(((value & 0xf00) >> 8) as u8)?;
                let reg2 = Register::try_from(((value & 0xf000) >> 12) as u8)?;
                println!("{:?}, {:?}", &reg1, &reg2);
                Ok(Op::AddRegister(reg1, reg2))
            }
            5 => Ok(Op::Signal(parse_instruction_arg(value))),
            _ => Err(format!("invalid instruction {:X}", value)),
        }
    }
}

impl TryFrom<String> for Op {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let splited_value: Vec<&str> = value.split(' ').filter(|v| !v.is_empty()).collect();
        match splited_value[0] {
            "Push" => {
                let arg: u16 = Op::parse_numeric(splited_value[1])? as u16;
                Ok(Op::Push(arg))
            }
            "PopRegister" => {
                if let Ok(arg) = Op::parse_numeric(splited_value[1]) {
                    return Ok(Op::PopRegister(arg.try_into().unwrap()));
                }
                Err(format!(
                    "Invalid argument {} for PopRegister.",
                    splited_value[1]
                ))
            }
            "AddRegister" => {
                if let (Ok(r1), Ok(r2)) = (
                    Register::try_from(splited_value[1]),
                    Register::try_from(splited_value[2]),
                ) {
                    return Ok(Op::AddRegister(r1, r2));
                }
                Err(format!(
                    "AddRegister reviced invalid arguments: {:?}",
                    splited_value
                ))
            }
            "AddStack" => Ok(Op::AddStack),
            "Signal" => {
                let arg: u8 = Op::parse_numeric(splited_value[1])?;
                Ok(Op::Signal(arg))
            }
            _ => Err(format!("Invalid instruction {}", splited_value[0])),
        }
    }
}
impl TryFrom<Op> for u16 {
    type Error = String;
    fn try_from(value: Op) -> Result<Self, Self::Error> {
        match value {
            Op::Push(x) => Ok(1 | x << 8),
            Op::PopRegister(x) => Ok(2 & 0xff | (x as u16) << 8),
            Op::AddStack => Ok(3),
            Op::Signal(x) => Ok(5 & 0xff | (x as u16) << 8),
            Op::AddRegister(r1, r2) => {
                println!("{:?}, {:?}", &r1, &r2);
                let r1 = Op::encode_r1(r1);
                let r2 = Op::encode_r2(r2);
                println!("{:?}, {:?}", r1, r2);
                Ok(4 | r1 | r2)
            }
            _ => Err(format!(
                "Unimplemented op {}!",
                u16::try_from(value).unwrap()
            )),
        }
    }
}
