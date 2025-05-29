use std::{
    collections::HashMap,
    io::{Cursor, Read, Write},
    path::Path,
};
use zip::{write::SimpleFileOptions, ZipArchive, ZipWriter};

use crate::combine::manifest::OmexManifest;

use super::{error::CombineArchiveError, manifest::Content};

/// A COMBINE Archive (OMEX) implementation for managing collections of files
/// with metadata according to the COMBINE Archive specification.
///
/// The COMBINE Archive format is used in computational biology to package
/// models, data, and metadata together in a standardized way. This implementation
/// provides a high-level interface for creating, reading, and modifying OMEX files.
pub struct CombineArchive {
    /// The manifest containing metadata about all files in the archive
    pub manifest: OmexManifest,

    /// Optional path to the archive file on disk
    path: Option<std::path::PathBuf>,

    // Internal state for efficient mutation tracking
    /// Original ZIP data when loaded from file
    original_zip: Option<Vec<u8>>,
    /// New or modified entries waiting to be written
    pending_entries: HashMap<String, Vec<u8>>,
    /// Entries marked for removal
    removed_entries: std::collections::HashSet<String>,
    /// Flag indicating if the archive needs to be rebuilt
    needs_rebuild: bool,
}

/// Represents a single entry (file) within a COMBINE Archive.
///
/// An entry contains both the file data and its associated metadata
/// from the manifest.
pub struct Entry {
    /// Metadata about this entry from the manifest
    pub content: Content,
    /// The raw file data
    pub data: Vec<u8>,
}

impl CombineArchive {
    /// Creates a new empty COMBINE Archive.
    ///
    /// The archive will have an empty manifest and no associated file path.
    /// Use [`add_entry`](Self::add_entry) or [`add_file`](Self::add_file) to add content.
    pub fn new() -> Self {
        Self {
            manifest: OmexManifest::new(),
            path: None,
            original_zip: None,
            pending_entries: HashMap::new(),
            removed_entries: std::collections::HashSet::new(),
            needs_rebuild: false,
        }
    }

    /// Opens an existing COMBINE Archive from a file.
    ///
    /// This method reads the ZIP file, extracts and parses the manifest,
    /// and prepares the archive for reading and modification.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the OMEX file to open
    ///
    /// # Returns
    ///
    /// Returns a `CombineArchive` instance on success, or a `CombineArchiveError`
    /// if the file cannot be read or is not a valid COMBINE Archive.
    ///
    /// # Errors
    ///
    /// * `CombineArchiveError::Io` - If the file cannot be read
    /// * `CombineArchiveError::Zip` - If the file is not a valid ZIP archive
    /// * `CombineArchiveError::Manifest` - If the manifest.xml is missing or invalid
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, CombineArchiveError> {
        let path_buf = path.as_ref().to_path_buf();
        let zip_data = std::fs::read(&path_buf)?;

        // Extract and parse the manifest
        let manifest = Self::extract_manifest(&zip_data)?;

        Ok(Self {
            manifest,
            path: Some(path_buf),
            original_zip: Some(zip_data),
            pending_entries: HashMap::new(),
            removed_entries: std::collections::HashSet::new(),
            needs_rebuild: false,
        })
    }

    /// Adds a file from the filesystem to the archive.
    ///
    /// This is a convenience method that reads a file from disk and adds it
    /// to the archive with the specified metadata.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file on disk to add
    /// * `location` - Location within the archive (e.g., "./model.xml")
    /// * `format` - MIME type or format identifier for the file
    /// * `master` - Whether this file is the master file of the archive
    ///
    /// # Errors
    ///
    /// * `CombineArchiveError::Io` - If the file cannot be read
    /// * `CombineArchiveError::Manifest` - If there's an error updating the manifest
    pub fn add_file<P: AsRef<Path>>(
        &mut self,
        file_path: P,
        location: impl Into<String>,
        format: impl Into<String>,
        master: bool,
    ) -> Result<(), CombineArchiveError> {
        let data = std::fs::read(file_path)?;
        self.add_entry(location, format, master, &data[..])
    }

