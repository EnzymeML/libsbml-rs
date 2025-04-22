use std::{cell::RefCell, pin::Pin};

use crate::{inner, model::Model, pin_ptr, sbmlcxx, upcast_annotation};

/// A safe wrapper around the libSBML ListOfParameters class.
///
/// This struct maintains a reference to the underlying C++ ListOfParameters object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct ListOfParameters<'a> {
    /// The underlying libSBML Model pointer wrapped in RefCell and Pin
    inner: RefCell<Pin<&'a mut sbmlcxx::ListOfParameters>>,
}

impl<'a> ListOfParameters<'a> {
    pub fn new(model: &'a Model<'a>) -> Self {
        let parameters_ptr = model.inner().borrow_mut().as_mut().getListOfParameters1();
        let parameters = pin_ptr!(parameters_ptr, sbmlcxx::ListOfParameters);

        Self {
            inner: RefCell::new(parameters),
        }
    }
}

// Derive the inner type from the ListOfParameters type
inner!(sbmlcxx::ListOfParameters, ListOfParameters<'a>);
upcast_annotation!(
    ListOfParameters<'a>,
    sbmlcxx::ListOfParameters,
    sbmlcxx::SBase
);
