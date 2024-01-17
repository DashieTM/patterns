use std::{cell::RefCell, rc::Rc};

pub trait TObserver {
    fn update(&self);
}

pub trait TSubject {
    fn attach(&mut self, observer: Box<dyn TObserver>);
    fn detach(&mut self, index: usize);
    fn notify(&self);
}

pub struct Observer {
    pub subject: Rc<RefCell<Subject>>,
}

impl TObserver for Observer {
    fn update(&self) {
        // note usually, the get_value function is called alter on
        // but it can also be called here.
        // The idea is that classes holding the observer, will at a random point
        // get the value after an update -> on ready
        println!("new val {}", self.subject.borrow().get_value());
    }
}

pub struct Subject {
    val: u32,
    pub observers: Vec<Box<dyn TObserver>>,
}

impl Subject {
    pub fn new(val: u32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            observers: Vec::new(),
        }))
    }
    pub fn set_value(&mut self, new_val: u32) {
        self.val = new_val;
    }
    pub fn get_value(&self) -> u32 {
        self.val
    }
}

impl TSubject for Subject {
    fn attach(&mut self, observer: Box<dyn TObserver>) {
        self.observers.push(observer);
    }

    fn detach(&mut self, index: usize) {
        self.observers.remove(index);
    }

    fn notify(&self) {
        for observer in self.observers.iter() {
            observer.update();
        }
    }
}
