pub trait TAggregate<'a, I, T> {
    fn create_iterator(&'a self) -> I;
}

pub trait TIterator<T> {
    fn next(&mut self) -> &T;
    fn previous(&mut self) -> &T;
    fn has_next(&self) -> bool;
    fn has_previous(&self) -> bool;
}

#[derive(Clone)]
pub struct GlobalGlobi<'a, T> {
    pub data: Vec<T>,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, T> GlobalGlobi<'a, T> {
    pub fn create(data: Vec<T>) -> Self {
        Self {
            data,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, T> TAggregate<'a, IteratorGlobalGlobi<'a, T>, T> for GlobalGlobi<'a, T> {
    fn create_iterator(&'a self) -> IteratorGlobalGlobi<'a, T> {
        IteratorGlobalGlobi::<'a, T> {
            globi: self,
            index: 0,
            start: true,
        }
    }
}

pub struct IteratorGlobalGlobi<'a, T> {
    pub globi: &'a GlobalGlobi<'a, T>,
    pub index: usize,
    start: bool,
}

impl<'a, T> TIterator<T> for IteratorGlobalGlobi<'a, T> {
    fn next(&mut self) -> &T {
        if self.start {
            self.start = false;
        } else {
            self.index += 1;
        }
        unsafe { self.globi.data.get_unchecked(self.index) }
    }

    fn previous(&mut self) -> &T {
        if self.start {
            self.start = false;
        } else {
            match self.index != 0 {
                true => {
                    self.index -= 1;
                }
                false => (),
            }
        }
        unsafe { self.globi.data.get_unchecked(self.index) }
    }

    fn has_next(&self) -> bool {
        let opt = self.globi.data.get(self.index + 1);
        if opt.is_some() {
            return true;
        }
        false
    }

    fn has_previous(&self) -> bool {
        if self.index == 0 {
            return false;
        }
        let opt = self.globi.data.get(self.index - 1);
        if opt.is_some() {
            return true;
        }
        false
    }
}
