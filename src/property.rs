//! Property macros for standardized handling of libSBML properties
//!
//! This module provides macros that standardize the handling of properties in libSBML objects.
//! These macros generate consistent getter and setter methods for various types of properties:
//!
//! - `required_property!`: For properties that are always present and must have a value
//! - `optional_property!`: For properties that may or may not be set (returns Option<T>)
//! - `upcast_property!`: For properties that require upcasting to a parent class before access
//! - `upcast_optional_property!`: Combines upcasting with optional property handling
//!
//! The macros ensure consistent behavior across the library by:
//! 1. Properly checking if a property is set before returning its value (using isSet methods)
//! 2. Handling type conversions between C++ and Rust types
//! 3. Providing consistent documentation for generated methods
//! 4. Standardizing error handling for missing or invalid properties
//!
//! The upcast variants handle cases where properties are defined in a parent class but
//! need to be accessed through a child class. These macros automatically handle the
//! upcasting process, making the API more intuitive while hiding the complexity of
//! the underlying C++ class hierarchy.
//!
//! This approach makes it easy to extend property handling to new objects and properties
//! while maintaining a consistent API throughout the library.

/// Generates getter and setter methods for an optional property with a specified type.
///
/// This macro creates a getter and setter method for a property that may or may not be set.
/// The getter returns an Option<T> that is None when the property is not set.
/// It handles the conversion between Rust and C++ types, including string conversions
/// where necessary.
///
/// # Arguments
/// * `$type` - The Rust wrapper type (e.g., Species<'a>)
/// * `$prop` - The property name (e.g., id, name)
/// * `$return_type` - The return type for the getter (e.g., String, f64, bool)
/// * `$cpp_getter` - The C++ getter method name (e.g., getId, getName)
/// * `$cpp_setter` - The C++ setter method name (e.g., setId, setName)
/// * `$cpp_isset` - The C++ isSet method name (e.g., isSetId, isSetName)
#[macro_export]
macro_rules! optional_property {
    // String return type variant - handles CxxString conversion
    ($type:ty, $prop:ident, String, $cpp_getter:ident, $cpp_setter:ident, $cpp_isset:ident) => {
        paste::paste! {
            #[doc = "Gets the " $prop " of this object."]
            ///
            /// # Returns
            #[doc = "The " $prop " as a String, or None if not set"]
            pub fn [<$prop>](&self) -> Option<String> {
                let inner = self.inner.borrow();
                if inner.$cpp_isset() {
                    Some(inner.$cpp_getter().to_str().unwrap().to_string())
                } else {
                    None
                }
            }

            #[doc = "Sets the " $prop " of this object."]
            ///
            /// # Arguments
            #[doc = "* `" $prop "` - The new " $prop " to set"]
            pub fn [<set_ $prop>](&self, $prop: impl Into<String>) {
                let $prop = $prop.into();
                let_cxx_string!($prop = $prop);
                self.inner.borrow_mut().as_mut().$cpp_setter(&$prop);
            }
        }
    };

    // Variant with explicit input type different from return type
    ($type:ty, $prop:ident, String, $cpp_getter:ident, $cpp_setter:ident, $cpp_isset:ident, $input_type:ty) => {
        paste::paste! {
            #[doc = "Gets the " $prop " of this object."]
            ///
            /// # Returns
            #[doc = "The " $prop " as a String, or None if not set"]
            pub fn [<$prop>](&self) -> Option<String> {
                let inner = self.inner.borrow();
                if inner.$cpp_isset() {
                    Some(inner.$cpp_getter().to_str().unwrap().to_string())
                } else {
                    None
                }
            }

            #[doc = "Sets the " $prop " of this object."]
            ///
            /// # Arguments
            #[doc = "* `" $prop "` - The new " $prop " to set"]
            pub fn [<set_ $prop>](&self, $prop: $input_type) {
                let id_str = $prop.into_id();
                let_cxx_string!(id_str = id_str);
                self.inner.borrow_mut().as_mut().$cpp_setter(&id_str);
            }
        }
    };

    // Standard variant - same input and return type
    ($type:ty, $prop:ident, $return_type:ty, $cpp_getter:ident, $cpp_setter:ident, $cpp_isset:ident) => {
        paste::paste! {
            #[doc = "Gets the " $prop " of this object."]
            ///
            /// # Returns
            #[doc = "The " $prop " as a " $return_type ", or None if not set"]
            pub fn [<$prop>](&self) -> Option<$return_type> {
                let inner = self.inner.borrow();
                if inner.$cpp_isset() {
                    Some(inner.$cpp_getter().into())
                } else {
                    None
                }
            }

            #[doc = "Sets the " $prop " of this object."]
            ///
            /// # Arguments
            #[doc = "* `" $prop "` - The new " $prop " to set"]
            pub fn [<set_ $prop>](&self, $prop: impl Into<$return_type>) {
                let $prop = $prop.into();
                self.inner.borrow_mut().as_mut().$cpp_setter($prop.into());
            }
        }
    };
}

