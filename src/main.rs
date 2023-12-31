use std::rc::Rc;

use patterns::{
    bridge::{Cat, CatImplementation, Monkey, MonkeyImplementation, Objecty},
    builder::{PoneBuilder, TPoneBuilder},
    command::{Globi, Peng, PrintCommand, TCommand},
    flyweight::{FlyWeightFactory, TFlyWeight, Tree},
    state::{Context, StateEnd, StateStart},
    strategy::{ContainerThing, StrategyGreng, StrategyPeng},
    visitor::{Leaf, Node, TComposite, Visitor},
};

use crate::patterns::observer::{Observer, Subject, TSubject};

pub mod patterns;

fn main() {
    // state
    let mut grengeng = Context {
        state: Box::new(StateStart {}),
    };
    grengeng.state.operation();
    grengeng.state = Box::new(StateEnd {});
    grengeng.state.operation();

    // strategy
    let mut thing = ContainerThing {
        container: Vec::new(),
        strategy: Box::new(StrategyPeng {}),
    };
    thing.strategy.algorithm();
    thing.strategy = Box::new(StrategyGreng {});
    thing.strategy.algorithm();

    // bridge
    let mut bridge_obj: Box<dyn Objecty> = Box::new(Monkey {
        imp: Box::new(MonkeyImplementation {}),
    });
    bridge_obj.objtype();
    bridge_obj.imp().operation();
    bridge_obj.set_imp(Box::new(CatImplementation {}));
    bridge_obj.objtype();
    bridge_obj.imp().operation();
    bridge_obj = Box::new(Cat {
        imp: Box::new(CatImplementation {}),
    });
    bridge_obj.objtype();
    bridge_obj.imp().operation();

    // visitor with composite
    let visitor = Rc::new(Visitor {});
    let tree = Node {
        children: vec![Rc::new(Node {
            children: vec![Rc::new(Leaf { val: 2 }), Rc::new(Leaf { val: 3 })],
            val: 1,
        })],
        val: 0,
    };
    tree.accept(visitor);

    // builder
    let pone = PoneBuilder::new()
        .height(20)
        .weight(20)
        .name(String::from("Dashie"))
        .best_pone(true)
        .build();
    pone.display();

    // command
    let peng = Box::new(Peng {});
    let mut command = PrintCommand { receiver: peng };
    // invoke
    command.exec();
    let globi = Box::new(Globi {});
    command.receiver = globi;
    command.exec();

    // Flyweight
    let factory = FlyWeightFactory::new();
    let tree = Box::new(Tree { height: 20, age: 1 });
    let flyweight = factory.get_flyweight("default");
    flyweight.operation(tree);
    let other = factory.get_flyweight("default");
    assert_eq!(flyweight, other);
    println!("worky");

    // Observer
    let subject = Subject::new(0);
    let observer1 = Box::new(Observer {
        subject: subject.clone(),
    });
    let observer2 = Box::new(Observer {
        subject: subject.clone(),
    });
    subject.borrow_mut().attach(observer1);
    subject.borrow_mut().attach(observer2);
    subject.borrow_mut().set_value(10);
    subject.borrow().notify();
}
