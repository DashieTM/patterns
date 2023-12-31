pub trait TCommand {
    fn exec(&self);
}

pub struct PrintCommand {
    pub receiver: Box<dyn TCommandable>,
}

impl TCommand for PrintCommand {
    fn exec(&self) {
        self.receiver.operation();
    }
}

pub trait TCommandable {
    fn operation(&self);
}

pub struct Peng {}

impl TCommandable for Peng {
    fn operation(&self) {
        println!("dodododo");
    }
}

pub struct Globi {}

impl TCommandable for Globi {
    fn operation(&self) {
        println!("mamau");
    }
}
