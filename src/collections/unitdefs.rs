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

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::sbmldoc::SBMLDocument;

    #[test]
    fn test_list_of_unitdefs_annotation_serde() {
        let doc = SBMLDocument::default();
        let model = doc.create_model("test");

        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let annotation = TestAnnotation {
            test: "Test".to_string(),
        };

        model
            .set_unit_definitions_annotation_serde(&annotation)
            .unwrap();

        let annotation: TestAnnotation = model.get_unit_definitions_annotation_serde().unwrap();
        assert_eq!(annotation.test, "Test");
    }

    #[test]
    fn test_list_of_unitdefs_annotation() {
        let doc = SBMLDocument::default();
        let model = doc.create_model("test");

        let annotation = "<test>Test</test>";
        model
            .set_unit_definitions_annotation(annotation)
            .expect("Failed to set annotation");

        let annotation = model.get_unit_definitions_annotation();
        assert_eq!(
            annotation
                .replace("\n", "")
                .replace("\r", "")
                .replace(" ", ""),
            "<annotation><test>Test</test></annotation>"
        );
    }
}
