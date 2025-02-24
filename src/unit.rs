//! This module provides a safe Rust interface to the libSBML Species class.
//!
//! The Species class represents a chemical or biological entity in an SBML model.
//! It can represent molecules, ions, proteins, or any other entity that participates
//! in reactions. Each species can have properties like initial amount/concentration,
//! boundary conditions, and compartment location.
//!
//! This wrapper provides safe access to the underlying C++ libSBML Species class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin, str::FromStr};

use crate::{
    model::Model,
    pin_ptr,
    sbmlcxx::{self},
};

/// A safe wrapper around the libSBML Species class.
///
/// This struct maintains a reference to the underlying C++ Species object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
/// A safe wrapper around the libSBML Unit class.
///
/// This struct maintains a reference to the underlying C++ Unit object
/// through a RefCell and Pin to ensure memory safety while allowing interior mutability.
pub struct Unit<'a> {
    unit: RefCell<Pin<&'a mut sbmlcxx::Unit>>,
}

impl<'a> Unit<'a> {
    /// Creates a new Unit instance within the given Model.
    ///
    /// # Arguments
    /// * `model` - The parent Model that will contain this unit
    /// * `kind` - The kind of unit to create
    ///
    /// # Returns
    /// A new Unit instance
    pub fn new(model: &Model<'a>, kind: UnitKind) -> Self {
        let unit_ptr = model.inner().borrow_mut().as_mut().createUnit();
        let mut unit = pin_ptr!(unit_ptr, sbmlcxx::Unit);

        // Set the default
        unit.as_mut().initDefaults();

        // Set the kind
        let kind = kind.into();
        unit.as_mut().setKind(kind);

        Self {
            unit: RefCell::new(unit),
        }
    }

    /// Gets the kind of unit.
    ///
    /// # Returns
    /// The UnitKind enum value representing this unit's type
    pub fn kind(&self) -> UnitKind {
        let kind = self.unit.borrow().getKind();
        UnitKind::from(kind)
    }

    /// Sets the kind of unit.
    ///
    /// # Arguments
    /// * `kind` - The new UnitKind to set for this unit
    pub fn set_kind(&self, kind: UnitKind) {
        let kind = kind.into();
        self.unit.borrow_mut().as_mut().setKind(kind);
    }

    /// Returns a reference to the inner RefCell containing the Unit pointer.
    ///
    /// This is primarily used internally by other parts of the library.
    pub(crate) fn inner(&self) -> &RefCell<Pin<&'a mut sbmlcxx::Unit>> {
        &self.unit
    }
}

/// Represents the different types of units that can be used in an SBML model.
///
/// This enum defines the various types of units that can be used in an SBML model,
/// including physical quantities like amperes, coulombs, and joules, as well as
/// dimensionless quantities like mole, item, and steradian.
///
/// This simply wraps the libSBML UnitKind_t enum for more concise enum variants.
pub enum UnitKind {
    Ampere,
    Avogadro,
    Becquerel,
    Candela,
    Celsius,
    Coulomb,
    Dimensionless,
    Farad,
    Gram,
    Gray,
    Henry,
    Hertz,
    Item,
    Joule,
    Katal,
    Kelvin,
    Kilogram,
    Liter,
    Litre,
    Lumen,
    Lux,
    Meter,
    Metre,
    Mole,
    Newton,
    Ohm,
    Pascal,
    Radian,
    Second,
    Siemens,
    Sievert,
    Steradian,
    Tesla,
    Volt,
    Watt,
    Weber,
    Invalid,
}

