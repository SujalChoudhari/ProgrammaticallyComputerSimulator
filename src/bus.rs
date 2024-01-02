pub struct Bus<T> {
    data: T,
}

impl<T> Bus<T> {
    pub fn new(initial: T) -> Self {
        Bus { data: initial }
    }

    pub fn dump(&mut self, value: T) {
        self.data = value;
    }

    pub fn get(&mut self) -> &T {
        &self.data
    }
}

struct BusAssembly {
    pub data_bus: Bus<i32>,
    pub address_bus: Bus<i32>,
    pub internal_bus: Bus<i32>,
}

impl BusAssembly {
    pub fn new() -> Self {
        BusAssembly {
            data_bus: Bus { data: 0 },
            address_bus: Bus { data: 0 },
            internal_bus: Bus { data: 0 },
        }
    }    
}
