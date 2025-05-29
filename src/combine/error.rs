/// Errors that can occur when working with COMBINE Archives.
#[derive(Debug, thiserror::Error)]
pub enum CombineArchiveError {
    /// I/O error (file reading, writing, etc.)
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// ZIP archive error (corruption, invalid format, etc.)
    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),

    /// Manifest parsing or serialization error
    #[error("Manifest error: {0}")]
    Manifest(#[from] quick_xml::DeError),

    /// Requested file not found in archive
    #[error("File not found: {0}")]
    FileNotFound(String),

    /// No files found with the specified format
    #[error("No files found with format: {0}")]
    FileFormatNotFound(String),

    /// No master file defined in the archive
    #[error("Master file not found")]
    MasterFileNotFound,

    /// Attempted to add an entry that already exists
    #[error("Location already exists: {0}")]
    LocationAlreadyExists(String),

    /// Attempted to save changes but no file path is available
    #[error("No file path specified for saving")]
    NoPath,
}
