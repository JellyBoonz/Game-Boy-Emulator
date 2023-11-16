enum Instruction {
    ADD(ArithmeticTarget),
    SUB(ArithmeticTarget),
    ADC(ArithmeticTarget),
}
    
enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
  }

  struct MemoryBus {
    memory: [u8; 0xFFFF]
  }
  
  impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
      self.memory[address as usize]
    }
  }

// There are functions which allow the game to read/write 2 bytes at the same
// time to combined register ("af", "bc", "de", "hl")
impl Registers {
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u16::from(self.f)
    }

    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from(value as u8);
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

#[derive(Clone, Copy)]
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8  {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

impl std::convert::From<FlagsRegister> for u16 {
    fn from(flag: FlagsRegister) -> Self {
        ((flag.zero as u16) << ZERO_FLAG_BYTE_POSITION) |
        ((flag.subtract as u16) << SUBTRACT_FLAG_BYTE_POSITION) |
        ((flag.half_carry as u16) << HALF_CARRY_FLAG_BYTE_POSITION) |
        ((flag.carry as u16) << CARRY_FLAG_BYTE_POSITION)
    }
}

impl CPU {

    pub fn new() -> CPU {
        CPU {
            registers: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                f: FlagsRegister { zero: false, subtract: false, half_carry: false, carry: false },
                h: 0,
                l: 0,
            },
            pc: 0,
            bus: MemoryBus { memory: [0; 0xFFFF] },
        }
    }
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
          Instruction::ADD(target) => {
            match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                _ => { /* TODO: support more targets */ }
            }
          }
          Instruction::ADC(target) => {
            match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                } 
            }
          }
          Instruction::SUB(target) => {
            match target {
                ArithmeticTarget::A => {
                    self.registers.a = 0;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                _ => {/* not sure what else there is */}
            }
          }
          _ => { /* TODO: support more instructions */ }
        }
      }
    
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        //TODO: set flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
    fn adc(&mut self, value: u8) -> u8 {
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value + carry);
        //TODO: set flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        if(self.registers.f.carry) {
            return new_value;
        }
        new_value 
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_underflow) = self.registers.a.overflowing_sub(value); 
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_underflow;

        self.registers.f.half_carry = (value & 0xF) > (self.registers.a & 0xF);

        new_value
    }
    }

    #[cfg(test)]
    mod tests {
        // Define your unit tests within this module
        use super::*;
        #[test]
        fn test_add_instruction() {
            let mut cpu = CPU::new();
            cpu.registers.a = 0x10;
            cpu.registers.b = 0x20;
    
            // Call the function that executes the ADD instruction
            cpu.execute(Instruction::ADD(ArithmeticTarget::B));
    
            // Assert the expected results
            assert_eq!(cpu.registers.a, 0x30);
        }

        #[test]
        fn test_adc_instruction() {
            let mut cpu = CPU::new();
            cpu.registers.a = 0xFF; // Set A to the maximum value
            cpu.registers.f.carry = true; // Set carry flag to true

            // Arbitrary target
            cpu.execute(Instruction::ADC(ArithmeticTarget::C));

            // Assert the expected results
            // Since A is already at its maximum value and there's a carry,
            // the result should wrap around to 0, and the carry flag should be set.
            assert_eq!(cpu.registers.a, 0x00);
            assert_eq!(cpu.registers.f.carry, true);
            // TODO: Add more assertions for other flags and values as needed
        }

        #[test]
        fn test_sub_instruction() {
            let mut cpu = CPU::new();
            cpu.registers.a = 0x30;
            cpu.registers.b = 0x10;

            cpu.execute(Instruction::SUB(ArithmeticTarget::B));

            assert_eq!(cpu.registers.a, 0x20);
        }
    }