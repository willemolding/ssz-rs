pub enum Error {
    NoInnerElement,
    InvalidInnerIndex,
}

pub trait Visitable<V: Visitor> {
    fn visit_element(&self, index: usize, visitor: &mut V) -> Result<(), Error> {
        Err(Error::NoInnerElement)
    }
}

pub trait Visitor: Sized {
    fn visit<T: Visitable<Self> + ?Sized>(&mut self, element: &T) -> Result<(), Error>;
}
