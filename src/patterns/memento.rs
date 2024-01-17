pub struct Memento {
    pub data: String,
}

impl Memento {
    pub fn get_data(&self) -> String {
        self.data.clone()
    }
}

pub struct Originator {
    pub internal_data: String,
}

impl Originator {
    pub fn create() -> Self {
        Self {
            internal_data: String::from(""),
        }
    }

    pub fn set_memento(&mut self, memento: Memento) {
        self.internal_data = memento.get_data();
    }

    pub fn create_memento(&self, data: String /*might be any type you want/need*/) -> Memento {
        Memento { data }
    }
}

// technically there is the caretaker -> this might be a DB, or just internal data, whatever you
// would need there....
