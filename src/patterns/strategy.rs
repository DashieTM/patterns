pub struct ContainerThing {
    pub container: Vec<i32>,
    pub strategy: Box<dyn Strategy>,
}

pub trait Strategy {
    fn algorithm(&self) {}
}

pub struct StrategyGreng {}

impl Strategy for StrategyGreng {
    fn algorithm(&self) {
        println!("grengeng");
    }
}

pub struct StrategyPeng {}

impl Strategy for StrategyPeng {
    fn algorithm(&self) {
        println!("dodododo");
    }
}
