use crate::{registers::RegisterSet, memory::Memory};
mod memory;
mod registers;

fn main() {
    println!("Hello, world!");

    let mut mem = Memory::new(2048, 512);
    let var = mem.allocate_n(10);
    mem.set_at(1, 30);


    println!("Value at var is {}",mem.get_at(1));
    mem.print();
}
