pub trait TPlush {
    fn print(&self);
}

pub trait TOS {
    fn print(&self);
}

pub struct AbstractPenguinPlush {}

impl TPlush for AbstractPenguinPlush {
    fn print(&self) {
        println!("fluffy af, but abstracted, kekw");
    }
}

pub struct AbstractPenguinOS {}

impl TOS for AbstractPenguinOS {
    fn print(&self) {
        println!("I use arch btw, but abstracted, kekw");
    }
}

pub trait TPenguinAbstractFactory {
    fn create_plush(&self) -> Box<dyn TPlush>;
    fn create_os(&self) -> Box<dyn TOS>;
}

pub struct PenguinAbstractFactory {}

impl PenguinAbstractFactory {
    pub fn create() -> Self {
        Self {}
    }
}

impl TPenguinAbstractFactory for PenguinAbstractFactory {
    fn create_plush(&self) -> Box<dyn TPlush> {
        Box::new(AbstractPenguinPlush {})
    }

    fn create_os(&self) -> Box<dyn TOS> {
        Box::new(AbstractPenguinOS {})
    }
}