impl From<UnitKind> for sbmlcxx::UnitKind_t {
    fn from(kind: UnitKind) -> Self {
        match kind {
            UnitKind::Ampere => sbmlcxx::UnitKind_t::UNIT_KIND_AMPERE,
            UnitKind::Avogadro => sbmlcxx::UnitKind_t::UNIT_KIND_AVOGADRO,
            UnitKind::Becquerel => sbmlcxx::UnitKind_t::UNIT_KIND_BECQUEREL,
            UnitKind::Candela => sbmlcxx::UnitKind_t::UNIT_KIND_CANDELA,
            UnitKind::Celsius => sbmlcxx::UnitKind_t::UNIT_KIND_CELSIUS,
            UnitKind::Coulomb => sbmlcxx::UnitKind_t::UNIT_KIND_COULOMB,
            UnitKind::Dimensionless => sbmlcxx::UnitKind_t::UNIT_KIND_DIMENSIONLESS,
            UnitKind::Farad => sbmlcxx::UnitKind_t::UNIT_KIND_FARAD,
            UnitKind::Gram => sbmlcxx::UnitKind_t::UNIT_KIND_GRAM,
            UnitKind::Gray => sbmlcxx::UnitKind_t::UNIT_KIND_GRAY,
            UnitKind::Henry => sbmlcxx::UnitKind_t::UNIT_KIND_HENRY,
            UnitKind::Hertz => sbmlcxx::UnitKind_t::UNIT_KIND_HERTZ,
            UnitKind::Item => sbmlcxx::UnitKind_t::UNIT_KIND_ITEM,
            UnitKind::Joule => sbmlcxx::UnitKind_t::UNIT_KIND_JOULE,
            UnitKind::Katal => sbmlcxx::UnitKind_t::UNIT_KIND_KATAL,
            UnitKind::Kelvin => sbmlcxx::UnitKind_t::UNIT_KIND_KELVIN,
            UnitKind::Kilogram => sbmlcxx::UnitKind_t::UNIT_KIND_KILOGRAM,
            UnitKind::Liter => sbmlcxx::UnitKind_t::UNIT_KIND_LITER,
            UnitKind::Litre => sbmlcxx::UnitKind_t::UNIT_KIND_LITRE,
            UnitKind::Lumen => sbmlcxx::UnitKind_t::UNIT_KIND_LUMEN,
            UnitKind::Lux => sbmlcxx::UnitKind_t::UNIT_KIND_LUX,
            UnitKind::Meter => sbmlcxx::UnitKind_t::UNIT_KIND_METER,
            UnitKind::Metre => sbmlcxx::UnitKind_t::UNIT_KIND_METRE,
            UnitKind::Mole => sbmlcxx::UnitKind_t::UNIT_KIND_MOLE,
            UnitKind::Newton => sbmlcxx::UnitKind_t::UNIT_KIND_NEWTON,
            UnitKind::Ohm => sbmlcxx::UnitKind_t::UNIT_KIND_OHM,
            UnitKind::Pascal => sbmlcxx::UnitKind_t::UNIT_KIND_PASCAL,
            UnitKind::Radian => sbmlcxx::UnitKind_t::UNIT_KIND_RADIAN,
            UnitKind::Second => sbmlcxx::UnitKind_t::UNIT_KIND_SECOND,
            UnitKind::Siemens => sbmlcxx::UnitKind_t::UNIT_KIND_SIEMENS,
            UnitKind::Sievert => sbmlcxx::UnitKind_t::UNIT_KIND_SIEVERT,
            UnitKind::Steradian => sbmlcxx::UnitKind_t::UNIT_KIND_STERADIAN,
            UnitKind::Tesla => sbmlcxx::UnitKind_t::UNIT_KIND_TESLA,
            UnitKind::Volt => sbmlcxx::UnitKind_t::UNIT_KIND_VOLT,
            UnitKind::Watt => sbmlcxx::UnitKind_t::UNIT_KIND_WATT,
            UnitKind::Weber => sbmlcxx::UnitKind_t::UNIT_KIND_WEBER,
            UnitKind::Invalid => sbmlcxx::UnitKind_t::UNIT_KIND_INVALID,
        }
    }
}

impl FromStr for UnitKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ampere" => Ok(UnitKind::Ampere),
            "avogadro" => Ok(UnitKind::Avogadro),
            "becquerel" => Ok(UnitKind::Becquerel),
            "candela" => Ok(UnitKind::Candela),
            "celsius" => Ok(UnitKind::Celsius),
            "coulomb" => Ok(UnitKind::Coulomb),
            "dimensionless" => Ok(UnitKind::Dimensionless),
            "farad" => Ok(UnitKind::Farad),
            "gram" => Ok(UnitKind::Gram),
            "gray" => Ok(UnitKind::Gray),
            "henry" => Ok(UnitKind::Henry),
            "hertz" => Ok(UnitKind::Hertz),
            "item" => Ok(UnitKind::Item),
            "joule" => Ok(UnitKind::Joule),
            "katal" => Ok(UnitKind::Katal),
            "kelvin" => Ok(UnitKind::Kelvin),
            "kilogram" => Ok(UnitKind::Kilogram),
            "liter" => Ok(UnitKind::Liter),
            "lumen" => Ok(UnitKind::Lumen),
            "lux" => Ok(UnitKind::Lux),
            "meter" => Ok(UnitKind::Meter),
            "metre" => Ok(UnitKind::Metre),
            "mole" => Ok(UnitKind::Mole),
            "newton" => Ok(UnitKind::Newton),
            "ohm" => Ok(UnitKind::Ohm),
            "pascal" => Ok(UnitKind::Pascal),
            "radian" => Ok(UnitKind::Radian),
            "second" => Ok(UnitKind::Second),
            "siemens" => Ok(UnitKind::Siemens),
            "sievert" => Ok(UnitKind::Sievert),
            "steradian" => Ok(UnitKind::Steradian),
            "tesla" => Ok(UnitKind::Tesla),
            "volt" => Ok(UnitKind::Volt),
            "watt" => Ok(UnitKind::Watt),
            "weber" => Ok(UnitKind::Weber),
            _ => Err(()),
        }
    }
}

