use std::rc::Rc;

pub trait TComposite {
    fn accept(&self, visitor: Rc<dyn TVisitor>);
    fn value(&self) -> &str;
}

pub struct Leaf {
    pub val: i32,
}

impl TComposite for Leaf {
    fn accept(&self, visitor: Rc<dyn TVisitor>) {
        visitor.visit_leaf(self);
    }

    fn value(&self) -> &str {
        "leaferoni"
    }
}

pub struct Node {
    pub children: Vec<Rc<dyn TComposite>>,
}

impl TComposite for Node {
    fn accept(&self, visitor: Rc<dyn TVisitor>) {
        visitor.visit_node(self);
        for child in self.children.iter() {
            child.accept(visitor.clone());
        }
    }

    fn value(&self) -> &str {
        "noderoni"
    }
}

pub trait TVisitor {
    fn visit_leaf(&self, leaf: &Leaf);
    fn visit_node(&self, leaf: &Node);
}

pub struct Visitor {}

impl TVisitor for Visitor {
    fn visit_leaf(&self, leaf: &Leaf) {
        println!("This is leaf with value {}", leaf.val);
    }

    fn visit_node(&self, node: &Node) {
        println!("This is node with {} children", node.children.len());
    }
}
