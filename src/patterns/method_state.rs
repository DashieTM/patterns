pub struct Mode {
    pub op1: fn(what_ref: &mut What) -> (),
    pub op2: fn(what_ref: &mut What) -> (),
    pub op3: fn(what_ref: &mut What) -> (),
    pub next_mode: fn() -> Mode, 
}

impl Mode {
    pub fn mode_grief() -> Self {
        Self {
            op1: |what_ref| {
                println!("grief");
            },
            op2: |what_ref| {
                what_ref.val += 1;
            },
            op3: |what_ref| {
                what_ref.val -= 1;
            },
            next_mode: Mode::mode_graf, 
        }
    }

    pub fn mode_graf() -> Self {
        Self {
            op1: |what_ref| {
                println!("graf");
            },
            op2: |what_ref| {
                what_ref.val += 10;
            },
            op3: |what_ref| {
                what_ref.val -= 10;
            },
            next_mode: Mode::mode_grief, 
        }
    }
}

pub struct What {
    pub val: u32,
    pub mode: Mode,
}

impl What {
    pub fn what_grief() -> Self {
        Self {
            val: 0,
            mode: Mode::mode_grief(),
        }
    }

    pub fn what_graf() -> Self {
        Self {
            val: 0,
            mode: Mode::mode_graf(),
        }
    }

    pub fn op1(&mut self) {
        (self.mode.op1)(self);
    }

    pub fn op2(&mut self) {
        (self.mode.op2)(self);
    }

    pub fn op3(&mut self) {
        (self.mode.op3)(self);
    }

    pub fn change_mode(&mut self) {
       self.mode = (self.mode.next_mode)();
    }

}
