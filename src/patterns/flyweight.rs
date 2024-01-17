use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub trait TFlyWeight {
    fn operation(&self, component: Box<dyn TComponent>);
}

pub trait TComponent {
    fn what(&self) {
        println!("Default");
    }
}

pub struct Tree {
    pub height: u32,
    pub age: u32,
}

impl TComponent for Tree {
    fn what(&self) {
        println!("tree with height {} and age {}", self.height, self.age);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FlyWeightTree {
    // these fields are random
    pub structure: String,
    pub name: String,
}

impl TFlyWeight for FlyWeightTree {
    fn operation(&self, component: Box<dyn TComponent>) {
        println!("Intrinsic: {} {}", self.structure, self.name);
        component.what();
    }
}

pub struct FlyWeightFactory {
    pub flyweights: RefCell<HashMap<String, Rc<FlyWeightTree>>>,
}

impl FlyWeightFactory {
    pub fn new() -> Self {
        Self {
            flyweights: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_flyweight(&self, key: &'static str) -> Rc<FlyWeightTree> {
        let mut borrow = self.flyweights.borrow_mut();
        if borrow.get(key).is_none() {
            println!("tree didn't exist, creating new one");
            borrow.insert(
                key.to_string(),
                Rc::new(FlyWeightTree {
                    structure: String::from("structure or something"),
                    name: String::from("name or something"),
                }),
            );
        } else {
            println!("tree existed, creating reference to it"); 
        }
        borrow.get(key).unwrap().clone()
    }
}

impl Default for FlyWeightFactory {
    fn default() -> Self {
        Self::new()
    }
}
