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

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::sbmldoc::SBMLDocument;

    #[test]
    fn test_list_of_parameters_annotation_serde() {
        let doc = SBMLDocument::default();
        let model = doc.create_model("test");

        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let annotation = TestAnnotation {
            test: "Test".to_string(),
        };

        model.set_parameters_annotation_serde(&annotation).unwrap();

        let annotation: TestAnnotation = model.get_parameters_annotation_serde().unwrap();
        assert_eq!(annotation.test, "Test");
    }

    #[test]
    fn test_list_of_parameters_annotation() {
        let doc = SBMLDocument::default();
        let model = doc.create_model("test");

        let annotation = "<test>Test</test>";
        model
            .set_parameters_annotation(annotation)
            .expect("Failed to set annotation");

        let annotation = model.get_parameters_annotation();
        assert_eq!(
            annotation
                .replace("\n", "")
                .replace("\r", "")
                .replace(" ", ""),
            "<annotation><test>Test</test></annotation>"
        );
    }
}
