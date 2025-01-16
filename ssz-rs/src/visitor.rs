use crate::SimpleSerialize;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    NoInnerElement,
    InvalidInnerIndex,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoInnerElement => write!(
                f,
                "requested to compute proof for an inner element which does not exist for this type"
            ),
            Self::InvalidInnerIndex => write!(
                f,
                "requested to compute proof for an inner element outside the bounds of what this type supports"
            ),
        }
    }
}

pub trait Visitable<V: Visitor> {
    fn visit_element(&self, index: usize, visitor: &mut V) -> Result<(), V::Error> {
        Err(Error::NoInnerElement.into())
    }
}

pub trait Visitor: Sized {
    type Error: From<Error>;

    fn visit<T: SimpleSerialize + Visitable<Self> + ?Sized>(
        &mut self,
        element: &T,
    ) -> Result<(), Self::Error>;
}
