pub trait TWindoof {
    fn bsod(&self);
}

pub trait TPenguinOS {
    fn kernel_panic(&self);
}

pub struct APenguinOS {}

impl TPenguinOS for APenguinOS {
    fn kernel_panic(&self) {
        println!("Bye mr peng");
    }
}

impl APenguinOS {
    pub fn create() -> Self {
        Self {}
    }
}

pub struct Windoof {}

impl TWindoof for Windoof {
    fn bsod(&self) {
        println!("A classic....");
    }
}

pub struct Wine {
    pub peng: APenguinOS,
}

impl Wine {
    pub fn create(peng: APenguinOS) -> Self {
        Self { peng }
    }
}

impl TWindoof for Wine {
    fn bsod(&self) {
        self.peng.kernel_panic();
        println!("wait no this should be a bsod...");
    }
}
