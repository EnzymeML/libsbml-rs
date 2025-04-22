use std::{cell::RefCell, pin::Pin};

use crate::{inner, model::Model, pin_ptr, sbmlcxx, upcast_annotation};

/// A safe wrapper around the libSBML ListOfRules class.
///
/// This struct maintains a reference to the underlying C++ ListOfRules object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct ListOfRules<'a> {
    /// The underlying libSBML Model pointer wrapped in RefCell and Pin
    inner: RefCell<Pin<&'a mut sbmlcxx::ListOfRules>>,
}

impl<'a> ListOfRules<'a> {
    pub fn new(model: &'a Model<'a>) -> Self {
        let rules_ptr = model.inner().borrow_mut().as_mut().getListOfRules1();
        let rules = pin_ptr!(rules_ptr, sbmlcxx::ListOfRules);

        Self {
            inner: RefCell::new(rules),
        }
    }
}

// Derive the inner type from the ListOfRules type
inner!(sbmlcxx::ListOfRules, ListOfRules<'a>);
upcast_annotation!(ListOfRules<'a>, sbmlcxx::ListOfRules, sbmlcxx::SBase);
