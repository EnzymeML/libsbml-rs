/// Creates a pinned reference from a raw pointer.
///
/// This macro takes a raw pointer and type, and creates a pinned mutable reference
/// to that type. It is used internally to safely work with C++ objects that must
/// not be moved in memory.
///
/// # Arguments
/// * `$name` - The raw pointer to convert
/// * `$type` - The type to convert the pointer to
///
/// # Safety
/// This macro uses unsafe code to create the reference and pin it. The caller must
/// ensure that:
/// - The pointer is valid and properly aligned
/// - The pointer points to an initialized value of the specified type
/// - The lifetime of the reference does not outlive the pointed-to data
#[macro_export]
macro_rules! pin_ptr {
    ($name:ident, $type:ty) => {{
        let ref_ptr: &mut $type = unsafe { &mut *$name };
        unsafe { Pin::new_unchecked(ref_ptr) }
    }};
}

/// Performs a safe upcast from one SBML type to another.
///
/// This macro takes an SBML object and performs an upcast from a derived type to a base type.
/// It is used internally to safely convert between SBML types that have an inheritance relationship,
/// such as converting a Species to an SBase.
///
/// # Arguments
/// * `$name` - The SBML object to upcast
/// * `$from` - The source type to convert from
/// * `$to` - The target type to convert to
///
/// # Safety
/// This macro uses unsafe code to perform the upcast. The caller must ensure that:
/// - The types have a valid inheritance relationship (e.g. Species -> SBase)
/// - The object being upcast is a valid instance of the source type
/// - The lifetime of the upcast reference matches the original object
#[macro_export]
macro_rules! upcast {
    ($name:expr, $from:ty, $to:ty) => {{
        use crate::cast::upcast;
        let mut borrow = $name.inner().borrow_mut();
        let ptr = unsafe { borrow.as_mut().get_unchecked_mut() };
        unsafe { upcast::<$from, $to>(ptr) }
    }};
}

/// Performs a safe upcast from one pinned SBML type to another.
///
/// This macro takes a pinned SBML object and performs an upcast from a derived type to a base type.
/// It is similar to the `upcast!` macro but works directly with pinned references rather than
/// going through a RefCell. This is useful when you already have a pinned mutable reference
/// and need to upcast it to a base type.
///
/// # Arguments
/// * `$name` - The pinned SBML object to upcast
/// * `$from` - The source type to convert from
/// * `$to` - The target type to convert to
///
/// # Safety
/// This macro uses unsafe code to perform the upcast. The caller must ensure that:
/// - The types have a valid inheritance relationship (e.g. Species -> SBase)
/// - The object being upcast is a valid instance of the source type
/// - The pinned reference maintains its pinning guarantees
/// - The lifetime of the upcast reference matches the original object
#[macro_export]
macro_rules! upcast_pin {
    ($name:expr, $from:ty, $to:ty) => {{
        use crate::cast::upcast;
        let ptr = unsafe { $name.as_mut().get_unchecked_mut() };
        unsafe { upcast::<$from, $to>(ptr) }
    }};
}

