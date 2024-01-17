use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub struct Value1 {
    val: i32,
    name: String,
    values: Vec<u32>,
}

impl Value1 {
    pub fn create(val: i32, name: String, values: Vec<u32>) -> Self {
        Self { val, name, values }
    }

    pub fn equal(&self, other: &Value1) -> bool {
        let mut hasher = DefaultHasher::new();
        self.val.hash(&mut hasher);
        self.name.hash(&mut hasher);
        self.values.hash(&mut hasher);
        let hash1 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        other.val.hash(&mut hasher);
        other.name.hash(&mut hasher);
        other.values.hash(&mut hasher);
        let hash2 = hasher.finish();
        hash1 == hash2
    }
}
