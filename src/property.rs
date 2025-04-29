/// Generates getter and setter methods for a property with a specified type.
///
/// This macro creates a getter and setter method for a property on a wrapper type.
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
/// * `$is_string` - Boolean indicating if the property is a string type (true/false)
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

/// Generates getter and setter methods for a property with a specified type.
///
/// This macro creates a getter and setter method for a property on a wrapper type.
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
/// * `$is_string` - Boolean indicating if the property is a string type (true/false)
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

/// Generates getter and setter methods for a property with a specified type, using upcast.
///
/// This macro creates a getter and setter method for a property on a wrapper type,
/// using the upcast! macro to convert between types. It handles the conversion between
/// Rust and C++ types, including string conversions where necessary.
///
/// # Arguments
/// * `$type` - The Rust wrapper type (e.g., Species<'a>)
/// * `$prop` - The property name (e.g., id, name)
/// * `$return_type` - The return type for the getter (e.g., String, f64, bool)
/// * `$cpp_getter` - The C++ getter method name (e.g., getId, getName)
/// * `$cpp_setter` - The C++ setter method name (e.g., setId, setName)
/// * `$cpp_isset` - The C++ isSet method name (e.g., isSetId, isSetName) - for optional properties
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
/// This macro creates a getter and setter method for a required property on a wrapper type,
/// using the upcast! macro to convert between types.
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
