pub trait State {
    fn operation(&self) {
        println!("not implemented");
    }
}

pub struct Context {
    pub state: Box<dyn State>,
}

pub struct StateStart {}
impl State for StateStart {
    fn operation(&self) {
        println!("Game started");
    }
}

pub struct StateEnd {}
impl State for StateEnd {
    fn operation(&self) {
        println!("Game ended");
    }
}
