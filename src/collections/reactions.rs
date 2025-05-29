use std::{cell::RefCell, pin::Pin};

use crate::{inner, model::Model, pin_ptr, sbase, sbmlcxx, upcast_annotation};

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
sbase!(ListOfReactions<'a>, sbmlcxx::ListOfReactions);
upcast_annotation!(
    ListOfReactions<'a>,
    sbmlcxx::ListOfReactions,
    sbmlcxx::SBase
);

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::sbmldoc::SBMLDocument;

    #[test]
    fn test_list_of_reactions_annotation_serde() {
        let doc = SBMLDocument::default();
        let model = doc.create_model("test");

        #[derive(Serialize, Deserialize)]
        struct TestAnnotation {
            test: String,
        }

        let annotation = TestAnnotation {
            test: "Test".to_string(),
        };

        model.set_reactions_annotation_serde(&annotation).unwrap();

        let annotation: TestAnnotation = model.get_reactions_annotation_serde().unwrap();
        assert_eq!(annotation.test, "Test");
    }

    #[test]
    fn test_list_of_reactions_annotation() {
        let doc = SBMLDocument::default();
        let model = doc.create_model("test");

        let annotation = "<test>Test</test>";
        model
            .set_reactions_annotation(annotation)
            .expect("Failed to set annotation");

        let annotation = model.get_reactions_annotation();
        assert_eq!(
            annotation
                .replace("\n", "")
                .replace("\r", "")
                .replace(" ", ""),
            "<annotation><test>Test</test></annotation>"
        );
    }
}
