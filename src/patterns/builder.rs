pub struct Pone {
    name: String,
    height: u32,
    weight: u32,
    cute: u32,
    best_pone: bool,
}

impl Pone {
    pub fn display(&self) {
        println!("This is {}, with height and weight of {}, {}, is {} cute, and it is it the best pone? {}", self.name, self.height, self.weight, self.cute, self.best_pone);
    }
}

pub trait TPoneBuilder {
    fn build(&self) -> Pone;
    fn name(&mut self, name: String) -> &mut PoneBuilder;
    fn height(&mut self, height: u32) -> &mut PoneBuilder;
    fn weight(&mut self, weight: u32) -> &mut PoneBuilder;
    fn cute(&mut self, cute: u32) -> &mut PoneBuilder;
    fn best_pone(&mut self, best_pone: bool) -> &mut PoneBuilder;
}

pub struct PoneBuilder {
    name: Option<String>,
    height: Option<u32>,
    weight: Option<u32>,
    cute: Option<u32>,
    best_pone: Option<bool>,
}

impl PoneBuilder {
    pub fn new() -> Self {
        PoneBuilder {
            name: None,
            height: None,
            weight: None,
            cute: None,
            best_pone: None,
        }
    }
}

impl Default for PoneBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TPoneBuilder for PoneBuilder {
    fn build(&self) -> Pone {
        Pone {
            name: self.name.clone().unwrap_or(String::from("noname")),
            height: self.height.unwrap_or(1),
            weight: self.weight.unwrap_or(1),
            cute: self.cute.unwrap_or(1),
            best_pone: self.best_pone.unwrap_or(false),
        }
    }

    fn name(&mut self, name: String) -> &mut PoneBuilder {
        self.name = Some(name);
        self
    }

    fn height(&mut self, height: u32) -> &mut PoneBuilder {
        self.height = Some(height);
        self
    }

    fn weight(&mut self, weight: u32) -> &mut PoneBuilder {
        self.weight = Some(weight);
        self
    }

    fn cute(&mut self, cute: u32) -> &mut PoneBuilder {
        self.cute = Some(cute);
        self
    }

    fn best_pone(&mut self, best_pone: bool) -> &mut PoneBuilder {
        self.best_pone = Some(best_pone);
        self
    }
}
