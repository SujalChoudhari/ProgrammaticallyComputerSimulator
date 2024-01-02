pub struct Register {
    value: i32,
}

impl Register {
    pub fn new(initial: i32) -> Self {
        Register { value: initial }
    }

    pub fn get(&self) -> i32 {
        self.value
    }

    pub fn set(&mut self, value: i32) {
        self.value = value;
    }

    pub fn print(self) {
        println!("{:?}", self.value);
    }
}

pub struct RegisterSet {
    pub memory_buffer_register: Register,
    pub memory_address_register: Register,
    pub instruction_register: Register,
    // in cpu
    pub instruction_buffer_register: Register,
    pub program_counter: Register,
    pub accumulator: Register,
    pub multiplier_quotient: Register,
}

impl RegisterSet {
    pub fn new() -> Self {
        RegisterSet {
            memory_buffer_register: Register::new(0),
            memory_address_register: Register::new(0),
            instruction_register: Register::new(0),
            instruction_buffer_register: Register::new(0),
            program_counter: Register::new(0),
            accumulator: Register::new(0),
            multiplier_quotient: Register::new(0),
        }
    }

    pub fn print_all(&self) {
        println!("MBR: {:?}", self.memory_buffer_register.get());
        println!("MAR: {:?}", self.memory_address_register.get());
        println!("IR:  {:?}", self.instruction_register.get());
        println!("IBR: {:?}", self.instruction_buffer_register.get());
        println!("PC:  {:?}", self.program_counter.get());
        println!("ACC: {:?}", self.accumulator.get());
        println!("MQ:  {:?}", self.multiplier_quotient.get());
    }
}
