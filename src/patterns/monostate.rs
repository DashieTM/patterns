pub trait SingletonMono {
    fn get_instance(&self) -> Box<dyn SingletonMono>;
    fn do_operation(&self);
}

pub struct TrueSingleton {
    pub data: i32,
}

impl TrueSingleton {
    const INSTANCE: Self = Self { data: 200 };

    pub fn create() -> Box<Self> {
        Box::new(TrueSingleton::INSTANCE)
    }
}

impl SingletonMono for TrueSingleton {
    fn get_instance(&self) -> Box<dyn SingletonMono> {
        Box::new(TrueSingleton::INSTANCE)
    }

    fn do_operation(&self) {
        println!("This is a true singleton");
    }
}

// for example for testing
pub struct MockSingleton {
    pub data: i32,
}

impl MockSingleton {
    pub fn create() -> Box<Self> {
        Box::new(Self { data: 400 })
    }
}

impl SingletonMono for MockSingleton {
    fn get_instance(&self) -> Box<dyn SingletonMono> {
        Box::new(MockSingleton { data: 400 })
    }

    fn do_operation(&self) {
        println!("This is a mock singleton");
    }
}

pub struct MonoGlobi {
    pub singleton: Box<dyn SingletonMono>,
}

impl MonoGlobi {
    pub fn create(singleton: Box<dyn SingletonMono>) -> Self {
        Self { singleton }
    }
}
