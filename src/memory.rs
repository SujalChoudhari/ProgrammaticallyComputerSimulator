use std::{collections::HashMap, usize};

pub struct Memory {
    max_address_space: usize,
    memory: Vec<i32>,
    pointer: usize,
    index_table: HashMap<usize, u32>,
}

impl Memory {
    pub fn new(max_address_space: usize, index_table_size: usize) -> Self {
        Memory {
            max_address_space,
            memory: vec![0; max_address_space],
            pointer: 0,
            index_table: HashMap::with_capacity(index_table_size),
        }
    }

    pub fn increment(&mut self) {
        self.pointer = (self.pointer + 1) % self.max_address_space;
    }

    pub fn decrement(&mut self) {
        if self.pointer == 0 {
            self.pointer = self.max_address_space - 1;
        } else {
            self.pointer -= 1;
        }
    }

    pub fn read_one(&mut self) -> i32 {
        self.memory[self.pointer]
    }

    pub fn read_n(&mut self, n: usize) -> Vec<i32> {
        let mut result = Vec::with_capacity(n);
        while result.len() < n {
            result.push(self.read_one());
            self.increment();
        }
        result
    }

    pub fn push(&mut self, value: i32) {
        self.memory[self.pointer] = value;
        self.increment();
    }

    pub fn pop(&mut self) -> i32 {
        self.decrement();
        self.memory[self.pointer]
    }

    pub fn allocate_n(&mut self, n: u32) -> usize {
        let index = self.pointer.clone();
        self.index_table.insert(self.pointer, n);
        self.pointer = (self.pointer + n as usize) % self.max_address_space;
        index
    }

    pub fn deallocate(&mut self, address: usize) {
        self.index_table.remove(&address);
    }

    pub fn set_at(&mut self, address: usize, value: i32) {
        for (key, value) in &self.index_table {
            if address >= *key && address <= key + *value as usize - 1 {
                // useable spcae
                self.memory.insert(address, *value as i32);
                return;
            }
        }
        panic!("Read Access Violation");
    }

    pub fn get_at(&mut self, address: usize) -> i32 {
        let out = self.memory.get(address);
        match out {
            Some(value) => value.clone(),
            None => 0,
        }
    }

    pub fn print(&self) {
        println!("Pointer: {}", self.pointer);
        println!("Index Table:");
        println!("{:<18} {:<12}", "Start Address", "Size");
        for (start_address, size) in &self.index_table {
            println!("{:<18} {:<12}", start_address, size);
        }
    }
}

struct MemoryAssembly {
    pub stack: Memory,
    pub heap: Memory,
    pub data: Memory,
    pub text: Memory,
}

impl MemoryAssembly {
    pub fn new(
        stack_size: usize,
        heap_size: usize,
        data_size: usize,
        text_size: usize,
        index_table_size: usize,
    ) -> Self {
        MemoryAssembly {
            stack: Memory::new(stack_size, index_table_size),
            heap: Memory::new(heap_size, index_table_size),
            data: Memory::new(data_size, index_table_size),
            text: Memory::new(text_size, index_table_size),
        }
    }
}
