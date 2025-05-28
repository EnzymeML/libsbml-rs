use std::{cell::RefCell, collections::HashMap, pin::Pin};

use autocxx::WithinBox;
use cxx::let_cxx_string;

use crate::{
    errors::LibSBMLError,
    pin_const_ptr,
    sbmlcxx::{CaContent, CombineArchive as CxxCombineArchive, KnownFormats},
};

/// A Rust wrapper around the libSBML CombineArchive functionality.
///
/// The COMBINE Archive format is used to bundle multiple files together with metadata
/// describing their purpose and format. This is commonly used in systems biology to
/// package models, data, and documentation together.
///
/// This wrapper provides a safe and idiomatic Rust interface to work with COMBINE archives,
/// allowing users to create new archives, add files, extract content, and manipulate archive
/// metadata without directly dealing with the underlying C++ implementation details.
pub struct CombineArchive {
    /// The underlying C++ CombineArchive object wrapped in RefCell for interior mutability
    archive: RefCell<Pin<Box<CxxCombineArchive>>>,
}

impl CombineArchive {
    /// Creates a new empty COMBINE archive.
    ///
    /// # Returns
    ///
    /// A new `CombineArchive` instance ready to have files added to it.
    pub fn new() -> Self {
        Self {
            archive: RefCell::new(CxxCombineArchive::new().within_box()),
        }
    }

    /// Adds a file from the filesystem to the archive.
    ///
    /// # Arguments
    ///
    /// * `file_name` - Path to the file on the filesystem to add
    /// * `target_name` - Name/path the file should have within the archive
    /// * `format` - MIME type or format identifier for the file
    /// * `is_master` - Whether this file should be marked as the master file
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or a `LibSBMLError` if the operation fails.
    pub fn add_file(
        &self,
        file_name: impl Into<String>,
        target_name: impl Into<String>,
        format: impl Into<String>,
        is_master: bool,
    ) -> Result<(), LibSBMLError> {
        let_cxx_string!(file_name = file_name.into());
        let_cxx_string!(target_name = target_name.into());

        // Try to infer the format from the file extension
        let format = format.into();
        let format = self.lookup_format(&format).unwrap_or(format.to_string());
        let_cxx_string!(format = format);

        self.archive
            .borrow_mut()
            .as_mut()
            .addFile(&file_name, &target_name, &format, is_master);
        Ok(())
    }

    /// Adds content from a string directly to the archive.
    ///
    /// This is useful when you have file content in memory rather than on disk.
    ///
    /// # Arguments
    ///
    /// * `content` - The string content to add to the archive
    /// * `target_name` - Name/path the content should have within the archive
    /// * `format` - MIME type or format identifier for the content
    /// * `is_master` - Whether this content should be marked as the master file
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or a `LibSBMLError` if the operation fails.
    pub fn add_from_string(
        &self,
        content: impl Into<String>,
        target_name: impl Into<String>,
        format: impl Into<String>,
        is_master: bool,
    ) -> Result<(), LibSBMLError> {
        let_cxx_string!(content = content.into());
        let_cxx_string!(target_name = target_name.into());

        let format = format.into();
        let format = self.lookup_format(&format).unwrap_or(format.to_string());
        let_cxx_string!(format = format);

        self.archive.borrow_mut().as_mut().addFileFromString(
            &content,
            &target_name,
            &format,
            is_master,
        );

        Ok(())
    }

    /// Writes the archive to a file on disk.
    ///
    /// # Arguments
    ///
    /// * `path` - The filesystem path where the archive should be written
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or a `LibSBMLError` if the write operation fails.
    pub fn write_to_file(&self, path: impl Into<String>) -> Result<(), LibSBMLError> {
        let_cxx_string!(path = path.into());
        self.archive.borrow_mut().as_mut().writeToFile(&path);
        Ok(())
    }

