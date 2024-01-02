use crate::{memory::Memory, registers::RegisterAssembly, bus::Bus};
mod bus;
mod memory;
mod registers;

fn main() {
    println!("Hello, world!");

    let mut bus = Bus::new(0);
    bus.dump(5);

}