/// Implements the Inner trait for a wrapper type.
///
/// This macro generates an implementation of the Inner trait for a wrapper type that contains
/// a pinned reference to a C++ object. The Inner trait provides a consistent interface for
/// accessing the inner pinned data across different wrapper types in the SBML library.
///
/// # Arguments
/// * `$cxx_type` - The C++ type that is being wrapped (e.g. sbmlcxx::Species)
/// * `$type` - The Rust wrapper type (e.g. Species<'a>)
///
/// This will generate an implementation of Inner<'a, sbmlcxx::Species> for Species<'a>
/// that provides access to the inner RefCell containing the pinned pointer.
///
/// The generated implementation ensures that:
/// - The wrapper type properly implements the Inner trait
/// - The inner pinned data can be accessed in a controlled manner
/// - The lifetime parameter 'a is properly propagated
/// - The RefCell provides interior mutability while maintaining Rust's borrowing rules
#[macro_export]
macro_rules! inner {
    ($cxx_type:ty, $type:ty) => {
        // Import necessary modules
        use crate::traits::inner::Inner;

        /// Implementation of the Inner trait for $type.
        ///
        /// This allows access to the inner RefCell containing the pinned $type pointer.
        impl<'a> Inner<'a, $cxx_type> for $type {
            fn inner(&self) -> &RefCell<Pin<&'a mut $cxx_type>> {
                &self.inner
            }
        }
    };
}
/// Implements the Annotation trait for a wrapper type.
///
/// This macro generates an implementation of the Annotation trait for a wrapper type that contains
/// a pinned reference to a C++ object. The Annotation trait provides a consistent interface for
/// accessing and modifying XML annotations across different wrapper types in the SBML library.
///
/// # Arguments
/// * `$type` - The Rust wrapper type (e.g. Species<'a>)
/// * `$cxx_type` - The C++ type that is being wrapped (e.g. sbmlcxx::Species)
/// * `$cxx_upcast` - The C++ base type to upcast to (e.g. sbmlcxx::SBase)
///
/// This will generate an implementation of the Annotation trait for Species<'a> that provides:
/// - get_annotation() - Gets the raw XML annotation string
/// - set_annotation() - Sets the XML annotation from a string
/// - get_annotation_serde() - Gets the annotation deserialized into a specified type
/// - set_annotation_serde() - Sets the annotation by serializing a type to XML
///
/// The generated implementation ensures that:
/// - XML annotations can be accessed and modified in a type-safe way
/// - Serialization/deserialization is handled consistently
/// - The C++ object is properly upcast to access base class annotation methods
/// - Interior mutability is maintained through RefCell
#[macro_export]
macro_rules! upcast_annotation {
    ($type:ty, $cxx_type:ty, $cxx_upcast:ty) => {
        // Import necessary modules
        use crate::traits::annotation::Annotation;
        use crate::wrapper::Wrapper;

        use quick_xml::{de::from_str, se::to_string, DeError, SeError};
        use serde::{Deserialize, Serialize};
        use std::error::Error;

        impl<'a> Annotation for $type {
            /// Gets the annotation for the compartment.
            ///
            /// We are using upcasting to access the base class's getAnnotationString method.
            ///
            /// # Returns
            /// The compartment's annotation as a String in XML format
            fn get_annotation(&self) -> String {
                let base = crate::upcast!(self, $cxx_type, $cxx_upcast);
                base.getAnnotationString().to_str().unwrap().to_string()
            }

            /// Sets the annotation for the compartment.
            ///
            /// We are using upcasting to access the base class's setAnnotation1 method.
            ///
            /// # Arguments
            /// * `annotation` - A string slice containing the XML annotation to set
            ///
            /// # Returns
            /// Result indicating success or containing an error if the annotation is invalid
            fn set_annotation(&self, annotation: &str) -> Result<(), Box<dyn Error>> {
                let mut base = crate::upcast!(self, $cxx_type, $cxx_upcast);
                cxx::let_cxx_string!(annotation = annotation);
                base.as_mut().setAnnotation1(&annotation);
                Ok(())
            }

            /// Sets a serializable annotation for the compartment.
            ///
            /// # Arguments
            /// * `annotation` - A reference to a type implementing Serialize that will be converted to XML
            ///
            /// # Returns
            /// Result indicating success or containing a serialization error
            fn set_annotation_serde<T: Serialize>(&self, annotation: &T) -> Result<(), SeError> {
                let annotation = to_string(annotation)?;
                self.set_annotation(&annotation)
                    .map_err(|e| SeError::Custom(e.to_string()))?;
                Ok(())
            }

            /// Gets the annotation as a deserialized type.
            ///
            /// # Type Parameters
            /// * `T` - The type to deserialize the annotation into
            ///
            /// # Returns
            /// Result containing the deserialized type or a deserialization error
            fn get_annotation_serde<T: for<'de> Deserialize<'de>>(&self) -> Result<T, DeError> {
                let annotation = self.get_annotation();
                let parsed: Wrapper<T> = from_str(&annotation)?;
                Ok(parsed.annotation)
            }
        }
    };
}

/// A macro for generating SBO (Systems Biology Ontology) term related methods.
///
/// This macro generates three methods for handling SBO terms:
/// - A getter method that returns the SBO term ID
/// - A getter method that returns the SBO term URL
/// - A setter method for setting the SBO term
///
/// The SBO provides controlled vocabularies of terms that can be used to indicate
/// the roles of model components in a standardized way.
///
/// # Arguments
/// * `$name` - The base name for the generated methods
/// * `$id` - The parameter name for the SBO term ID in the setter method
/// * `$url` - The parameter name for the URL (unused but kept for API consistency)
///
/// # Generated Methods
/// For a macro invocation `sbo_term!(sbo_term, id, url)`, it generates:
///
/// - `sbo_term(&self) -> String` - Gets the SBO term identifier
/// - `sbo_term_url(&self) -> String` - Gets the SBO term as a URL
/// - `set_sbo_term(&self, id: &str)` - Sets the SBO term using an identifier
#[macro_export]
macro_rules! sbo_term {
    ($cxx_type:ty, $cxx_upcast:ty) => {
        /// Gets the SBO term identifier.
        ///
        /// # Returns
        /// The SBO term ID as a String (e.g. "SBO:0000001")
        pub fn sbo_term_id(&self) -> String {
            let base = crate::upcast!(self, $cxx_type, $cxx_upcast);
            base.getSBOTermID().to_str().unwrap().to_string()
        }

        /// Gets the SBO term as a URL.
        ///
        /// # Returns
        /// The SBO term URL as a String (e.g. "http://biomodels.net/SBO/SBO_0000001")
        pub fn sbo_term_url(&self) -> String {
            let base = crate::upcast!(self, $cxx_type, $cxx_upcast);
            base.getSBOTermAsURL().to_str().unwrap().to_string()
        }

        /// Sets the SBO term using an identifier.
        ///
        /// # Arguments
        /// * `id` - The SBO term identifier to set (e.g. "SBO:0000001")
        pub fn set_sbo_term(&self, id: &str) {
            let mut base = crate::upcast!(self, $cxx_type, $cxx_upcast);
            cxx::let_cxx_string!(id = id);
            base.as_mut().setSBOTerm1(&id);
        }
    };
}

