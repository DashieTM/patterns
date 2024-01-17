// the main difference is the bounding of children to 1 -> e.g. a straight hierarchy 0-1 parents
// and 1 child for each parent, instead of 1-* children with a composite

pub trait TDecorator {
    fn operation(&self);
}

pub struct Decorator {
    pub child: Box<dyn TDecorator>,
}

impl Decorator {
    pub fn create(child: Box<dyn TDecorator>) -> Box<Self> {
        Box::new(Self { child })
    }
}

impl TDecorator for Decorator {
    fn operation(&self) {
        self.child.operation();
        println!("Decorated");
    }
}

pub struct DecoratorLeaf {
    pub val: i32,
}

impl DecoratorLeaf {
    pub fn create(val: i32) -> Box<Self> {
        Box::new(Self { val })
    }
}

impl TDecorator for DecoratorLeaf {
    fn operation(&self) {
        println!("hit the leaf, val: {}", self.val);
    }
}
