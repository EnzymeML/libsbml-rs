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

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::sbmldoc::SBMLDocument;

    #[test]
    fn test_list_of_species_annotation_serde() {
        let doc = SBMLDocument::default();
        let model = doc.create_model("test");

        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let annotation = TestAnnotation {
            test: "Test".to_string(),
        };

        model.set_species_annotation_serde(&annotation).unwrap();

        let annotation: TestAnnotation = model.get_species_annotation_serde().unwrap();
        assert_eq!(annotation.test, "Test");
    }

    #[test]
    fn test_list_of_species_annotation() {
        let doc = SBMLDocument::default();
        let model = doc.create_model("test");

        let annotation = "<test>Test</test>";
        model
            .set_species_annotation(annotation)
            .expect("Failed to set annotation");

        let annotation = model.get_species_annotation();
        assert_eq!(
            annotation
                .replace("\n", "")
                .replace("\r", "")
                .replace(" ", ""),
            "<annotation><test>Test</test></annotation>"
        );
    }
}