    /// Adds data to the archive from any source that implements `Read`.
    ///
    /// This is the primary method for adding content to the archive. It updates
    /// the manifest and stages the data for writing. If an entry with the same
    /// location already exists, it will be updated if the format and master flag
    /// match, or replaced if they differ.
    ///
    /// # Arguments
    ///
    /// * `location` - Location within the archive (e.g., "./model.xml")
    /// * `format` - MIME type or format identifier for the file
    /// * `master` - Whether this file is the master file of the archive
    /// * `data` - Data source implementing `Read`
    ///
    /// # Behavior with Existing Entries
    ///
    /// - If an entry exists with the same location, format, and master flag:
    ///   the data is updated while preserving the manifest entry
    /// - If an entry exists but format or master flag differs:
    ///   the old entry is completely replaced
    ///
    /// # Errors
    ///
    /// * `CombineArchiveError::Io` - If reading from the data source fails
    /// * `CombineArchiveError::Manifest` - If there's an error updating the manifest
    pub fn add_entry(
        &mut self,
        location: impl Into<String>,
        format: impl Into<String>,
        master: bool,
        mut data: impl Read,
    ) -> Result<(), CombineArchiveError> {
        let location = location.into();
        let format = format.into();

        // Check if entry already exists and handle accordingly
        if let Some(existing_content) = self.find_content(&location) {
            if existing_content.format == format && existing_content.master == master {
                // Same metadata - just update the data
                let mut data_buf = Vec::new();
                data.read_to_end(&mut data_buf)?;

                let zip_location = location.replace("./", "");
                self.removed_entries.remove(&zip_location);
                self.pending_entries.insert(zip_location, data_buf);
                self.needs_rebuild = true;
                return Ok(());
            } else {
                // Different metadata - remove the old entry first
                self.manifest.content.retain(|c| c.location != location);
            }
        }

        // Add new entry to manifest
        self.manifest.add_entry(location.clone(), format, master)?;

        // Read and store the data
        let mut data_buf = Vec::new();
        data.read_to_end(&mut data_buf)?;

        let zip_location = location.replace("./", "");
        self.removed_entries.remove(&zip_location);
        self.pending_entries.insert(zip_location, data_buf);
        self.needs_rebuild = true;

        Ok(())
    }

    /// Removes an entry from the archive.
    ///
    /// This removes both the file data and its metadata from the manifest.
    /// The change is staged and will take effect when the archive is saved.
    ///
    /// # Arguments
    ///
    /// * `location` - Location of the entry to remove (e.g., "./model.xml")
    ///
    /// # Note
    ///
    /// Removing the master file will leave the archive without a master file,
    /// which may make it invalid according to the COMBINE specification.
    pub fn remove_entry(&mut self, location: &str) -> Result<(), CombineArchiveError> {
        let zip_location = location.replace("./", "");

        // Remove from manifest
        self.manifest.content.retain(|c| c.location != location);

        // Mark for removal from ZIP
        self.removed_entries.insert(zip_location.clone());
        self.pending_entries.remove(&zip_location);
        self.needs_rebuild = true;

        Ok(())
    }

    /// Retrieves an entry from the archive.
    ///
    /// This method returns both the file data and its metadata. It will check
    /// pending changes first, then fall back to the original archive data.
    ///
    /// # Arguments
    ///
    /// * `location` - Location of the entry to retrieve (e.g., "./model.xml")
    ///
    /// # Returns
    ///
    /// Returns an `Entry` containing the file data and metadata, or an error
    /// if the entry doesn't exist or cannot be read.
    ///
    /// # Errors
    ///
    /// * `CombineArchiveError::FileNotFound` - If the entry doesn't exist
    /// * `CombineArchiveError::Zip` - If there's an error reading from the ZIP
    /// * `CombineArchiveError::Io` - If there's an I/O error
    pub fn entry(&mut self, location: &str) -> Result<Entry, CombineArchiveError> {
        if !self.manifest.has_location(location) {
            return Err(CombineArchiveError::FileNotFound(location.to_string()));
        }

        let zip_location = location.replace("./", "");

        // Check pending entries first (most recent changes)
        if let Some(data) = self.pending_entries.get(&zip_location) {
            return Ok(Entry {
                content: self.find_content(location).unwrap().clone(),
                data: data.clone(),
            });
        }

        // Check if it was removed
        if self.removed_entries.contains(&zip_location) {
            return Err(CombineArchiveError::FileNotFound(location.to_string()));
        }

        // Read from original ZIP archive
        if let Some(ref zip_data) = self.original_zip {
            let mut archive = ZipArchive::new(Cursor::new(zip_data))?;
            let mut file = archive.by_name(&zip_location)?;
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;

            return Ok(Entry {
                content: self.find_content(location).unwrap().clone(),
                data,
            });
        }

        Err(CombineArchiveError::FileNotFound(location.to_string()))
    }

