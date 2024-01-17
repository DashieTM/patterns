// so this is akward, no need for this pattern in rust... lel

pub trait Prototype {
    fn clone(&self) -> Self;
}

#[derive(Debug)]
pub struct PrototypeImplementation {
    pub val: i32,
    pub whatever: Vec<u32>,
}

impl PrototypeImplementation {
    pub fn create() -> Self {
        Self {
            val: 4,
            whatever: vec![2, 43, 1, 4, 1],
        }
    }
}

impl Prototype for PrototypeImplementation {
    fn clone(&self) -> Self {
        // in javfuck collEction.addAll(whatever)
        let mut vec = Vec::new();
        vec.extend(self.whatever.iter());
        println!("cloned the object with deep copy!");
        Self {
            val: self.val,
            whatever: vec,
        }
    }
}
