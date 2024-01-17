// difference to the command pattern is the ability to rollback and the command processor between
// the command and the reciever

pub trait TCommand {
    fn exec(&self);
    fn undo(&self);
}

pub struct CommandProcessor {
    pub stack: Vec<Box<dyn TCommand>>,
}

impl CommandProcessor {
    pub fn do_command(&mut self, command: Box<dyn TCommand>) {
        command.exec();
        self.stack.push(command);
    }

    pub fn undo_command(&mut self) {
        if let Some(command) = self.stack.pop() {
            command.undo();
        }
    }

    pub fn create() -> Self {
        Self { stack: Vec::new() }
    }
}

pub struct ProcessorPrintCommand {
    pub receiver: Box<dyn TCommandable>,
}

impl TCommand for ProcessorPrintCommand {
    fn exec(&self) {
        self.receiver.operation();
    }

    fn undo(&self) {
        self.receiver.undo();
    }
}

// Receiver trait
pub trait TCommandable {
    fn operation(&self);
    fn undo(&self);
}

pub struct ProcessorPeng {}

impl TCommandable for ProcessorPeng {
    fn operation(&self) {
        println!("dodododo");
    }

    fn undo(&self) {
        println!("no longer dododo");
    }
}

pub struct ProcessorGlobi {}

impl TCommandable for ProcessorGlobi {
    fn operation(&self) {
        println!("mamau");
    }

    fn undo(&self) {
        println!("no longer mamau");
    }
}
