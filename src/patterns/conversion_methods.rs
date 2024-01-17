pub struct Globi1 {
    pub val: u32,
}

pub struct Globi2 {
    pub val: i32,
}

impl Globi1 {
    pub fn to_globi2(&self) -> Globi2 {
        Globi2 {
            val: self.val as i32,
        }
    }
}

