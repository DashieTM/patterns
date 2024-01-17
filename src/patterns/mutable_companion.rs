pub struct ImmutableValue {
    val: i32,
}

impl ImmutableValue {
    pub fn get_value(&self) -> i32 {
        self.val
    }
    pub fn create(val: i32) -> Self {
        Self { val }
    }
}

pub struct MutableCompanion {
    val: i32,
}

impl MutableCompanion {
    pub fn create(val: ImmutableValue) -> Self {
        Self { val: val.val }
    }

    pub fn increment_value(&mut self) {
        self.val += 1;
    }

    pub fn decrement_value(&mut self) {
        self.val -= 1;
    }

    // plain factory method
    pub fn as_value(&self) -> ImmutableValue {
        ImmutableValue { val: self.val }
    }
}
