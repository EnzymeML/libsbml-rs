use std::{cell::RefCell, pin::Pin};

use crate::{inner, model::Model, pin_ptr, sbmlcxx, upcast_annotation};

/// A safe wrapper around the libSBML ListOfCompartments class.
///
/// This struct maintains a reference to the underlying C++ ListOfCompartments object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct ListOfUnitDefinitions<'a> {
    /// The underlying libSBML Model pointer wrapped in RefCell and Pin
    inner: RefCell<Pin<&'a mut sbmlcxx::ListOfUnitDefinitions>>,
}

impl<'a> ListOfUnitDefinitions<'a> {
    pub fn new(model: &'a Model<'a>) -> Self {
        let unitdefs_ptr = model
            .inner()
            .borrow_mut()
            .as_mut()
            .getListOfUnitDefinitions1();
        let unitdefs = pin_ptr!(unitdefs_ptr, sbmlcxx::ListOfUnitDefinitions);

        Self {
            inner: RefCell::new(unitdefs),
        }
    }
}

// Derive the inner type from the ListOfUnitDefinitions type
inner!(sbmlcxx::ListOfUnitDefinitions, ListOfUnitDefinitions<'a>);
upcast_annotation!(
    ListOfUnitDefinitions<'a>,
    sbmlcxx::ListOfUnitDefinitions,
    sbmlcxx::SBase
);