impl From<sbmlcxx::UnitKind_t> for UnitKind {
    fn from(kind: sbmlcxx::UnitKind_t) -> Self {
        match kind {
            sbmlcxx::UnitKind_t::UNIT_KIND_AMPERE => UnitKind::Ampere,
            sbmlcxx::UnitKind_t::UNIT_KIND_AVOGADRO => UnitKind::Avogadro,
            sbmlcxx::UnitKind_t::UNIT_KIND_BECQUEREL => UnitKind::Becquerel,
            sbmlcxx::UnitKind_t::UNIT_KIND_CANDELA => UnitKind::Candela,
            sbmlcxx::UnitKind_t::UNIT_KIND_CELSIUS => UnitKind::Celsius,
            sbmlcxx::UnitKind_t::UNIT_KIND_COULOMB => UnitKind::Coulomb,
            sbmlcxx::UnitKind_t::UNIT_KIND_DIMENSIONLESS => UnitKind::Dimensionless,
            sbmlcxx::UnitKind_t::UNIT_KIND_FARAD => UnitKind::Farad,
            sbmlcxx::UnitKind_t::UNIT_KIND_GRAM => UnitKind::Gram,
            sbmlcxx::UnitKind_t::UNIT_KIND_GRAY => UnitKind::Gray,
            sbmlcxx::UnitKind_t::UNIT_KIND_HENRY => UnitKind::Henry,
            sbmlcxx::UnitKind_t::UNIT_KIND_HERTZ => UnitKind::Hertz,
            sbmlcxx::UnitKind_t::UNIT_KIND_ITEM => UnitKind::Item,
            sbmlcxx::UnitKind_t::UNIT_KIND_JOULE => UnitKind::Joule,
            sbmlcxx::UnitKind_t::UNIT_KIND_KATAL => UnitKind::Katal,
            sbmlcxx::UnitKind_t::UNIT_KIND_KELVIN => UnitKind::Kelvin,
            sbmlcxx::UnitKind_t::UNIT_KIND_KILOGRAM => UnitKind::Kilogram,
            sbmlcxx::UnitKind_t::UNIT_KIND_LITER => UnitKind::Liter,
            sbmlcxx::UnitKind_t::UNIT_KIND_LITRE => UnitKind::Litre,
            sbmlcxx::UnitKind_t::UNIT_KIND_LUMEN => UnitKind::Lumen,
            sbmlcxx::UnitKind_t::UNIT_KIND_LUX => UnitKind::Lux,
            sbmlcxx::UnitKind_t::UNIT_KIND_METER => UnitKind::Meter,
            sbmlcxx::UnitKind_t::UNIT_KIND_METRE => UnitKind::Metre,
            sbmlcxx::UnitKind_t::UNIT_KIND_MOLE => UnitKind::Mole,
            sbmlcxx::UnitKind_t::UNIT_KIND_NEWTON => UnitKind::Newton,
            sbmlcxx::UnitKind_t::UNIT_KIND_OHM => UnitKind::Ohm,
            sbmlcxx::UnitKind_t::UNIT_KIND_PASCAL => UnitKind::Pascal,
            sbmlcxx::UnitKind_t::UNIT_KIND_RADIAN => UnitKind::Radian,
            sbmlcxx::UnitKind_t::UNIT_KIND_SECOND => UnitKind::Second,
            sbmlcxx::UnitKind_t::UNIT_KIND_SIEMENS => UnitKind::Siemens,
            sbmlcxx::UnitKind_t::UNIT_KIND_SIEVERT => UnitKind::Sievert,
            sbmlcxx::UnitKind_t::UNIT_KIND_STERADIAN => UnitKind::Steradian,
            sbmlcxx::UnitKind_t::UNIT_KIND_TESLA => UnitKind::Tesla,
            sbmlcxx::UnitKind_t::UNIT_KIND_VOLT => UnitKind::Volt,
            sbmlcxx::UnitKind_t::UNIT_KIND_WATT => UnitKind::Watt,
            sbmlcxx::UnitKind_t::UNIT_KIND_WEBER => UnitKind::Weber,
            sbmlcxx::UnitKind_t::UNIT_KIND_INVALID => UnitKind::Invalid,
        }
    }
}
