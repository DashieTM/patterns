pub trait Objecty {
    fn objtype(&self) {
        println!("This is the default object");
    }
    fn set_imp(&mut self, imp: Box<dyn Implementation>);
    fn imp(&self) -> &Box<dyn Implementation>;
}

pub trait Implementation {
    fn operation(&self) {
        println!("This is the default implementation");
    }
}

pub struct Monkey {
    pub imp: Box<dyn Implementation>,
}

impl Objecty for Monkey {
    fn objtype(&self) {
        println!("This is a monke");
    }
    fn set_imp(&mut self, imp: Box<dyn Implementation>) {
        self.imp = imp;
    }

    fn imp(&self) -> &Box<dyn Implementation> {
        &self.imp
    }
}

pub struct MonkeyImplementation {}

impl Implementation for MonkeyImplementation {
    fn operation(&self) {
        println!("uhahaahahha");
    }
}

pub struct Cat {
    pub imp: Box<dyn Implementation>,
}

impl Objecty for Cat {
    fn objtype(&self) {
        println!("This is a cat");
    }
    fn set_imp(&mut self, imp: Box<dyn Implementation>) {
        self.imp = imp;
    }
    fn imp(&self) -> &Box<dyn Implementation> {
        &self.imp
    }
}

pub struct CatImplementation {}

impl Implementation for CatImplementation {
    fn operation(&self) {
        println!("grengeng eeehh");
    }
}
