use std::{cell::RefCell, pin::Pin};

use crate::{inner, model::Model, pin_ptr, sbmlcxx, upcast_annotation};

/// A safe wrapper around the libSBML ListOfReactions class.
///
/// This struct maintains a reference to the underlying C++ ListOfReactions object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct ListOfReactions<'a> {
    /// The underlying libSBML Model pointer wrapped in RefCell and Pin
    inner: RefCell<Pin<&'a mut sbmlcxx::ListOfReactions>>,
}

impl<'a> ListOfReactions<'a> {
    pub fn new(model: &'a Model<'a>) -> Self {
        let reactions_ptr = model.inner().borrow_mut().as_mut().getListOfReactions1();
        let reactions = pin_ptr!(reactions_ptr, sbmlcxx::ListOfReactions);

        Self {
            inner: RefCell::new(reactions),
        }
    }
}

// Derive the inner type from the ListOfReactions type
inner!(sbmlcxx::ListOfReactions, ListOfReactions<'a>);
upcast_annotation!(
    ListOfReactions<'a>,
    sbmlcxx::ListOfReactions,
    sbmlcxx::SBase
);
