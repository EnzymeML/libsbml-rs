use std::{cell::RefCell, pin::Pin};

use crate::{inner, model::Model, pin_ptr, sbmlcxx, upcast_annotation};

/// A safe wrapper around the libSBML ListOfCompartments class.
///
/// This struct maintains a reference to the underlying C++ ListOfCompartments object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct ListOfCompartments<'a> {
    /// The underlying libSBML Model pointer wrapped in RefCell and Pin
    inner: RefCell<Pin<&'a mut sbmlcxx::ListOfCompartments>>,
}

impl<'a> ListOfCompartments<'a> {
    pub fn new(model: &'a Model<'a>) -> Self {
        let compartments_ptr = model.inner().borrow_mut().as_mut().getListOfCompartments1();
        let compartments = pin_ptr!(compartments_ptr, sbmlcxx::ListOfCompartments);

        Self {
            inner: RefCell::new(compartments),
        }
    }
}

// Derive the inner type from the ListOfCompartments type
inner!(sbmlcxx::ListOfCompartments, ListOfCompartments<'a>);
upcast_annotation!(
    ListOfCompartments<'a>,
    sbmlcxx::ListOfCompartments,
    sbmlcxx::SBase
);
