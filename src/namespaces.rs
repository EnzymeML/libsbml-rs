use std::{cell::RefCell, collections::HashMap};

use autocxx::WithinUniquePtr;
use cxx::{let_cxx_string, UniquePtr};

use crate::{packages::PackageSpec, pin_const_ptr, sbmlcxx};

/// Represents SBML namespaces that define the SBML level, version, and packages used in a model.
///
/// SBML namespaces are used to specify which version of SBML is being used and which
/// extension packages are included in a model.
pub struct SBMLNamespaces {
    inner: RefCell<UniquePtr<sbmlcxx::SBMLNamespaces>>,
}

impl SBMLNamespaces {
    /// Creates a new SBML namespace with the specified level and version.
    ///
    /// # Arguments
    ///
    /// * `level` - The SBML level (typically 1, 2, or 3)
    /// * `version` - The SBML version within that level
    ///
    /// # Returns
    ///
    /// A new `SBMLNamespaces` instance
    pub fn new(level: u32, version: u32) -> Self {
        let namespaces =
            sbmlcxx::SBMLNamespaces::new(level.into(), version.into()).within_unique_ptr();

        Self {
            inner: RefCell::new(namespaces),
        }
    }

    /// Returns the inner unique pointer to the SBML namespaces.
    pub(crate) fn inner(&self) -> &RefCell<UniquePtr<sbmlcxx::SBMLNamespaces>> {
        &self.inner
    }

    /// Returns the SBML level of the namespaces.
    ///
    /// # Returns
    ///
    /// The SBML level of the namespaces
    pub fn level(&self) -> u32 {
        self.inner.borrow().as_ref().unwrap().getLevel1().into()
    }

    /// Returns the SBML version of the namespaces.
    ///
    /// # Returns
    ///
    /// The SBML version of the namespaces
    pub fn version(&self) -> u32 {
        self.inner.borrow().as_ref().unwrap().getVersion1().into()
    }

    /// Adds a package to the SBML namespaces.
    ///
    /// # Arguments
    ///
    /// * `package` - The package to add
    pub fn add_package(&self, package: impl Into<PackageSpec>) {
        let package = package.into();
        let_cxx_string!(name = package.name.clone());
        let_cxx_string!(prefix = package.prefix.clone());
        self.inner
            .borrow_mut()
            .as_mut()
            .unwrap()
            .addPackageNamespace(&name, package.version.into(), &prefix);
    }

    /// Returns the package name of the SBML namespaces.
    ///
    /// This will always return `core` for the core package, but yeah,
    /// for the sake of completeness, it's here :'-)
    ///
    /// # Returns
    ///
    /// The package name of the SBML namespaces
    pub fn package_name(&self) -> String {
        self.inner
            .borrow()
            .as_ref()
            .unwrap()
            .getPackageName()
            .to_string()
    }

    /// Returns a map of prefixes to URIs for the SBML namespaces.
    ///
    /// This will return a map of prefixes to URIs for the SBML namespaces.
    ///
    /// # Returns
    ///
    /// A map of prefixes to URIs for the SBML namespaces
    pub fn prefixes(&self) -> HashMap<String, String> {
        let namespaces = self.inner.borrow().as_ref().unwrap().getNamespaces1();
        let xml_namespaces = pin_const_ptr!(namespaces, sbmlcxx::XMLNamespaces);
        let mut namespaces = HashMap::new();
        let num_namespaces = xml_namespaces.as_ref().getNumNamespaces().into();
        for i in 0..num_namespaces {
            let prefix = xml_namespaces.as_ref().getPrefix(i.into());
            let uri = xml_namespaces.as_ref().getURI(i.into());
            namespaces.insert(prefix.to_string(), uri.to_string());
        }

        namespaces
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let namespaces = SBMLNamespaces::new(3, 2);
        assert_eq!(namespaces.level(), 3);
        assert_eq!(namespaces.version(), 2);
        assert_eq!(namespaces.package_name(), "core");
    }

    #[test]
    fn test_prefixes() {
        let namespaces = SBMLNamespaces::new(3, 2);
        namespaces.add_package(PackageSpec::new("fbc", 1, "fbc"));
        let prefixes = namespaces.prefixes();

        assert_eq!(
            prefixes.get("fbc"),
            Some(&"http://www.sbml.org/sbml/level3/version1/fbc/version1".to_string())
        );

        assert_eq!(
            prefixes.get(""),
            Some(&"http://www.sbml.org/sbml/level3/version2/core".to_string())
        );
    }
}
