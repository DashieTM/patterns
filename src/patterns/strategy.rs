pub struct ContainerThing {
    pub container: Vec<i32>,
    pub strategy: Box<dyn Strategy>,
}

impl ContainerThing {
    pub fn create(container: Vec<i32>, strategy: Box<dyn Strategy>) -> Self {
        Self {
            container,
            strategy,
        }
    }
    
    pub fn operation(&self) {
        self.strategy.algorithm();
    }
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
