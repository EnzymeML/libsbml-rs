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

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::sbmldoc::SBMLDocument;

    #[test]
    fn test_list_of_compartments_annotation_serde() {
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
            .set_compartments_annotation_serde(&annotation)
            .unwrap();

        let annotation: TestAnnotation = model.get_compartments_annotation_serde().unwrap();
        assert_eq!(annotation.test, "Test");
    }

    #[test]
    fn test_list_of_compartments_annotation() {
        let doc = SBMLDocument::default();
        let model = doc.create_model("test");

        let annotation = "<test>Test</test>";
        model
            .set_compartments_annotation(annotation)
            .expect("Failed to set annotation");

        let annotation = model.get_compartments_annotation();
        assert_eq!(
            annotation
                .replace("\n", "")
                .replace("\r", "")
                .replace(" ", ""),
            "<annotation><test>Test</test></annotation>"
        );
    }
}