/// A macro for generating the `into_id` method for a wrapper type.
///
/// This macro generates an implementation of the `IntoId` trait for a wrapper type,
/// allowing the wrapper type to be converted into an identifier string.
///
/// # Arguments
/// * `$type` - The Rust wrapper type (e.g. Species<'a>)
/// * `$property` - The property to convert to an identifier string
///
/// This will generate an implementation of the `IntoId` trait for the wrapper type,
/// allowing the wrapper type to be converted into an identifier string.
#[macro_export]
macro_rules! into_id {
    ($type:ty, $property:ident) => {
        impl<'a> crate::traits::intoid::IntoId<'a> for $type {
            fn into_id(self) -> &'a str {
                Box::leak(self.$property().into_boxed_str())
            }
        }
    };
}

/// A macro to implement the Clone trait for a wrapper type.
///
/// This macro generates an implementation of the Clone trait for a wrapper type,
/// allowing the wrapper type to be cloned.
///
/// # Arguments
/// * `$type` - The Rust wrapper type (e.g. Species<'a>)
/// * `$cxx_type` - The C++ type that is being wrapped (e.g. sbmlcxx::Species)
///
/// This will generate an implementation of the Clone trait for the wrapper type,
/// allowing the wrapper type to be cloned.
#[macro_export]
macro_rules! clone {
    // Base case with just the inner field
    ($type:ty, $cxx_type:ty) => {
        impl<'a> Clone for $type {
            fn clone(&self) -> Self {
                let raw_ptr = self.inner.borrow_mut().as_mut().clone();
                let inner_ptr = pin_ptr!(raw_ptr, $cxx_type);
                Self {
                    inner: RefCell::new(inner_ptr),
                }
            }
        }
    };

    // Case with additional fields
    ($type:ty, $cxx_type:ty, $($field:ident),+) => {
        impl<'a> Clone for $type {
            fn clone(&self) -> Self {
                let raw_ptr = self.inner.borrow_mut().as_mut().clone();
                let inner_ptr = pin_ptr!(raw_ptr, $cxx_type);
                Self {
                    inner: RefCell::new(inner_ptr),
                    $(
                        $field: self.$field.clone(),
                    )+
                }
            }
        }
    };
}

/// A macro to implement the set_annotation method for collection types.
///
/// This macro generates an implementation of the set_annotation method for a collection type,
/// allowing annotations to be set on the collection.
///
/// # Arguments
/// * `$type` - The Rust wrapper type for the model (e.g. Model<'a>)
/// * `$collection_name` - The name of the collection (e.g. "reactions", "species")
/// * `$collection_type` - The Rust wrapper type for the collection (e.g. ListOfReactions)
#[macro_export]
macro_rules! set_collection_annotation {
    ($type:ty, $collection_name:expr, $collection_type:ty) => {
        paste::paste! {
            /// Sets the annotation for the [$collection_name] collection.
            ///
            /// # Arguments
            /// * `annotation` - A string slice containing the XML annotation to set
            ///
            /// # Returns
            /// Result indicating success or containing an error if the annotation is invalid
            pub fn [<set_ $collection_name _annotation>](&'a self, annotation: &str) -> Result<(), Box<dyn Error>> {
                let collection = $collection_type::new(self);
                collection.set_annotation(annotation)?;
                Ok(())
            }

            /// Gets the annotation for the [$collection_name] collection.
            ///
            /// # Returns
            /// The annotation for the [$collection_name] collection as a String
            pub fn [<get_ $collection_name _annotation>](&'a self) -> String {
                let collection = $collection_type::new(self);
                collection.get_annotation()
            }

            /// Gets the annotation as a deserialized type.
            ///
            /// # Type Parameters
            /// * `T` - The type to deserialize the annotation into
            ///
            /// # Returns
            /// Result containing the deserialized type or a deserialization error
            pub fn [<get_ $collection_name _annotation_serde>]<T: for<'de> Deserialize<'de>>(&'a self) -> Result<T, Box<dyn Error>> {
                let collection = $collection_type::new(self);
                Ok(collection.get_annotation_serde()?)
            }

            /// Sets a serializable annotation.
            ///
            /// # Arguments
            /// * `annotation` - A reference to a type implementing Serialize that will be converted to XML
            ///
            /// # Returns
            /// Result indicating success or containing a serialization error
            pub fn [<set_ $collection_name _annotation_serde>]<T: Serialize>(&'a self, annotation: &T) -> Result<(), Box<dyn Error>> {
                let collection = $collection_type::new(self);
                collection.set_annotation_serde(annotation)?;
                Ok(())
            }
        }
    };
}
