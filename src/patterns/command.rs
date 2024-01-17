// the idea is that you can execute commands on a struct from anywhere as long as you have access
// to the command reference
// E.g. you can execute command xyz on Peng or on Globi, depending on which reference the command
// has

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

// Receiver trait
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
