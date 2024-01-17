pub trait TFactoryProduct {
    fn print(&self);
}

pub struct PenguinPlush {}

impl TFactoryProduct for PenguinPlush {
    fn print(&self) {
        println!("fluffy af");
    }
}

pub struct PenguinOS {}

impl TFactoryProduct for PenguinOS {
    fn print(&self) {
        println!("I use arch btw");
    }
}

pub enum ProductType {
    PenguinPlush,
    PenguinOS
}

pub trait TPenguinFactory {
    fn create_product(&self, product: ProductType ) -> Box<dyn TFactoryProduct>;
}

pub struct PenguinFactory {}

impl PenguinFactory {
    pub fn create() -> Self {
        Self {  }
    }
}

impl TPenguinFactory for PenguinFactory {
    fn create_product(&self, product: ProductType ) -> Box<dyn TFactoryProduct> {
        match product {
            ProductType::PenguinPlush => Box::new(PenguinPlush {}),
            ProductType::PenguinOS => Box::new(PenguinOS {}),
        }
    }
}
