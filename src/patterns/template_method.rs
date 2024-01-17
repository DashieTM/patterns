pub trait TTemplate {
    fn to_implement(&self);

    fn algorithm(&self) {
        self.to_implement();
        self.globus();
        self.maximus();
    }

    fn globus(&self) {
        println!("I am a globi");
    }

    fn maximus(&self) {
        println!("and a maximal one at that");
    }
}

pub struct TemplateImplementation {}

impl TemplateImplementation {
    pub fn create() -> Self {
        Self {}
    }
}

impl TTemplate for TemplateImplementation {
    fn to_implement(&self) {
        println!("this is the one function that you needed to implement");
    }
}
