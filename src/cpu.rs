enum Instruction {
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    AND(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget)
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
            Instruction::ADD(target) => self.add(target),
            Instruction::ADC(target) => self.adc(target),
            Instruction::ADDHL(target) => self.add_hl(target),
            Instruction::AND(target) => self.and(target),
            Instruction::SUB(target) => self.sub(target),
            Instruction::SBC(target) => self.sbc(target),
            // Add more instructions as needed
            _ => {} // Ignore unsupported instructions for now
        }
    }

    fn add(&mut self, target: ArithmeticTarget) {
        let value = self.get_register_value(target);
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.update_flags_add(value, new_value, did_overflow);
        self.set_register_value(ArithmeticTarget::A, new_value);
    }

    fn adc(&mut self, target: ArithmeticTarget) {
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let value = self.get_register_value(target);
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value + carry);
        self.update_flags_add(value, new_value, did_overflow);
        self.set_register_value(ArithmeticTarget::A, new_value);
    }

    fn add_hl(&mut self, target: ArithmeticTarget) {
        let value = self.get_register_value(target);
        let hl = self.registers.get_hl();
        let (result, did_overflow) = hl.overflowing_add(value as u16);

        // Update HL register
        self.registers.set_hl(result);

        // Set flags if needed (similar to the add function)
    }

    fn and(&mut self, target: ArithmeticTarget) {
        let value = self.get_register_value(target);
        self.set_register_value(ArithmeticTarget::A, self.registers.a & value);
    }

    fn sub(&mut self, target: ArithmeticTarget) {
        let value = self.get_register_value(target);
        let (new_value, did_underflow) = self.registers.a.overflowing_sub(value);
        self.set_register_value(ArithmeticTarget::A, new_value);
        self.update_flags_sub(value, new_value, did_underflow);
    }

    fn sbc(&mut self, target: ArithmeticTarget) {
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let value = self.get_register_value(target);
        let (new_value, did_underflow) = self.registers.a.overflowing_sub(value + carry);
        self.set_register_value(ArithmeticTarget::A, new_value);
        self.update_flags_sub(value, new_value, did_underflow);
    }

    // Other helper functions
    fn get_register_value(&self, target: ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
        }
    }

    fn set_register_value(&mut self, target: ArithmeticTarget, value: u8) {
        match target {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
        }
    }

    fn update_flags_add(&mut self, operand: u8, result: u8, did_overflow: bool) {
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (operand & 0xF) > 0xF;
    }

    fn update_flags_sub(&mut self, operand: u8, result: u8, did_underflow: bool) {
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_underflow;

        self.registers.f.half_carry = (operand & 0xF) > (self.registers.a & 0xF)
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

        #[test]
        fn test_sbc_instruction() {
            let mut cpu = CPU::new();
            cpu.registers.a = 0x30;
            cpu.registers.b = 0x10;
            cpu.registers.f.carry = true;

            cpu.execute(Instruction::SBC(ArithmeticTarget::B));
            

            assert_eq!(cpu.registers.a, 0x1F);
        }

        #[test]
        fn test_addhl_instruction() {
            let mut cpu = CPU::new();
            cpu.registers.set_hl(0x0011);
            cpu.registers.b = 0x01;

            // Call the function that executes the ADDHL instruction with, for example, ArithmeticTarget::B
            cpu.execute(Instruction::ADDHL(ArithmeticTarget::B));

            // Assert the expected results
            // Adjust the expected values based on your specific test case
            assert_eq!(cpu.registers.get_hl(), 0x0012);
            // TODO: Add more assertions for other flags and values as needed
        }

        #[test]
        fn test_and_instruction() {
            let mut cpu = CPU::new();
            cpu.registers.a = 0b0110;
            cpu.registers.b = 0b1100;

            // Call the function that executes the ADDHL instruction with, for example, ArithmeticTarget::B
            cpu.execute(Instruction::AND(ArithmeticTarget::B));

            // Assert the expected results
            // Adjust the expected values based on your specific test case
            assert_eq!(cpu.registers.a, 0b0100);
            // TODO: Add more assertions for other flags and values as needed
        }


    }