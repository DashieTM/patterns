pub struct BadSingleton {
    pub data: &'static str,
}

pub static mut BADSINGLETON: BadSingleton = BadSingleton {
    data: "kekw singleton",
};