/// Generates getter and setter methods for a required property with a specified type.
///
/// This macro creates a getter and setter method for a property that must always be set.
/// The getter returns the value directly (not wrapped in an Option).
/// It handles the conversion between Rust and C++ types, including string conversions
/// where necessary.
///
/// # Arguments
/// * `$type` - The Rust wrapper type (e.g., Species<'a>)
/// * `$prop` - The property name (e.g., id, name)
/// * `$return_type` - The return type for the getter (e.g., String, f64, bool)
/// * `$cpp_getter` - The C++ getter method name (e.g., getId, getName)
/// * `$cpp_setter` - The C++ setter method name (e.g., setId, setName)
#[macro_export]
macro_rules! required_property {
    // String return type variant - handles CxxString conversion
    ($type:ty, $prop:ident, String, $cpp_getter:ident, $cpp_setter:ident) => {
        paste::paste! {
            #[doc = "Gets the " $prop " of this object."]
            ///
            /// # Returns
            #[doc = "The " $prop " as a String"]
            pub fn [<$prop>](&self) -> String {
                let inner = self.inner.borrow();
                inner.$cpp_getter().to_str().unwrap().to_string()
            }

            #[doc = "Sets the " $prop " of this object."]
            ///
            /// # Arguments
            #[doc = "* `" $prop "` - The new " $prop " to set"]
            pub fn [<set_ $prop>](&self, $prop: impl Into<String>) {
                let $prop = $prop.into();
                let_cxx_string!($prop = $prop);
                self.inner.borrow_mut().as_mut().$cpp_setter(&$prop);
            }
        }
    };

    // Variant with explicit input type different from return type
    ($type:ty, $prop:ident, String, $cpp_getter:ident, $cpp_setter:ident, $input_type:ty) => {
        paste::paste! {
            #[doc = "Gets the " $prop " of this object."]
            ///
            /// # Returns
            #[doc = "The " $prop " as a String"]
            pub fn [<$prop>](&self) -> String {
                let inner = self.inner.borrow();
                inner.$cpp_getter().to_str().unwrap().to_string()
            }

            #[doc = "Sets the " $prop " of this object."]
            ///
            /// # Arguments
            #[doc = "* `" $prop "` - The new " $prop " to set"]
            pub fn [<set_ $prop>](&self, $prop: $input_type) {
                let id_str = $prop.into_id();
                let_cxx_string!(id_str = id_str);
                self.inner.borrow_mut().as_mut().$cpp_setter(&id_str);
            }
        }
    };

    // Standard variant - same input and return type
    ($type:ty, $prop:ident, $return_type:ty, $cpp_getter:ident, $cpp_setter:ident) => {
        paste::paste! {
            #[doc = "Gets the " $prop " of this object."]
            ///
            /// # Returns
            #[doc = "The " $prop " as a " $return_type]
            pub fn [<$prop>](&self) -> $return_type {
                let inner = self.inner.borrow();
                inner.$cpp_getter().into()
            }

            #[doc = "Sets the " $prop " of this object."]
            ///
            /// # Arguments
            #[doc = "* `" $prop "` - The new " $prop " to set"]
            pub fn [<set_ $prop>](&self, $prop: impl Into<$return_type>) {
                let $prop = $prop.into();
                self.inner.borrow_mut().as_mut().$cpp_setter($prop.into());
            }
        }
    };
}

