use std::{cell::RefCell, pin::Pin};

use crate::{inner, model::Model, pin_ptr, sbmlcxx, upcast_annotation};

/// A safe wrapper around the libSBML ListOfSpecies class.
///
/// This struct maintains a reference to the underlying C++ ListOfSpecies object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct ListOfSpecies<'a> {
    /// The underlying libSBML Model pointer wrapped in RefCell and Pin
    inner: RefCell<Pin<&'a mut sbmlcxx::ListOfSpecies>>,
}

impl<'a> ListOfSpecies<'a> {
    pub fn new(model: &'a Model<'a>) -> Self {
        let species_ptr = model.inner().borrow_mut().as_mut().getListOfSpecies1();
        let species = pin_ptr!(species_ptr, sbmlcxx::ListOfSpecies);

        Self {
            inner: RefCell::new(species),
        }
    }
}

// Derive the inner type from the ListOfSpecies type
inner!(sbmlcxx::ListOfSpecies, ListOfSpecies<'a>);
upcast_annotation!(ListOfSpecies<'a>, sbmlcxx::ListOfSpecies, sbmlcxx::SBase);