    /// Retrieves the master file of the archive.
    ///
    /// The master file is the primary file in a COMBINE Archive, typically
    /// the main model or simulation description.
    ///
    /// # Returns
    ///
    /// Returns an `Entry` for the master file, or an error if no master file
    /// is defined or it cannot be read.
    ///
    /// # Errors
    ///
    /// * `CombineArchiveError::MasterFileNotFound` - If no master file is defined
    /// * Other errors from [`entry`](Self::entry) method
    pub fn master(&mut self) -> Result<Entry, CombineArchiveError> {
        let location = self
            .manifest
            .master_file()
            .ok_or(CombineArchiveError::MasterFileNotFound)?
            .location
            .clone();
        self.entry(&location)
    }

    /// Lists all entries in the archive.
    ///
    /// Returns references to the metadata for all files in the archive.
    /// This reflects the current state including any pending additions or removals.
    ///
    /// # Returns
    ///
    /// A vector of references to `Content` objects representing each entry's metadata.
    pub fn list_entries(&self) -> Vec<&Content> {
        self.manifest.content.iter().collect()
    }

    /// Checks if an entry exists in the archive.
    ///
    /// This checks the manifest for the specified location, reflecting
    /// any pending changes.
    ///
    /// # Arguments
    ///
    /// * `location` - Location to check (e.g., "./model.xml")
    ///
    /// # Returns
    ///
    /// `true` if the entry exists, `false` otherwise.
    pub fn has_entry(&self, location: &str) -> bool {
        self.manifest.has_location(location)
    }

    /// Saves the archive to a file.
    ///
    /// This method builds the complete ZIP archive with all current entries
    /// and writes it to the specified path. After saving, the internal state
    /// is updated to reflect the saved state.
    ///
    /// # Arguments
    ///
    /// * `path` - Path where the archive should be saved
    ///
    /// # Errors
    ///
    /// * `CombineArchiveError::Io` - If the file cannot be written
    /// * `CombineArchiveError::Zip` - If there's an error creating the ZIP
    /// * `CombineArchiveError::Manifest` - If the manifest cannot be serialized
    pub fn save<P: AsRef<Path>>(&mut self, path: P) -> Result<(), CombineArchiveError> {
        let zip_data = self.build_zip()?;
        std::fs::write(path, &zip_data)?;

        // Update internal state to reflect saved state
        self.original_zip = Some(zip_data);
        self.pending_entries.clear();
        self.removed_entries.clear();
        self.needs_rebuild = false;

        Ok(())
    }

    /// Saves changes to the original file.
    ///
    /// This method is only available for archives that were opened from a file.
    /// It saves the current state back to the original file path.
    ///
    /// # Errors
    ///
    /// * `CombineArchiveError::NoPath` - If the archive wasn't opened from a file
    /// * Other errors from [`save`](Self::save) method
    pub fn save_changes(&mut self) -> Result<(), CombineArchiveError> {
        if let Some(ref path) = self.path.clone() {
            self.save(path)
        } else {
            Err(CombineArchiveError::NoPath)
        }
    }

    /// Gets the archive as bytes without saving to disk.
    ///
    /// This method builds the complete ZIP archive in memory and returns
    /// the raw bytes. Useful for streaming or when you don't want to
    /// create a temporary file.
    ///
    /// # Returns
    ///
    /// Returns the complete archive as a byte vector.
    ///
    /// # Errors
    ///
    /// * `CombineArchiveError::Zip` - If there's an error creating the ZIP
    /// * `CombineArchiveError::Manifest` - If the manifest cannot be serialized
    pub fn to_bytes(&mut self) -> Result<Vec<u8>, CombineArchiveError> {
        self.build_zip()
    }

    // Private helper methods

    /// Extracts and parses the manifest from ZIP data.
    fn extract_manifest(zip_data: &[u8]) -> Result<OmexManifest, CombineArchiveError> {
        let mut archive = ZipArchive::new(Cursor::new(zip_data))?;
        let mut manifest_buf = Vec::new();
        archive
            .by_name("manifest.xml")?
            .read_to_end(&mut manifest_buf)?;
        let manifest = OmexManifest::from_xml(&String::from_utf8(manifest_buf).unwrap())?;
        Ok(manifest)
    }

    /// Finds content metadata by location.
    fn find_content(&self, location: &str) -> Option<&Content> {
        self.manifest
            .content
            .iter()
            .find(|c| c.location == location)
    }

