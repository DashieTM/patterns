pub struct TestMediator {
    pub mediator: Mediator2,
}

impl TestMediator {
    pub fn create() -> Self {
        let mut gg = Self {
            mediator: Mediator2::create(),
        };
        gg.mediator.add_event_listener_sender(|name| {
            println!("sent {}", name);
        });

        gg.mediator.add_event_listener_receiver(|name| {
            println!("received {}", name);
        });
        gg.mediator.add_event_listener_receiver(|name| {
            println!("received 2.0 {}", name);
        });
        gg
    }

    pub fn send_data(&mut self, name: String) { 
        self.mediator.send_data(name);
    }

    pub fn receive_data(&mut self) {
        self.mediator.receive_data(String::from("received something"));
    }
}

#[derive(Clone)]
pub struct Mediator2 {
    // pub clients: Vec<Rc<dyn TClient>>,
    pub receivers: Vec<fn(name: String) -> ()>,
    pub senders: Vec<fn(name: String) -> ()>,
}

impl Mediator2 {
    pub fn create() -> Self {
        Self {
            receivers: Vec::new(),
            senders: Vec::new(),
        }
    }

    pub fn add_event_listener_sender(&mut self, sender: fn(name: String) -> ()) {
        self.senders.push(sender);
    }

    pub fn add_event_listener_receiver(&mut self, receiver: fn(name: String) -> ()) {
        self.receivers.push(receiver);
    }

    pub fn receive_data(&mut self, name: String) {
        for receiver in self.receivers.iter() {
            (receiver)(name.clone());
        }
    }

    pub fn send_data(&mut self, name: String) {
        for sender in self.senders.iter() {
            (sender)(name.clone());
        }
    }
}
