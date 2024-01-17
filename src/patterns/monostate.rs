use std::sync::Arc;
use once_cell::sync::Lazy;

pub static INSTANCE: Lazy<Arc<TrueSingleton>> = Lazy::new(|| Arc::new(TrueSingleton { data: 200 }));

pub trait SingletonMono: std::fmt::Debug {
    fn get_instance(&self) -> Arc<dyn SingletonMono>;
    fn do_operation(&self);
}

#[derive(Debug)]
pub struct TrueSingleton {
    pub data: i32,
}

impl TrueSingleton {
    pub fn create() -> Arc<Self> {
        INSTANCE.clone()
    }
}

impl SingletonMono for TrueSingleton {
    fn get_instance(&self) -> Arc<dyn SingletonMono> {
        INSTANCE.clone()
    }

    fn do_operation(&self) {
        println!("This is a true singleton");
    }
}

// for example for testing
#[derive(Debug)]
pub struct MockSingleton {
    pub data: i32,
}

impl MockSingleton {
    pub fn create() -> Arc<Self> {
        Arc::new(Self { data: 400 })
    }
}

impl SingletonMono for MockSingleton {
    fn get_instance(&self) -> Arc<dyn SingletonMono> {
        Arc::new(MockSingleton { data: 400 })
    }

    fn do_operation(&self) {
        println!("This is a mock singleton");
    }
}

pub struct MonoGlobi {
    pub singleton: Arc<dyn SingletonMono>,
}

impl MonoGlobi {
    pub fn create(singleton: Arc<dyn SingletonMono>) -> Self {
        Self { singleton }
    }
}

pub fn eq<T: ?Sized>(left: Arc<T>, right: Arc<T>) -> bool {
    let left: *const T = left.as_ref();
    let right: *const T = right.as_ref();
    let ret = left == right;
    dbg!(left);
    dbg!(right);
    ret
}
