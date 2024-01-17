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

use crate::patterns::{
    abstract_factory::{PenguinAbstractFactory, TPenguinAbstractFactory},
    adapter::{APenguinOS, TWindoof, Wine},
    command_processor::{CommandProcessor, ProcessorGlobi, ProcessorPeng, ProcessorPrintCommand},
    decorator::{Decorator, DecoratorLeaf, TDecorator},
    external_iterator::{GlobalGlobi, TAggregate, TIterator},
    facade::PCFacade,
    factory_method::{PenguinFactory, ProductType, TPenguinFactory},
    internal_iterator::{InternalGlobi, TInternalIterator},
    mediator::{Mediator, TMediator},
    memento::Originator,
    method_state::What,
    monostate::{eq, MockSingleton, MonoGlobi, SingletonMono, TrueSingleton},
    observer::{Observer, Subject, TSubject},
    prototype::{Prototype, PrototypeImplementation},
    singleton_bad::BADSINGLETON,
    template_method::{TTemplate, TemplateImplementation},
    value_object::Value1,
    whole_value::Date, mutable_companion::{ImmutableValue, MutableCompanion}, relative_value::Value2,
};

pub mod patterns;

fn main() {
    // mediator
    println!("=============Mediator==============");
    let mediator = Mediator::create();
    mediator.mediate();
    println!("=============Done==============\n");

    println!("=============Memento==============");
    // memento
    let mut originator = Originator::create();
    let mem1 = originator.create_memento(String::from("Ina is best vtuber, apparently"));
    let mem2 = originator.create_memento(String::from("Dashie is best pony"));
    originator.set_memento(mem2);
    println!("{}", originator.internal_data);
    originator.set_memento(mem1);
    println!("{}", originator.internal_data);
    println!("=============Done==============\n");

    println!("=============Shit Singleton==============");
    // shit ""singleton""
    // likely not even created due to optimization -> inlining data due to compile time evaluation
    unsafe { println!("{}", BADSINGLETON.data) };
    // aka will be likely this
    println!("kekw singleton");
    println!("=============Done==============\n");

    println!("=============Monostate Singleton==============");
    let singleton_globi_mock = MonoGlobi::create(MockSingleton::create());
    singleton_globi_mock.singleton.do_operation();
    // not really "created" -> static instance
    let singleton_globi = MonoGlobi::create(TrueSingleton::create());
    let same_globi_singleton = MonoGlobi::create(TrueSingleton::create());
    singleton_globi.singleton.do_operation();
    debug_assert!(eq::<dyn SingletonMono>(
        singleton_globi.singleton,
        same_globi_singleton.singleton
    ));
    println!("=============Done==============\n");

    println!("=============Factory Method==============");
    let penguin_factory = PenguinFactory::create();
    let product1 = penguin_factory.create_product(ProductType::PenguinOS);
    product1.print();
    let product2 = penguin_factory.create_product(ProductType::PenguinPlush);
    product2.print();
    println!("=============Done==============\n");

    println!("=============Facade==============");
    let pc = PCFacade::create();
    pc.boot();
    println!("=============Done==============\n");

    println!("=============Template Method==============");
    let template_method = TemplateImplementation::create();
    template_method.algorithm();
    println!("=============Done==============\n");

    println!("=============Prototype==============");
    let prototype = PrototypeImplementation::create();
    // this is not the standard clone btw.... lel
    dbg!(&prototype);
    let cloneroni = prototype.clone();
    dbg!(&cloneroni);
    println!("=============Done==============\n");

    println!("=============Whole Value==============");
    let date1 = Date::create(2025, 12, 4);
    assert_eq!(date1, None);
    let date2 = Date::create(2024, 12, 4);
    assert!(date2.is_some());
    println!("done");
    println!("=============Done==============\n");

    println!("=============Value Object==============");
    let value_object = Value1::create(12, "Something".into(), vec![1, 2, 4, 2]);
    let value_object2 = Value1::create(12, "Something".into(), vec![1, 2, 4, 2]);
    assert!(value_object.equal(&value_object2));
    println!("done");
    println!("=============Done==============\n");

    println!("=============Mutable Companion==============");
    let immutable_value = ImmutableValue::create(100); 
    let mut mutable_companion = MutableCompanion::create(immutable_value);
    mutable_companion.increment_value();
    mutable_companion.increment_value();
    let new_val = mutable_companion.as_value();
    assert_eq!(new_val.get_value(), 102);
    println!("done");
    println!("=============Done==============\n");

    println!("=============Relative Value==============");
    let val1 = Value2 {val: 100};
    let val2 = Value2 {val: 105};
    assert_eq!(val1.cmp(&val2), std::cmp::Ordering::Less);
    println!("done");
    println!("=============Done==============\n");

    println!("=============Abstract Factory Method==============");
    let abstract_factory = PenguinAbstractFactory::create();
    let plush = abstract_factory.create_plush();
    plush.print();
    let os = abstract_factory.create_os();
    os.print();
    println!("=============Done==============\n");

    println!("=============Object States==============");
    // state
    let mut grengeng = Context {
        state: Box::new(StateStart {}),
    };
    grengeng.state.operation();
    grengeng.state = Box::new(StateEnd {});
    grengeng.state.operation();
    println!("=============Done==============\n");

    println!("=============Adapter==============");
    let adapter = Wine::create(APenguinOS::create());
    adapter.bsod();
    println!("=============Done==============\n");

    println!("=============Method States==============");
    // method states
    let mut grief = What::what_grief();
    grief.op1();
    grief.op2();
    println!("val: {}", grief.val);
    grief.op3();
    println!("val: {}", grief.val);
    grief.change_mode();
    grief.op1();
    grief.op2();
    println!("val: {}", grief.val);
    grief.op3();
    println!("val: {}", grief.val);
    println!("=============Done==============\n");

    println!("=============External Iterator==============");
    let globus = GlobalGlobi::<i32>::create(vec![1, 5, 1, 9, 7]);
    let mut iter = globus.create_iterator();
    println!("{}", iter.next());
    println!("{}", iter.next());
    println!("{}", iter.previous());
    println!("{}", iter.has_next());
    println!("{}", iter.has_previous());
    println!("=============Done==============\n");

    println!("=============Internal Iterator==============");
    let mut internal_globi = InternalGlobi::create(vec![2, 1, 4, 8, 9]);
    dbg!(&internal_globi.data);
    internal_globi.for_each(|x| *x += 1);
    dbg!(&internal_globi.data);
    println!("=============Done==============\n");

    println!("=============Strategy==============");
    // strategy
    let mut thing = ContainerThing::create(vec![2, 1, 4, 9, 0], Box::new(StrategyPeng {}));
    thing.operation();
    thing.strategy = Box::new(StrategyGreng {});
    thing.operation();
    println!("=============Done==============\n");

    println!("=============Bridge==============");
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
    println!("=============Done==============\n");

    println!("=============Visitor with Composite==============");
    // visitor with composite
    let visitor = Rc::new(Visitor {});
    let tree = Node {
        children: vec![Rc::new(Node {
            children: vec![Rc::new(Leaf { val: 2 }), Rc::new(Leaf { val: 3 })],
        })],
    };
    tree.accept(visitor);
    println!("=============Done==============\n");

    println!("=============Builder==============");
    // builder
    let pone = PoneBuilder::new()
        .height(20)
        .weight(20)
        .name(String::from("Dashie"))
        .best_pone(true)
        .build();
    pone.display();
    println!("=============Done==============\n");

    println!("=============Command==============");
    // command
    let peng = Box::new(Peng {});
    let mut command = PrintCommand { receiver: peng };
    // invoke
    command.exec();
    let globi = Box::new(Globi {});
    command.receiver = globi;
    command.exec();
    println!("=============Done==============\n");

    println!("=============Command Processor==============");
    // command processor
    let new_peng = Box::new(ProcessorPeng {});
    let new_globi = Box::new(ProcessorGlobi {});
    let mut command_processor = CommandProcessor::create();
    command_processor.do_command(Box::new(ProcessorPrintCommand { receiver: new_peng }));
    command_processor.do_command(Box::new(ProcessorPrintCommand {
        receiver: new_globi,
    }));
    command_processor.undo_command();
    command_processor.undo_command();
    println!("=============Done==============\n");

    println!("=============Flyweight==============");
    // Flyweight
    let factory = FlyWeightFactory::new();
    let tree = Box::new(Tree { height: 20, age: 1 });
    let tree2 = Box::new(Tree {
        height: 50,
        age: 200,
    });
    let flyweight = factory.get_flyweight("default");
    flyweight.operation(tree);
    let other = factory.get_flyweight("default");
    other.operation(tree2);
    assert_eq!(flyweight, other);
    println!("=============Done==============\n");

    println!("=============Observer==============");
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
    println!("=============Done==============\n");

    println!("=============Decorator==============");
    let decorator_tree = Decorator::create(Decorator::create(DecoratorLeaf::create(4)));
    decorator_tree.operation();
    println!("=============Done==============\n");
}