    /// Builds the complete ZIP archive with current state.
    fn build_zip(&self) -> Result<Vec<u8>, CombineArchiveError> {
        let mut buffer = Vec::new();
        let mut writer = ZipWriter::new(Cursor::new(&mut buffer));
        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        // Copy entries from original ZIP that aren't removed or overwritten
        if let Some(ref original_data) = self.original_zip {
            let mut original_archive = ZipArchive::new(Cursor::new(original_data))?;
            for i in 0..original_archive.len() {
                let mut file = original_archive.by_index(i)?;
                let name = file.name().to_string();

                // Skip if removed, overwritten, or is manifest (we'll add manifest last)
                if self.removed_entries.contains(&name)
                    || self.pending_entries.contains_key(&name)
                    || name == "manifest.xml"
                {
                    continue;
                }

                writer.start_file(&name, options)?;
                std::io::copy(&mut file, &mut writer)?;
            }
        }

        // Add all pending entries (new or modified files)
        for (name, data) in &self.pending_entries {
            writer.start_file(name, options)?;
            writer.write_all(data)?;
        }

        // Always add manifest last to ensure it's up to date
        let manifest_xml = self.manifest.to_xml().map_err(|e| {
            CombineArchiveError::Manifest(quick_xml::DeError::Custom(e.to_string()))
        })?;
        writer.start_file("manifest.xml", options)?;
        writer.write_all(manifest_xml.as_bytes())?;

        writer.finish()?;
        Ok(buffer)
    }
}

impl Default for CombineArchive {
    fn default() -> Self {
        Self::new()
    }
}

