//! Error handling for SBML validation and consistency checking.
//!
//! This module provides types for representing and working with errors that occur
//! during SBML document validation. It includes structures for individual errors
//! and collections of errors, as well as severity levels to indicate the importance
//! of each error.
//!
//! The main types in this module are:
//! - `SBMLErrorLog`: A collection of validation errors from an SBML document
//! - `SBMLError`: An individual validation error with detailed information
//! - `SBMLErrorSeverity`: The severity level of an error (Error, Warning, etc.)

use std::pin::Pin;

use crate::{pin_ptr, sbmlcxx, SBMLDocument};

/// Represents a collection of SBML validation errors from a document.
///
/// This struct contains the validation status of an SBML document and
/// a list of all errors encountered during validation.
#[derive(Debug)]
pub struct SBMLErrorLog {
    /// Indicates whether the document is valid (true) or has errors (false)
    pub valid: bool,
    /// Collection of all errors found during validation
    pub errors: Vec<SBMLError>,
}

impl SBMLErrorLog {
    /// Creates a new error log from an SBML document.
    ///
    /// Extracts all errors from the document's internal error log and
    /// determines if the document is valid based on the presence of
    /// errors with severity level Error or Fatal.
    ///
    /// # Arguments
    /// * `document` - Reference to the SBML document to extract errors from
    ///
    /// # Returns
    /// A new `SBMLErrorLog` containing all errors and validation status
    pub fn new(document: &SBMLDocument) -> Self {
        // Get the amount of errors for extraction
        let n_errors = document.inner().borrow().getNumErrors().0;

        // Pin the error log to extract all errors
        let errorlog_ptr = document.inner().borrow_mut().pin_mut().getErrorLog();
        let errorlog = pin_ptr!(errorlog_ptr, sbmlcxx::SBMLErrorLog);

        // Convert the errors to a Vec with pre-allocated capacity for efficiency
        let mut errors = Vec::with_capacity(n_errors as usize);
        for i in 0..n_errors {
            errors.push(SBMLError::new(errorlog.as_ref().getError(i.into())));
        }

        // Document is invalid if it contains any Error or Fatal severity errors
        let has_errors = errors.iter().any(|error| {
            error.severity == SBMLErrorSeverity::Error || error.severity == SBMLErrorSeverity::Fatal
        });

        Self {
            valid: !has_errors,
            errors,
        }
    }
}

/// Represents a single SBML validation error.
///
/// Contains detailed information about an error encountered during
/// SBML document validation, including its message, severity level,
/// location in the document, and category.
#[derive(Debug)]
pub struct SBMLError {
    /// The error message describing the issue
    pub message: String,
    /// The severity level of the error
    pub severity: SBMLErrorSeverity,
    /// The line number where the error occurred
    pub line: u32,
    /// The column number where the error occurred
    pub column: u32,
    /// The category of the error (e.g., "SBML", "XML", etc.)
    pub category: String,
}

impl SBMLError {
    /// Creates a new SBML error from a raw error pointer.
    ///
    /// # Arguments
    /// * `error` - Pointer to the native SBMLError object
    ///
    /// # Returns
    /// A new `SBMLError` with information extracted from the native error
    pub fn new(error: *const sbmlcxx::SBMLError) -> Self {
        let xml_error = error as *const sbmlcxx::XMLError;
        let xml_error = unsafe { Pin::new_unchecked(&*xml_error) };

        let message = xml_error.as_ref().getMessage().to_string();
        let line = xml_error.as_ref().getLine().0;
        let column = xml_error.as_ref().getColumn().0;
        let category = xml_error.as_ref().getCategoryAsString().to_string();
        let severity = SBMLErrorSeverity::from(&*xml_error);

        Self {
            message,
            severity,
            line,
            column,
            category,
        }
    }
}

/// Represents the severity level of an SBML error.
///
/// SBML errors can have different severity levels, ranging from
/// informational messages to fatal errors that prevent document processing.
#[derive(Debug, PartialEq, Eq)]
pub enum SBMLErrorSeverity {
    /// Standard error that indicates a problem with the SBML document
    Error,
    /// Severe error that prevents further processing
    Fatal,
    /// Warning that doesn't invalidate the document but should be addressed
    Warning,
    /// Informational message that doesn't indicate a problem
    Info,
    /// Internal error in the SBML library
    Internal,
    /// System-level error (e.g., file I/O problems)
    System,
    /// Unknown error type
    Unknown,
}

impl From<&sbmlcxx::XMLError> for SBMLErrorSeverity {
    /// Converts a native XMLError to an SBMLErrorSeverity.
    ///
    /// # Arguments
    /// * `xml_error` - Reference to the native XMLError
    ///
    /// # Returns
    /// The corresponding SBMLErrorSeverity variant
    fn from(xml_error: &sbmlcxx::XMLError) -> Self {
        if xml_error.isError() {
            SBMLErrorSeverity::Error
        } else if xml_error.isWarning() {
            SBMLErrorSeverity::Warning
        } else if xml_error.isInternal() {
            SBMLErrorSeverity::Internal
        } else if xml_error.isFatal() {
            SBMLErrorSeverity::Fatal
        } else if xml_error.isSystem() {
            SBMLErrorSeverity::System
        } else if xml_error.isInfo() {
            SBMLErrorSeverity::Info
        } else {
            SBMLErrorSeverity::Unknown
        }
    }
}
