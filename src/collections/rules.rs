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

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::sbmldoc::SBMLDocument;

    #[test]
    fn test_list_of_rules_annotation_serde() {
        let doc = SBMLDocument::default();
        let model = doc.create_model("test");

        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let annotation = TestAnnotation {
            test: "Test".to_string(),
        };

        model.set_rate_rules_annotation_serde(&annotation).unwrap();

        let annotation: TestAnnotation = model.get_rate_rules_annotation_serde().unwrap();
        assert_eq!(annotation.test, "Test");
    }

    #[test]
    fn test_list_of_rules_annotation() {
        let doc = SBMLDocument::default();
        let model = doc.create_model("test");

        let annotation = "<test>Test</test>";
        model
            .set_rate_rules_annotation(annotation)
            .expect("Failed to set annotation");

        let annotation = model.get_rate_rules_annotation();
        assert_eq!(
            annotation
                .replace("\n", "")
                .replace("\r", "")
                .replace(" ", ""),
            "<annotation><test>Test</test></annotation>"
        );
    }
}
