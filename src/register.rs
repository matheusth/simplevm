#[repr(u16)]
#[derive(Debug)]
pub enum Register {
    A,
    B,
    C,
    M,
    SP,
    PC,
    BP,
    Flags,
}

impl TryFrom<u8> for Register {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::A),
            1 => Ok(Self::B),
            2 => Ok(Self::C),
            3 => Ok(Self::M),
            4 => Ok(Self::SP),
            5 => Ok(Self::PC),
            6 => Ok(Self::BP),
            7 => Ok(Self::Flags),
            _ => Err(format!("invalid register {:X}", value)),
        }
    }
}
