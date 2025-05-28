//! SBML Package handling
//!
//! This module provides types for working with SBML extension packages like FBC (Flux Balance Constraints).
//! SBML packages extend the core SBML functionality with domain-specific features.

use crate::namespaces::SBMLNamespaces;

/// Represents an SBML extension package with its version.
///
/// SBML packages extend the core SBML functionality with domain-specific features.
/// Currently supported packages:
/// - FBC (Flux Balance Constraints) - for constraint-based modeling
#[derive(Debug, Clone, Copy)]
pub enum Package {
    /// Flux Balance Constraints package with specified version
    Fbc(u32),
}

impl From<Package> for PackageSpec {
    fn from(package: Package) -> Self {
        match package {
            Package::Fbc(version) => PackageSpec::new("fbc", version, "fbc"),
        }
    }
}

/// Detailed specification of an SBML package including name, version, and XML prefix.
///
/// This struct contains the necessary information to add a package to an SBML model's
/// namespaces, enabling the use of package-specific elements and attributes.
#[derive(Debug, Clone)]
pub struct PackageSpec {
    /// The name of the package (e.g., "fbc")
    pub(crate) name: String,
    /// The version of the package
    pub(crate) version: u32,
    /// The XML prefix used for package elements (e.g., "fbc")
    pub(crate) prefix: String,
}

impl PackageSpec {
    /// Creates a new package specification with the given name, version, and prefix.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the package (e.g., "fbc")
    /// * `version` - The version of the package
    /// * `prefix` - The XML prefix used for package elements
    ///
    /// # Returns
    ///
    /// A new `PackageSpec` instance
    pub fn new(name: &str, version: u32, prefix: &str) -> Self {
        Self {
            name: name.to_string(),
            version,
            prefix: prefix.to_string(),
        }
    }

    /// Adds this package to the given SBML namespaces.
    ///
    /// This enables the use of package-specific elements and attributes in the SBML model.
    ///
    /// # Arguments
    ///
    /// * `namespaces` - The SBML namespaces to add this package to
    pub fn add_to_namespace(&self, namespaces: &mut SBMLNamespaces) {
        namespaces.add_package(self.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_spec() {
        let package = PackageSpec::new("fbc", 1, "fbc");
        assert_eq!(package.name, "fbc");
        assert_eq!(package.version, 1);
        assert_eq!(package.prefix, "fbc");
    }

    #[test]
    fn test_package_spec_into() {
        let package = Package::Fbc(1);
        let package_spec: PackageSpec = package.into();
        assert_eq!(package_spec.name, "fbc");
        assert_eq!(package_spec.version, 1);
    }

    #[test]
    fn test_package_spec_add_to_namespace() {
        let mut namespaces = SBMLNamespaces::new(3, 2);
        let package: PackageSpec = Package::Fbc(1).into();
        package.add_to_namespace(&mut namespaces);
        assert_eq!(namespaces.package_name(), "core");
    }
}
