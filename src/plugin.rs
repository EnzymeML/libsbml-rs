//! Utilities for working with SBML plugins.
//!
//! This module provides functions for retrieving and managing SBML plugins,
//! which extend the core SBML functionality with additional features.

use std::pin::Pin;

use cxx::let_cxx_string;

use crate::{errors::LibSBMLError, pin_ptr, sbmlcxx, traits::sbase::SBase, upcast_pin};

/// Retrieves a plugin from an SBML object by name and casts it to the specified type.
///
/// # Type Parameters
/// * `'a` - The lifetime of the SBML object
/// * `T` - The target plugin type to cast to
/// * `H` - The type of the SBML object that implements the SBase trait
///
/// # Arguments
/// * `obj` - The SBML object to get the plugin from
/// * `plugin_name` - The name of the plugin to retrieve (e.g., "fbc", "layout")
///
/// # Returns
/// * `Result<Pin<&'a mut T>, LibSBMLError>` - A pinned mutable reference to the plugin
///   or an error if the plugin was not found
///
/// # Errors
/// * `LibSBMLError::PluginNotFound` - If the requested plugin is not available
pub(crate) fn get_plugin<'a, T, H, U>(
    obj: &H,
    plugin_name: &str,
) -> Result<Pin<&'a mut T>, LibSBMLError>
where
    H: SBase<'a, U> + 'a,
{
    let_cxx_string!(pkg = plugin_name);

    // Upcast the model to SBase d
    let sbase = obj.base();

    // Get the plugin
    let plugin_ptr = sbase.getPlugin(&pkg);

    if plugin_ptr.is_null() {
        return Err(LibSBMLError::PluginNotFound(plugin_name.to_string()));
    }

    let mut plugin = pin_ptr!(plugin_ptr, sbmlcxx::SBasePlugin);

    // Downcast the plugin to the desired type
    Ok(upcast_pin!(plugin, sbmlcxx::SBasePlugin, T))
}