    /// Creates a `CombineArchive` instance from an existing archive file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the existing COMBINE archive file
    ///
    /// # Returns
    ///
    /// A new `CombineArchive` instance loaded with the contents of the specified file.
    pub fn from_path(path: impl Into<String>) -> Self {
        let mut archive = CxxCombineArchive::new().within_box();
        let_cxx_string!(archive_file = path.into());
        archive
            .as_mut()
            .initializeFromArchive(&archive_file.as_ref(), false);

        Self {
            archive: RefCell::new(archive),
        }
    }

    /// Extracts all files from the archive into a HashMap.
    ///
    /// This is a convenience method that extracts all files at once. For large archives
    /// or when you only need specific files, consider using the individual extraction methods.
    ///
    /// # Returns
    ///
    /// A `HashMap` where keys are file locations within the archive and values are the
    /// file contents as strings. Files that cannot be extracted are omitted from the result.
    pub fn extract_all(&self) -> HashMap<String, String> {
        self.locations()
            .iter()
            .filter_map(|location| {
                self.extract_by_location(location)
                    .map(|content| (location.clone(), content))
            })
            .collect()
    }

    /// Returns a list of all file locations within the archive.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing all file paths/locations within the archive.
    pub fn locations(&self) -> Vec<String> {
        self.archive
            .borrow()
            .getAllLocations()
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    /// Gets the format of a file at the specified location within the archive.
    ///
    /// # Arguments
    ///
    /// * `location` - The path/location of the file within the archive
    ///
    /// # Returns
    ///
    /// `Some(String)` containing the format (MIME type) if the file exists,
    /// or `None` if the file doesn't exist.
    pub fn get_format(&self, location: &str) -> Option<String> {
        let_cxx_string!(location = location.to_string());
        let content_ptr = self.archive.borrow().getEntryByLocation(&location);
        if content_ptr.is_null() {
            None
        } else {
            let content = pin_const_ptr!(content_ptr, CaContent);
            Some(content.getFormat().to_string())
        }
    }

    /// Extracts content from a file at the specified location within the archive.
    ///
    /// # Arguments
    ///
    /// * `location` - The path/location of the file within the archive
    ///
    /// # Returns
    ///
    /// `Some(String)` containing the file content if the file exists and can be extracted,
    /// or `None` if the file doesn't exist or cannot be extracted.
    pub fn extract_by_location(&self, location: &str) -> Option<String> {
        let_cxx_string!(location = location.to_string());
        let content_ptr = self.archive.borrow().getEntryByLocation(&location);
        self.extract_content_from_ptr(content_ptr)
    }

    /// Extracts the content of the master file from the archive.
    ///
    /// The master file is the primary file in a COMBINE archive, typically the main model file.
    ///
    /// # Returns
    ///
    /// `Ok(String)` containing the master file content if it exists and can be extracted,
    /// or an error if the master file doesn't exist or cannot be extracted.
    pub fn extract_master_file_content(&self) -> Result<String, LibSBMLError> {
        let master_ptr = self.archive.borrow().getMasterFile();
        self.extract_content_from_ptr(master_ptr)
            .ok_or_else(|| LibSBMLError::CombineArchiveError("Master file not found".to_string()))
    }

    /// Internal helper method to extract content from a CaContent pointer.
    ///
    /// # Arguments
    ///
    /// * `content_ptr` - Raw pointer to a CaContent object
    ///
    /// # Returns
    ///
    /// `Some(String)` if the pointer is valid and content can be extracted, `None` otherwise.
    ///
    /// # Safety
    ///
    /// This method handles null pointer checks internally and is safe to call.
    fn extract_content_from_ptr(&self, content_ptr: *const CaContent) -> Option<String> {
        if content_ptr.is_null() {
            None
        } else {
            let content = pin_const_ptr!(content_ptr, CaContent);
            let content = self
                .archive
                .borrow_mut()
                .as_mut()
                .extractEntryToString(content.getLocation());
            Some(content.to_string())
        }
    }

    /// Looks up a standardized format identifier from a format string.
    ///
    /// # Arguments
    ///
    /// * `format` - The format string to look up
    ///
    /// # Returns
    ///
    /// `Some(String)` containing the standardized format if found, or `None` if not found.
    fn lookup_format(&self, format: &str) -> Option<String> {
        let_cxx_string!(format = format.to_string());
        let format = KnownFormats::lookupFormat(&format);

        if format.is_null() {
            None
        } else {
            Some(format.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_archive() {
        let archive = CombineArchive::new();
        assert_eq!(archive.locations().len(), 0);
    }

    #[test]
    fn test_read_combine_archive() {
        let archive = CombineArchive::from_path("tests/data/example.omex");
        let content = archive.extract_all();

        let expected_keys = ["./manifest.xml", "./data.tsv", "./model.xml"];
        for key in expected_keys {
            assert!(
                content.contains_key(key),
                "Expected key '{}' not found in archive content",
                key
            );
        }
    }

    #[test]
    fn test_extract_by_location() {
        let archive = CombineArchive::from_path("tests/data/example.omex");
        let content = archive.extract_by_location("./model.xml");
        assert!(content.is_some(), "Should be able to extract model.xml");

        let model_content = content.unwrap();
        assert!(
            !model_content.is_empty(),
            "Model content should not be empty"
        );
    }

    #[test]
    fn test_extract_non_existent_location() {
        let archive = CombineArchive::from_path("tests/data/example.omex");
        let content = archive.extract_by_location("./non_existent.xml");
        assert!(content.is_none(), "Non-existent file should return None");
    }

    #[test]
    fn test_locations() {
        let archive = CombineArchive::from_path("tests/data/example.omex");
        let locations = archive.locations();
        assert!(!locations.is_empty(), "Archive should contain files");

        // Check that all expected files are present
        let expected_files = ["./manifest.xml", "./data.tsv", "./model.xml"];
        for expected_file in expected_files {
            assert!(
                locations.contains(&expected_file.to_string()),
                "Expected file '{}' not found in locations",
                expected_file
            );
        }
    }

    #[test]
    fn test_extract_all_consistency() {
        let archive = CombineArchive::from_path("tests/data/example.omex");
        let all_content = archive.extract_all();
        let locations = archive.locations();

        // All locations should have corresponding content (assuming all files are extractable)
        for location in &locations {
            if let Some(individual_content) = archive.extract_by_location(location) {
                assert_eq!(
                    all_content.get(location),
                    Some(&individual_content),
                    "Content mismatch for location '{}'",
                    location
                );
            }
        }
    }

    #[test]
    fn test_add_file_and_locations() {
        let archive = CombineArchive::new();
        archive
            .add_file("tests/data/example.xml", "./model.xml", "sbml", true)
            .unwrap();

        assert_eq!(archive.locations().len(), 1);
    }

    #[test]
    fn test_extract_by_location_after_add() {
        let archive = CombineArchive::new();
        archive
            .add_file("tests/data/example.xml", "./model.xml", "sbml", true)
            .unwrap();

        let expected_content = include_str!("../tests/data/example.xml");
        let content = archive
            .extract_by_location("./model.xml")
            .expect("Should be able to extract model.xml");
        assert_eq!(
            content, expected_content,
            "Model content should be the same"
        );
    }

    #[test]
    fn test_extract_all_after_add() {
        let archive = CombineArchive::new();
        archive
            .add_file("tests/data/example.xml", "./model.xml", "sbml", true)
            .unwrap();

        let expected_content = include_str!("../tests/data/example.xml");
        let content = archive.extract_all();
        assert_eq!(content.len(), 1);
        assert_eq!(content.get("./model.xml").unwrap(), expected_content);
    }

    #[test]
    fn test_extract_master_file_after_add() {
        let archive = CombineArchive::new();
        archive
            .add_file("tests/data/example.xml", "./model.xml", "sbml", true)
            .unwrap();

        let expected_content = include_str!("../tests/data/example.xml");
        let content = archive
            .extract_master_file_content()
            .expect("Should be able to extract master file content");
        assert_eq!(
            content, expected_content,
            "Model content should be the same"
        );
    }
}
