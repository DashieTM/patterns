pub trait TInternalIterator {
    fn for_each(&mut self, func: fn(&mut i32));
}

pub struct InternalGlobi {
    // usually generic
    pub data: Vec<i32>,
}

impl InternalGlobi {
    pub fn create(data: Vec<i32>) -> Self {
        Self { data }
    }
}

impl TInternalIterator for InternalGlobi {
    fn for_each(&mut self, func: fn(&mut i32)) {
        for mut item in self.data.iter_mut() {
            (func)(item);
        }
    }
}