/// Generates getter and setter methods for an optional property with a specified type, using upcast.
///
/// This macro creates a getter and setter method for a property that may or may not be set,
/// using the upcast! macro to access the property from a parent type. The getter returns
/// an Option<T> that is None when the property is not set.
///
/// # Arguments
/// * `$type` - The Rust wrapper type (e.g., Species<'a>)
/// * `$prop` - The property name (e.g., id, name)
/// * `$return_type` - The return type for the getter (e.g., String, f64, bool)
/// * `$cpp_getter` - The C++ getter method name (e.g., getId, getName)
/// * `$cpp_setter` - The C++ setter method name (e.g., setId, setName)
/// * `$cpp_isset` - The C++ isSet method name (e.g., isSetId, isSetName)
/// * `$from_type` - The source C++ type to upcast from
/// * `$to_type` - The target C++ type to upcast to
#[macro_export]
macro_rules! upcast_optional_property {
    // String return type variant with upcast - handles CxxString conversion
    ($type:ty, $prop:ident, String, $cpp_getter:ident, $cpp_setter:ident, $cpp_isset:ident, $from_type:ty, $to_type:ty) => {
        paste::paste! {
            #[doc = "Gets the " $prop " of this object."]
            ///
            /// # Returns
            #[doc = "The " $prop " as a String, or None if not set"]
            pub fn [<$prop>](&self) -> Option<String> {
                let upcast_obj = upcast!(self, $from_type, $to_type);
                if upcast_obj.$cpp_isset() {
                    Some(upcast_obj.$cpp_getter().to_str().unwrap().to_string())
                } else {
                    None
                }
            }

            #[doc = "Sets the " $prop " of this object."]
            ///
            /// # Arguments
            #[doc = "* `" $prop "` - The new " $prop " to set"]
            pub fn [<set_ $prop>](&self, $prop: impl Into<String>) {
                let $prop = $prop.into();
                let_cxx_string!($prop = $prop);
                let upcast_obj = upcast!(self, $from_type, $to_type);
                upcast_obj.$cpp_setter(&$prop);
            }
        }
    };

    // Non-string return type variant with upcast
    ($type:ty, $prop:ident, $return_type:ty, $cpp_getter:ident, $cpp_setter:ident, $cpp_isset:ident, $from_type:ty, $to_type:ty) => {
        paste::paste! {
            #[doc = "Gets the " $prop " of this object."]
            ///
            /// # Returns
            #[doc = "The " $prop " as a " $return_type ", or None if not set"]
            pub fn [<$prop>](&self) -> Option<$return_type> {
                let upcast_obj = upcast!(self, $from_type, $to_type);
                if upcast_obj.$cpp_isset() {
                    Some(upcast_obj.$cpp_getter())
                } else {
                    None
                }
            }

            #[doc = "Sets the " $prop " of this object."]
            ///
            /// # Arguments
            #[doc = "* `" $prop "` - The new " $prop " to set"]
            pub fn [<set_ $prop>](&self, $prop: impl Into<$return_type>) {
                let $prop = $prop.into();
                let upcast_obj = upcast!(self, $from_type, $to_type);
                upcast_obj.$cpp_setter($prop);
            }
        }
    };
}

/// Generates getter and setter methods for a required property with a specified type, using upcast.
///
/// This macro creates a getter and setter method for a property that must always be set,
/// using the upcast! macro to access the property from a parent type. The getter returns
/// the value directly (not wrapped in an Option).
///
/// # Arguments
/// * `$type` - The Rust wrapper type (e.g., Species<'a>)
/// * `$prop` - The property name (e.g., id, name)
/// * `$return_type` - The return type for the getter (e.g., String, f64, bool)
/// * `$cpp_getter` - The C++ getter method name (e.g., getId, getName)
/// * `$cpp_setter` - The C++ setter method name (e.g., setId, setName)
/// * `$from_type` - The source C++ type to upcast from
/// * `$to_type` - The target C++ type to upcast to
#[macro_export]
macro_rules! upcast_required_property {
    // String return type variant with upcast - handles CxxString conversion
    ($type:ty, $prop:ident, String, $cpp_getter:ident, $cpp_setter:ident, $from_type:ty, $to_type:ty) => {
        paste::paste! {
            #[doc = "Gets the " $prop " of this object."]
            ///
            /// # Returns
            #[doc = "The " $prop " as a String"]
            pub fn [<$prop>](&self) -> String {
                let upcast_obj = upcast!(self, $from_type, $to_type);
                upcast_obj.$cpp_getter().to_str().unwrap().to_string()
            }

            #[doc = "Sets the " $prop " of this object."]
            ///
            /// # Arguments
            #[doc = "* `" $prop "` - The new " $prop " to set"]
            pub fn [<set_ $prop>](&self, $prop: impl Into<String>) {
                let $prop = $prop.into();
                let_cxx_string!($prop = $prop);
                let upcast_obj = upcast!(self, $from_type, $to_type);
                upcast_obj.$cpp_setter(&$prop);
            }
        }
    };

    // Non-string return type variant with upcast
    ($type:ty, $prop:ident, $return_type:ty, $cpp_getter:ident, $cpp_setter:ident, $from_type:ty, $to_type:ty) => {
        paste::paste! {
            #[doc = "Gets the " $prop " of this object."]
            ///
            /// # Returns
            #[doc = "The " $prop " as a " $return_type]
            pub fn [<$prop>](&self) -> $return_type {
                let upcast_obj = upcast!(self, $from_type, $to_type);
                upcast_obj.$cpp_getter()
            }

            #[doc = "Sets the " $prop " of this object."]
            ///
            /// # Arguments
            #[doc = "* `" $prop "` - The new " $prop " to set"]
            pub fn [<set_ $prop>](&self, $prop: impl Into<$return_type>) {
                let $prop = $prop.into();
                let upcast_obj = upcast!(self, $from_type, $to_type);
                upcast_obj.$cpp_setter($prop);
            }
        }
    };
}
