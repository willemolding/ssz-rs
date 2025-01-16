use crate::SimpleSerialize;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum VisitorError {
    NoInnerElement,
    InvalidInnerIndex,
}

impl Display for VisitorError {
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

pub trait Visitable {
    fn visit_element<V: Visitor>(&self, index: usize, visitor: &mut V) -> Result<(), V::Error> {
        Err(VisitorError::NoInnerElement.into())
    }
}

pub trait Visitor: Sized {
    type Error: From<VisitorError>;

    fn visit<T: SimpleSerialize + Visitable + ?Sized>(
        &mut self,
        element: &T,
    ) -> Result<(), Self::Error>;
}
