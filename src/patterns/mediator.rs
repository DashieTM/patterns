use std::rc::Rc;

pub trait TMediator {
    fn mediate(&self);
}

pub trait TClient {
    fn get_name(&self) -> String;
}

#[derive(Clone)]
pub struct Mediator {
    pub clients: Vec<Rc<dyn TClient>>,
}

impl Mediator {
    pub fn create() -> Box<Self> {
        let mut mediator = Box::new(Self {
            clients: Vec::new(),
        });

        // annoying in rust due to borrow checking rules -> use of unique pointer in one direction
        // and a shared pointer in the other direction
        mediator.clients = vec![
            Rc::new(Client::new(mediator.clone(), String::from("peng"))),
            Rc::new(Client::new(mediator.clone(), String::from("ereng"))),
        ];
        mediator
    }
}

impl TMediator for Mediator {
    fn mediate(&self) {
        println!("Mediating for: ");
        for client in self.clients.iter() {
            println!("{}", client.get_name());
        }
    }
}

pub struct Client {
    pub mediator: Box<dyn TMediator>,
    pub name: String,
}

impl Client {
    pub fn new(mediator: Box<dyn TMediator>, name: String) -> Self {
        Self { mediator, name }
    }
}

impl TClient for Client {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
