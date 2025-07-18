use std::str::FromStr;

use crate::{errors::LibSBMLError, sbmlcxx};

/// Represents the different types of operations that can be applied to flux bounds
/// in a constraint-based model.
///
/// Flux bounds are used to constrain the allowable flux values through reactions
/// in metabolic network models, forming the basis for flux balance analysis and
/// related constraint-based modeling approaches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FluxBoundOperation {
    /// The flux must be less than or equal to a specified value
    LessEqual,
    /// The flux must be greater than or equal to a specified value
    GreaterEqual,
    /// The flux must be strictly less than a specified value
    Less,
    /// The flux must be strictly greater than a specified value
    Greater,
    /// The flux must be exactly equal to a specified value
    Equal,
    /// The operation is unknown or undefined
    Unknown,
}

impl From<FluxBoundOperation> for sbmlcxx::FluxBoundOperation_t {
    /// Converts a Rust FluxBoundOperation enum to the corresponding C++ SBML enum value
    fn from(value: FluxBoundOperation) -> Self {
        match value {
            FluxBoundOperation::LessEqual => {
                sbmlcxx::FluxBoundOperation_t::FLUXBOUND_OPERATION_LESS_EQUAL
            }
            FluxBoundOperation::GreaterEqual => {
                sbmlcxx::FluxBoundOperation_t::FLUXBOUND_OPERATION_GREATER_EQUAL
            }
            FluxBoundOperation::Less => sbmlcxx::FluxBoundOperation_t::FLUXBOUND_OPERATION_LESS,
            FluxBoundOperation::Greater => {
                sbmlcxx::FluxBoundOperation_t::FLUXBOUND_OPERATION_GREATER
            }
            FluxBoundOperation::Equal => sbmlcxx::FluxBoundOperation_t::FLUXBOUND_OPERATION_EQUAL,
            FluxBoundOperation::Unknown => {
                sbmlcxx::FluxBoundOperation_t::FLUXBOUND_OPERATION_UNKNOWN
            }
        }
    }
}

impl From<sbmlcxx::FluxBoundOperation_t> for FluxBoundOperation {
    /// Converts a C++ SBML FluxBoundOperation_t enum to the Rust equivalent
    fn from(value: sbmlcxx::FluxBoundOperation_t) -> Self {
        match value {
            sbmlcxx::FluxBoundOperation_t::FLUXBOUND_OPERATION_LESS_EQUAL => {
                FluxBoundOperation::LessEqual
            }
            sbmlcxx::FluxBoundOperation_t::FLUXBOUND_OPERATION_GREATER_EQUAL => {
                FluxBoundOperation::GreaterEqual
            }
            sbmlcxx::FluxBoundOperation_t::FLUXBOUND_OPERATION_LESS => FluxBoundOperation::Less,
            sbmlcxx::FluxBoundOperation_t::FLUXBOUND_OPERATION_GREATER => {
                FluxBoundOperation::Greater
            }
            sbmlcxx::FluxBoundOperation_t::FLUXBOUND_OPERATION_EQUAL => FluxBoundOperation::Equal,
            sbmlcxx::FluxBoundOperation_t::FLUXBOUND_OPERATION_UNKNOWN => {
                FluxBoundOperation::Unknown
            }
        }
    }
}

impl FromStr for FluxBoundOperation {
    type Err = LibSBMLError;

    /// Parses a string into a FluxBoundOperation.
    ///
    /// Accepts both standard names and common abbreviations:
    /// - "less_equal", "lessEqual", "leq" for LessEqual
    /// - "greater_equal", "greaterEqual", "geq" for GreaterEqual
    /// - "less", "lt" for Less
    /// - "greater", "gt" for Greater
    /// - "equal", "eq" for Equal
    /// - "unknown" for Unknown
    ///
    /// # Errors
    ///
    /// Returns a LibSBMLError if the string cannot be parsed into a valid FluxBoundOperation.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "less_equal" | "lessEqual" | "leq" => Ok(FluxBoundOperation::LessEqual),
            "greater_equal" | "greaterEqual" | "geq" => Ok(FluxBoundOperation::GreaterEqual),
            "less" | "lt" => Ok(FluxBoundOperation::Less),
            "greater" | "gt" => Ok(FluxBoundOperation::Greater),
            "equal" | "eq" => Ok(FluxBoundOperation::Equal),
            "unknown" => Ok(FluxBoundOperation::Unknown),
            _ => Err(LibSBMLError::InvalidArgument(format!(
                "Invalid flux bound operation: {s}. Only 'less_equal', 'greater_equal', 'less', 'greater', 'equal', and 'unknown' are supported."
            ))),
        }
    }
}
