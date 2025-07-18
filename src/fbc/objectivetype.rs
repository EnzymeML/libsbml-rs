use std::str::FromStr;

use crate::{errors::LibSBMLError, sbmlcxx};

/// Represents the type of optimization objective in a constraint-based model.
///
/// In flux balance analysis and related constraint-based modeling approaches,
/// objectives can be either maximized (e.g., biomass production) or minimized
/// (e.g., nutrient uptake).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectiveType {
    /// The objective function should be maximized during optimization
    Maximize,
    /// The objective function should be minimized during optimization
    Minimize,
    /// Unknown objective type or not specified
    Unknown,
}

impl From<ObjectiveType> for sbmlcxx::ObjectiveType_t {
    /// Converts a Rust ObjectiveType enum to the corresponding C++ SBML enum value
    fn from(value: ObjectiveType) -> Self {
        match value {
            ObjectiveType::Maximize => sbmlcxx::ObjectiveType_t::OBJECTIVE_TYPE_MAXIMIZE,
            ObjectiveType::Minimize => sbmlcxx::ObjectiveType_t::OBJECTIVE_TYPE_MINIMIZE,
            ObjectiveType::Unknown => sbmlcxx::ObjectiveType_t::OBJECTIVE_TYPE_UNKNOWN,
        }
    }
}

impl From<sbmlcxx::ObjectiveType_t> for ObjectiveType {
    /// Converts a C++ SBML ObjectiveType_t enum to the Rust equivalent
    fn from(value: sbmlcxx::ObjectiveType_t) -> Self {
        match value {
            sbmlcxx::ObjectiveType_t::OBJECTIVE_TYPE_MAXIMIZE => ObjectiveType::Maximize,
            sbmlcxx::ObjectiveType_t::OBJECTIVE_TYPE_MINIMIZE => ObjectiveType::Minimize,
            sbmlcxx::ObjectiveType_t::OBJECTIVE_TYPE_UNKNOWN => ObjectiveType::Unknown,
        }
    }
}

impl FromStr for ObjectiveType {
    type Err = LibSBMLError;

    /// Parses a string into an ObjectiveType.
    ///
    /// Accepts "maximize" or "minimize" strings to create the corresponding objective type.
    ///
    /// # Errors
    ///
    /// Returns a LibSBMLError if the string cannot be parsed into a valid ObjectiveType.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "maximize" => Ok(ObjectiveType::Maximize),
            "minimize" => Ok(ObjectiveType::Minimize),
            _ => Err(LibSBMLError::InvalidArgument(format!(
                "Invalid objective type: {s}. Only 'maximize' and 'minimize' are supported."
            ))),
        }
    }
}