impl Entry {
    /// Converts the entry data to a UTF-8 string.
    ///
    /// This is useful for text-based files like XML, CSV, or JSON.
    ///
    /// # Returns
    ///
    /// Returns the file content as a string, or an error if the data
    /// is not valid UTF-8.
    ///
    /// # Errors
    ///
    /// Returns `std::string::FromUtf8Error` if the data is not valid UTF-8.
    pub fn as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }

    /// Gets the raw data bytes.
    ///
    /// Returns a slice of the raw file data. This works for both
    /// text and binary files.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Creates a reader for the entry data.
    ///
    /// Returns a `Cursor` that implements `Read` and `Seek`, allowing
    /// you to read the data incrementally or seek to specific positions.
    pub fn reader(&self) -> Cursor<&[u8]> {
        Cursor::new(&self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_dir() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn test_new_archive_creation() {
        let archive = CombineArchive::new();
        assert_eq!(archive.list_entries().len(), 0);
        assert!(!archive.has_entry("./test.xml"));
        assert!(archive.path.is_none());
        assert!(!archive.needs_rebuild);
    }

    #[test]
    fn test_open_archive_to_sbml() {
        let archive_path = Path::new("tests/data/test.omex");
        let mut archive = CombineArchive::open(archive_path).unwrap();

        // Get the master SBML file
        let master = archive.master().unwrap();
        let xml_string = master.as_string().unwrap();

        let expected_path = Path::new("tests/data/expected_omex_content.xml");
        let expected_content = fs::read_to_string(expected_path)
            .unwrap()
            .replace("\r\n", "\n");
        assert_eq!(xml_string, expected_content);

        // Check the CSV content
        let csv_entry = archive.entry("./data.tsv").unwrap();
        let csv_string = csv_entry.as_string().unwrap();
        let expected_csv_path = Path::new("tests/data/expected_omex_data.tsv");
        let expected_csv_content = fs::read_to_string(expected_csv_path)
            .unwrap()
            .replace("\r\n", "\n");
        assert_eq!(csv_string, expected_csv_content);
    }

    #[test]
    fn test_add_entry_basic() {
        let mut archive = CombineArchive::new();

        archive
            .add_entry(
                "./model.xml",
                "http://identifiers.org/combine.specifications/sbml",
                true,
                b"<sbml>model</sbml>".as_slice(),
            )
            .unwrap();

        assert_eq!(archive.list_entries().len(), 1);
        assert!(archive.has_entry("./model.xml"));
        assert!(archive.needs_rebuild);

        let entry = archive.entry("./model.xml").unwrap();
        assert_eq!(entry.as_string().unwrap(), "<sbml>model</sbml>");
        assert_eq!(
            entry.content.format,
            "http://identifiers.org/combine.specifications/sbml"
        );
        assert!(entry.content.master);
    }

    #[test]
    fn test_add_multiple_entries() {
        let mut archive = CombineArchive::new();

        // Add multiple entries
        archive
            .add_entry(
                "./model.xml",
                "http://identifiers.org/combine.specifications/sbml",
                true,
                b"<sbml>model</sbml>".as_slice(),
            )
            .unwrap();

        archive
            .add_entry("./data.csv", "text/csv", false, b"x,y\n1,2\n3,4".as_slice())
            .unwrap();

        archive
            .add_entry(
                "./script.py",
                "text/x-python",
                false,
                b"print('hello world')".as_slice(),
            )
            .unwrap();

        assert_eq!(archive.list_entries().len(), 3);
        assert!(archive.has_entry("./model.xml"));
        assert!(archive.has_entry("./data.csv"));
        assert!(archive.has_entry("./script.py"));

        // Check master file
        let master = archive.master().unwrap();
        assert_eq!(master.as_string().unwrap(), "<sbml>model</sbml>");
    }

    #[test]
    fn test_add_file_from_disk() {
        let temp_dir = create_test_dir();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello from file!").unwrap();

        let mut archive = CombineArchive::new();
        archive
            .add_file(&file_path, "./test.txt", "text/plain", false)
            .unwrap();

        assert!(archive.has_entry("./test.txt"));
        let entry = archive.entry("./test.txt").unwrap();
        assert_eq!(entry.as_string().unwrap(), "Hello from file!");
    }

    #[test]
    fn test_end_to_end_save_and_load() {
        let temp_dir = create_test_dir();
        let archive_path = temp_dir.path().join("test.omex");

        // Create and populate archive
        let mut archive = CombineArchive::new();
        archive
            .add_entry(
                "./model.xml",
                "http://identifiers.org/combine.specifications/sbml",
                true,
                b"<sbml><model>test model</model></sbml>".as_slice(),
            )
            .unwrap();

        archive
            .add_entry(
                "./data.csv",
                "text/csv",
                false,
                b"time,value\n0,1\n1,2\n2,3".as_slice(),
            )
            .unwrap();

        // Save to disk
        archive.save(&archive_path).unwrap();
        assert!(archive_path.exists());
        assert!(!archive.needs_rebuild); // Should be clean after save

        // Load from disk
        let mut loaded_archive = CombineArchive::open(&archive_path).unwrap();
        assert_eq!(loaded_archive.list_entries().len(), 2);
        assert!(loaded_archive.has_entry("./model.xml"));
        assert!(loaded_archive.has_entry("./data.csv"));

        // Verify content
        let model_entry = loaded_archive.entry("./model.xml").unwrap();
        assert_eq!(
            model_entry.as_string().unwrap(),
            "<sbml><model>test model</model></sbml>"
        );
        assert!(model_entry.content.master);

        let data_entry = loaded_archive.entry("./data.csv").unwrap();
        assert_eq!(data_entry.as_string().unwrap(), "time,value\n0,1\n1,2\n2,3");
        assert!(!data_entry.content.master);

        // Verify master file access
        let master = loaded_archive.master().unwrap();
        assert_eq!(
            master.as_string().unwrap(),
            "<sbml><model>test model</model></sbml>"
        );
    }

    #[test]
    fn test_archive_mutation_add_remove() {
        let temp_dir = create_test_dir();
        let archive_path = temp_dir.path().join("test.omex");

        // Create initial archive
        let mut archive = CombineArchive::new();
        archive
            .add_entry(
                "./model.xml",
                "application/xml",
                true,
                b"<model>v1</model>".as_slice(),
            )
            .unwrap();
        archive
            .add_entry("./data1.csv", "text/csv", false, b"a,b\n1,2".as_slice())
            .unwrap();
        archive
            .add_entry("./data2.csv", "text/csv", false, b"c,d\n3,4".as_slice())
            .unwrap();
        archive.save(&archive_path).unwrap();

        // Load and mutate
        let mut loaded_archive = CombineArchive::open(&archive_path).unwrap();
        assert_eq!(loaded_archive.list_entries().len(), 3);

        // Remove an entry
        loaded_archive.remove_entry("./data1.csv").unwrap();
        assert_eq!(loaded_archive.list_entries().len(), 2);
        assert!(!loaded_archive.has_entry("./data1.csv"));
        assert!(loaded_archive.has_entry("./data2.csv"));

        // Add a new entry
        loaded_archive
            .add_entry(
                "./script.py",
                "text/x-python",
                false,
                b"print('new script')".as_slice(),
            )
            .unwrap();
        assert_eq!(loaded_archive.list_entries().len(), 3);
        assert!(loaded_archive.has_entry("./script.py"));

        // Modify existing entry (overwrite)
        loaded_archive
            .add_entry(
                "./model.xml",
                "application/xml",
                true,
                b"<model>v2</model>".as_slice(),
            )
            .unwrap();

        // Save changes
        loaded_archive.save_changes().unwrap();

        // Reload and verify mutations
        let mut final_archive = CombineArchive::open(&archive_path).unwrap();
        assert_eq!(final_archive.list_entries().len(), 3);
        assert!(!final_archive.has_entry("./data1.csv"));
        assert!(final_archive.has_entry("./data2.csv"));
        assert!(final_archive.has_entry("./script.py"));
        assert!(final_archive.has_entry("./model.xml"));

        // Verify modified content
        let model = final_archive.entry("./model.xml").unwrap();
        assert_eq!(model.as_string().unwrap(), "<model>v2</model>");

        let script = final_archive.entry("./script.py").unwrap();
        assert_eq!(script.as_string().unwrap(), "print('new script')");
    }

    #[test]
    fn test_complex_mutation_workflow() {
        let temp_dir = create_test_dir();
        let archive_path = temp_dir.path().join("complex.omex");

        // Create archive with multiple files
        let mut archive = CombineArchive::new();
        for i in 1..=5 {
            archive
                .add_entry(
                    format!("./file{}.txt", i),
                    "text/plain",
                    i == 1, // First file is master
                    format!("Content of file {}", i).as_bytes(),
                )
                .unwrap();
        }
        archive.save(&archive_path).unwrap();

        // Load and perform complex mutations
        let mut archive = CombineArchive::open(&archive_path).unwrap();

        // Remove some files
        archive.remove_entry("./file2.txt").unwrap();
        archive.remove_entry("./file4.txt").unwrap();

        // Add new files
        archive
            .add_entry(
                "./new1.json",
                "application/json",
                false,
                b"{\"new\": 1}".as_slice(),
            )
            .unwrap();
        archive
            .add_entry(
                "./new2.xml",
                "application/xml",
                false,
                b"<new>2</new>".as_slice(),
            )
            .unwrap();

        // Modify existing file
        archive
            .add_entry(
                "./file3.txt",
                "text/plain",
                false,
                b"Modified file 3".as_slice(),
            )
            .unwrap();

        // Save and reload
        archive.save_changes().unwrap();
        let mut final_archive = CombineArchive::open(&archive_path).unwrap();

        // Verify final state
        assert_eq!(final_archive.list_entries().len(), 5); // 1,3,5 + new1,new2
        assert!(final_archive.has_entry("./file1.txt"));
        assert!(!final_archive.has_entry("./file2.txt"));
        assert!(final_archive.has_entry("./file3.txt"));
        assert!(!final_archive.has_entry("./file4.txt"));
        assert!(final_archive.has_entry("./file5.txt"));
        assert!(final_archive.has_entry("./new1.json"));
        assert!(final_archive.has_entry("./new2.xml"));

        // Verify content
        let file3 = final_archive.entry("./file3.txt").unwrap();
        assert_eq!(file3.as_string().unwrap(), "Modified file 3");

        let new1 = final_archive.entry("./new1.json").unwrap();
        assert_eq!(new1.as_string().unwrap(), "{\"new\": 1}");
    }

    #[test]
    fn test_to_bytes_without_saving() {
        let mut archive = CombineArchive::new();
        archive
            .add_entry("./test.txt", "text/plain", true, b"test content".as_slice())
            .unwrap();

        let bytes = archive.to_bytes().unwrap();
        assert!(!bytes.is_empty());

        // Verify we can read the bytes back
        let temp_dir = create_test_dir();
        let temp_path = temp_dir.path().join("from_bytes.omex");
        fs::write(&temp_path, &bytes).unwrap();

        let mut loaded = CombineArchive::open(&temp_path).unwrap();
        assert!(loaded.has_entry("./test.txt"));
        let entry = loaded.entry("./test.txt").unwrap();
        assert_eq!(entry.as_string().unwrap(), "test content");
    }

    #[test]
    fn test_entry_methods() {
        let mut archive = CombineArchive::new();
        archive
            .add_entry(
                "./test.txt",
                "text/plain",
                false,
                b"Hello World!".as_slice(),
            )
            .unwrap();

        let entry = archive.entry("./test.txt").unwrap();

        // Test different access methods
        assert_eq!(entry.as_string().unwrap(), "Hello World!");
        assert_eq!(entry.as_bytes(), b"Hello World!");

        let mut reader = entry.reader();
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).unwrap();
        assert_eq!(buffer, "Hello World!");
    }

    #[test]
    fn test_error_cases() {
        let mut archive = CombineArchive::new();

        // Test file not found
        assert!(matches!(
            archive.entry("./nonexistent.txt"),
            Err(CombineArchiveError::FileNotFound(_))
        ));

        // Test master file not found on empty archive
        assert!(matches!(
            archive.master(),
            Err(CombineArchiveError::MasterFileNotFound)
        ));

        // Test save_changes without path
        assert!(matches!(
            archive.save_changes(),
            Err(CombineArchiveError::NoPath)
        ));

        // Test opening non-existent file
        assert!(CombineArchive::open("./nonexistent.omex").is_err());
    }

    #[test]
    fn test_removed_entry_access() {
        let temp_dir = create_test_dir();
        let archive_path = temp_dir.path().join("test.omex");

        // Create archive with entry
        let mut archive = CombineArchive::new();
        archive
            .add_entry("./test.txt", "text/plain", true, b"content".as_slice())
            .unwrap();
        archive.save(&archive_path).unwrap();

        // Load and remove entry
        let mut archive = CombineArchive::open(&archive_path).unwrap();
        archive.remove_entry("./test.txt").unwrap();

        // Try to access removed entry
        assert!(matches!(
            archive.entry("./test.txt"),
            Err(CombineArchiveError::FileNotFound(_))
        ));
        assert!(!archive.has_entry("./test.txt"));
    }

    #[test]
    fn test_location_normalization() {
        let mut archive = CombineArchive::new();

        // Add with "./" prefix
        archive
            .add_entry("./test.txt", "text/plain", false, b"content1".as_slice())
            .unwrap();

        // Should be accessible both ways
        assert!(archive.has_entry("./test.txt"));
        let entry = archive.entry("./test.txt").unwrap();
        assert_eq!(entry.as_string().unwrap(), "content1");
    }

    #[test]
    fn test_overwrite_entry() {
        let mut archive = CombineArchive::new();

        // Add initial entry
        archive
            .add_entry("./test.txt", "text/plain", false, b"original".as_slice())
            .unwrap();

        // Overwrite with new content
        archive
            .add_entry("./test.txt", "text/plain", false, b"updated".as_slice())
            .unwrap();

        // Should have updated content
        let entry = archive.entry("./test.txt").unwrap();
        assert_eq!(entry.as_string().unwrap(), "updated");
        assert_eq!(archive.list_entries().len(), 1); // Should not duplicate
    }

    #[test]
    fn test_binary_data() {
        let mut archive = CombineArchive::new();
        let binary_data = vec![0u8, 1, 2, 3, 255, 254, 253];

        archive
            .add_entry(
                "./binary.dat",
                "application/octet-stream",
                false,
                binary_data.as_slice(),
            )
            .unwrap();

        let entry = archive.entry("./binary.dat").unwrap();
        assert_eq!(entry.as_bytes(), binary_data.as_slice());

        // String conversion should fail for binary data
        assert!(entry.as_string().is_err());
    }

    #[test]
    fn test_large_archive_operations() {
        let temp_dir = create_test_dir();
        let archive_path = temp_dir.path().join("large.omex");

        // Create archive with many entries
        let mut archive = CombineArchive::new();
        for i in 0..100 {
            archive
                .add_entry(
                    format!("./file{:03}.txt", i),
                    "text/plain",
                    i == 0, // First file is master
                    format!("Content of file number {}", i).as_bytes(),
                )
                .unwrap();
        }

        archive.save(&archive_path).unwrap();

        // Load and verify all entries
        let mut loaded = CombineArchive::open(&archive_path).unwrap();
        assert_eq!(loaded.list_entries().len(), 100);

        // Verify random entries
        for i in [0, 25, 50, 75, 99] {
            let entry = loaded.entry(&format!("./file{:03}.txt", i)).unwrap();
            assert_eq!(
                entry.as_string().unwrap(),
                format!("Content of file number {}", i)
            );
        }

        // Remove half the entries
        for i in (0..100).step_by(2) {
            loaded.remove_entry(&format!("./file{:03}.txt", i)).unwrap();
        }

        assert_eq!(loaded.list_entries().len(), 50);
        loaded.save_changes().unwrap();

        // Reload and verify
        let final_archive = CombineArchive::open(&archive_path).unwrap();
        assert_eq!(final_archive.list_entries().len(), 50);
    }

    #[test]
    fn test_update_entry_same_format() {
        let mut archive = CombineArchive::new();

        // Add initial entry
        archive
            .add_entry(
                "./test.txt",
                "text/plain",
                false,
                b"original content".as_slice(),
            )
            .unwrap();

        assert_eq!(archive.list_entries().len(), 1);
        let entry = archive.entry("./test.txt").unwrap();
        assert_eq!(entry.as_string().unwrap(), "original content");

        // Update with same format - should update content, keep manifest entry
        archive
            .add_entry(
                "./test.txt",
                "text/plain",
                false,
                b"updated content".as_slice(),
            )
            .unwrap();

        // Should still have only one entry
        assert_eq!(archive.list_entries().len(), 1);
        let entry = archive.entry("./test.txt").unwrap();
        assert_eq!(entry.as_string().unwrap(), "updated content");
        assert_eq!(entry.content.format, "text/plain");
        assert!(!entry.content.master);
    }

    #[test]
    fn test_update_entry_different_format() {
        let mut archive = CombineArchive::new();

        // Add initial entry
        archive
            .add_entry(
                "./test.txt",
                "text/plain",
                false,
                b"original content".as_slice(),
            )
            .unwrap();

        assert_eq!(archive.list_entries().len(), 1);

        // Update with different format - should replace manifest entry
        archive
            .add_entry(
                "./test.txt",
                "application/json",
                false,
                b"{\"updated\": true}".as_slice(),
            )
            .unwrap();

        // Should still have only one entry but with new format
        assert_eq!(archive.list_entries().len(), 1);
        let entry = archive.entry("./test.txt").unwrap();
        assert_eq!(entry.as_string().unwrap(), "{\"updated\": true}");
        assert_eq!(entry.content.format, "application/json");
        assert!(!entry.content.master);
    }

    #[test]
    fn test_update_entry_different_master_flag() {
        let mut archive = CombineArchive::new();

        // Add initial entry as non-master
        archive
            .add_entry("./test.txt", "text/plain", false, b"content".as_slice())
            .unwrap();

        assert_eq!(archive.list_entries().len(), 1);
        assert!(!archive.entry("./test.txt").unwrap().content.master);

        // Update with same format but different master flag
        archive
            .add_entry(
                "./test.txt",
                "text/plain",
                true,
                b"master content".as_slice(),
            )
            .unwrap();

        // Should still have only one entry but now as master
        assert_eq!(archive.list_entries().len(), 1);
        let entry = archive.entry("./test.txt").unwrap();
        assert_eq!(entry.as_string().unwrap(), "master content");
        assert_eq!(entry.content.format, "text/plain");
        assert!(entry.content.master);
    }

    #[test]
    fn test_end_to_end_with_updates() {
        let temp_dir = create_test_dir();
        let archive_path = temp_dir.path().join("updates.omex");

        // Create initial archive
        let mut archive = CombineArchive::new();
        archive
            .add_entry(
                "./model.xml",
                "application/xml",
                true,
                b"<model>v1</model>".as_slice(),
            )
            .unwrap();
        archive
            .add_entry("./data.csv", "text/csv", false, b"a,b\n1,2".as_slice())
            .unwrap();
        archive.save(&archive_path).unwrap();

        // Load and update entries
        let mut archive = CombineArchive::open(&archive_path).unwrap();

        // Update model with same format (should preserve manifest entry)
        archive
            .add_entry(
                "./model.xml",
                "application/xml",
                true,
                b"<model>v2</model>".as_slice(),
            )
            .unwrap();

        // Update data with different format (should replace manifest entry)
        archive
            .add_entry(
                "./data.csv",
                "application/json",
                false,
                b"{\"data\": [1,2,3]}".as_slice(),
            )
            .unwrap();

        archive.save_changes().unwrap();

        // Reload and verify
        let mut final_archive = CombineArchive::open(&archive_path).unwrap();
        assert_eq!(final_archive.list_entries().len(), 2);

        let model = final_archive.entry("./model.xml").unwrap();
        assert_eq!(model.as_string().unwrap(), "<model>v2</model>");
        assert_eq!(model.content.format, "application/xml");
        assert!(model.content.master);

        let data = final_archive.entry("./data.csv").unwrap();
        assert_eq!(data.as_string().unwrap(), "{\"data\": [1,2,3]}");
        assert_eq!(data.content.format, "application/json");
        assert!(!data.content.master);
    }
}
