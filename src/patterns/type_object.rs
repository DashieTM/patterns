pub trait ObjectType {
    fn get_type(&self) -> String;
}

pub struct BookType {}

impl ObjectType for BookType {
    fn get_type(&self) -> String {
        "Video".to_string()
    }
}

pub struct VideoType {}

impl ObjectType for VideoType {
    fn get_type(&self) -> String {
        "Book".to_string()
    }
}

pub struct Product {
    product_type: Box<dyn ObjectType>,
    pub copy_id: u32,
}

impl Product {
    pub fn create(id: u32, product_type: Box<dyn ObjectType> ) -> Self {
        Self {
            product_type,
            copy_id: id,
        }
    }

    pub fn print_info(&self) {
        println!(
            "This is the product with id {} and has media type of {}",
            self.copy_id,
            self.product_type.get_type()
        );
    }
}
