//! This module provides a safe Rust interface to the libSBML Species class.
//!
//! The Species class represents a chemical or biological entity in an SBML model.
//! It can represent molecules, ions, proteins, or any other entity that participates
//! in reactions. Each species can have properties like initial amount/concentration,
//! boundary conditions, and compartment location.
//!
//! This wrapper provides safe access to the underlying C++ libSBML Species class while
//! maintaining Rust's safety guarantees through the use of RefCell and Pin.

use std::{cell::RefCell, pin::Pin, rc::Rc, str::FromStr};

use autocxx::c_int;

use crate::{
    clone, inner, pin_ptr, sbmlcxx, sbo_term, traits::fromptr::FromPtr, unitdef::UnitDefinition,
    upcast_annotation,
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
    inner: RefCell<Pin<&'a mut sbmlcxx::Unit>>,
}

// Set the inner trait for the Unit struct
inner!(sbmlcxx::Unit, Unit<'a>);

// Set the annotation trait for the Unit struct
upcast_annotation!(Unit<'a>, sbmlcxx::Unit, sbmlcxx::SBase);

// Implement the Clone trait for the Unit struct
clone!(Unit<'a>, sbmlcxx::Unit);

impl<'a> Unit<'a> {
    /// Creates a new Unit instance within the given Model.
    ///
    /// # Arguments
    /// * `unit_definition` - The parent UnitDefinition that will contain this unit
    /// * `kind` - The kind of unit to create
    ///
    /// # Returns
    /// A new Unit instance
    pub fn new(unit_definition: &UnitDefinition, kind: UnitKind) -> Self {
        let unit_ptr = unit_definition.inner().borrow_mut().as_mut().createUnit();
        let mut unit = pin_ptr!(unit_ptr, sbmlcxx::Unit);

        // Set the default values first
        unit.as_mut().initDefaults();

        // Set the kind after defaults
        let kind = kind.into();
        unit.as_mut().setKind(kind);

        Self {
            inner: RefCell::new(unit),
        }
    }

    /// Gets the kind of unit.
    ///
    /// # Returns
    /// The UnitKind enum value representing this unit's type
    pub fn kind(&self) -> UnitKind {
        let kind = self.inner.borrow().getKind();
        UnitKind::from(kind)
    }

    /// Sets the kind of unit.
    ///
    /// # Arguments
    /// * `kind` - The new UnitKind to set for this unit
    pub fn set_kind(&self, kind: UnitKind) {
        let kind = kind.into();
        self.inner.borrow_mut().as_mut().setKind(kind);
    }

    /// Gets the exponent of the unit.
    ///
    /// The exponent is used to indicate the power to which the unit is raised.
    /// For example, an exponent of 2 means the unit is squared.
    ///
    /// # Returns
    /// The exponent value as an i32
    pub fn exponent(&self) -> i32 {
        self.inner.borrow().getExponent().into()
    }

    /// Sets the exponent of the unit.
    ///
    /// # Arguments
    /// * `exponent` - The new exponent value to set
    pub fn set_exponent(&self, exponent: i32) {
        self.inner
            .borrow_mut()
            .as_mut()
            .setExponent(c_int::from(exponent));
    }

    /// Gets the multiplier of the unit.
    ///
    /// The multiplier is a scaling factor applied to the unit. For example,
    /// a multiplier of 1000 would represent kilo-units.
    ///
    /// # Returns
    /// The multiplier value as an f64
    pub fn multiplier(&self) -> f64 {
        self.inner.borrow().getMultiplier()
    }

    /// Sets the multiplier of the unit.
    ///
    /// # Arguments
    /// * `multiplier` - The new multiplier value to set
    pub fn set_multiplier(&self, multiplier: f64) {
        self.inner.borrow_mut().as_mut().setMultiplier(multiplier);
    }

    /// Gets the scale of the unit.
    ///
    /// The scale is an integer used to set the scale of the unit (e.g., milli, micro, etc.).
    /// It represents a power of ten: scale = -3 represents 10^-3 (milli).
    ///
    /// # Returns
    /// The scale value as an i32
    pub fn scale(&self) -> i32 {
        self.inner.borrow().getScale().into()
    }

    /// Sets the scale of the unit.
    ///
    /// # Arguments
    /// * `scale` - The new scale value to set
    pub fn set_scale(&self, scale: i32) {
        self.inner
            .borrow_mut()
            .as_mut()
            .setScale(c_int::from(scale));
    }

    /// Gets the offset of the unit.
    ///
    /// The offset is used for units that have a different zero point than their
    /// base SI unit (e.g., degrees Celsius = Kelvin - 273.15).
    ///
    /// # Returns
    /// The offset value as an f64
    pub fn offset(&self) -> f64 {
        self.inner.borrow().getOffset()
    }

    /// Sets the offset of the unit.
    ///
    /// # Arguments
    /// * `offset` - The new offset value to set
    pub fn set_offset(&self, offset: f64) {
        self.inner.borrow_mut().as_mut().setOffset(offset);
    }

    // SBO Term Methods generated by the `sbo_term` macro
    sbo_term!(sbmlcxx::Unit, sbmlcxx::SBase);
}

impl FromPtr<sbmlcxx::Unit> for Unit<'_> {
    /// Creates a new Unit instance from a unique pointer to a libSBML Unit.
    ///
    /// This method is primarily used internally by the UnitDefinition class to create
    /// Unit instances from libSBML Unit pointers.
    ///
    /// # Arguments
    /// * `ptr` - A unique pointer to a libSBML Unit
    ///
    /// # Returns
    /// A new Unit instance
    fn from_ptr(ptr: *mut sbmlcxx::Unit) -> Self {
        let unit = pin_ptr!(ptr, sbmlcxx::Unit);
        Self {
            inner: RefCell::new(unit),
        }
    }
}
/// A builder for constructing Unit instances with a fluent interface.
///
/// This builder allows for step-by-step construction of a Unit with method chaining.
/// All unit properties (kind, exponent, multiplier, scale, offset) can be set through
/// the builder before finally constructing the Unit with build().
pub struct UnitBuilder<'a> {
    inner: Rc<Unit<'a>>,
}

impl<'a> UnitBuilder<'a> {
    /// Creates a new UnitBuilder instance.
    ///
    /// # Arguments
    /// * `unit_definition` - The UnitDefinition that will contain the Unit being built
    /// * `kind` - The UnitKind to set
    ///
    /// # Returns
    /// A new UnitBuilder initialized with a dimensionless Unit
    pub fn new(unit_definition: &UnitDefinition<'a>, kind: UnitKind) -> Self {
        let unit = unit_definition.create_unit(kind);
        Self { inner: unit }
    }

    /// Sets the kind of unit being built.
    ///
    /// # Arguments
    /// * `kind` - The UnitKind to set
    ///
    /// # Returns
    /// The builder for method chaining
    pub fn kind(self, kind: UnitKind) -> Self {
        self.inner.set_kind(kind);
        self
    }

    /// Sets the exponent of the unit.
    ///
    /// # Arguments
    /// * `exponent` - The exponent value to set
    ///
    /// # Returns
    /// The builder for method chaining
    pub fn exponent(self, exponent: i32) -> Self {
        self.inner.set_exponent(exponent);
        self
    }

    /// Sets the multiplier of the unit.
    ///
    /// # Arguments
    /// * `multiplier` - The multiplier value to set
    ///
    /// # Returns
    /// The builder for method chaining
    pub fn multiplier(self, multiplier: f64) -> Self {
        self.inner.set_multiplier(multiplier);
        self
    }

    /// Sets the scale of the unit.
    ///
    /// # Arguments
    /// * `scale` - The scale value to set
    ///
    /// # Returns
    /// The builder for method chaining
    pub fn scale(self, scale: i32) -> Self {
        self.inner.set_scale(scale);
        self
    }

    /// Sets the offset of the unit.
    ///
    /// # Arguments
    /// * `offset` - The offset value to set
    ///
    /// # Returns
    /// The builder for method chaining
    pub fn offset(self, offset: f64) -> Self {
        self.inner.set_offset(offset);
        self
    }

    /// Builds and returns the constructed Unit.
    ///
    /// # Returns
    /// The fully constructed Unit wrapped in an Rc
    pub fn build(self) -> Rc<Unit<'a>> {
        self.inner
    }
}

/// Represents the different types of units that can be used in an SBML model.
///
/// This enum defines the various types of units that can be used in an SBML model,
/// including physical quantities like amperes, coulombs, and joules, as well as
/// dimensionless quantities like mole, item, and steradian.
///
/// This simply wraps the libSBML UnitKind_t enum for more concise enum variants.
#[derive(Debug, PartialEq, Clone, Copy)]
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

#[cfg(test)]
mod tests {
    use crate::SBMLDocument;

    use super::*;

    #[test]
    fn test_unit_new() {
        let doc = SBMLDocument::new(3, 2);
        let model = doc.create_model("test");
        let unit_definition = model.build_unit_definition("test", "test").build();
        let unit = Unit::new(&unit_definition, UnitKind::Ampere);
        unit.set_kind(UnitKind::Avogadro);
        unit.set_exponent(1);
        unit.set_multiplier(1.0);
        unit.set_scale(0);
        unit.set_offset(0.0);

        assert_eq!(unit.kind(), UnitKind::Avogadro);
        assert_eq!(unit.exponent(), 1);
        assert_eq!(unit.multiplier(), 1.0);
        assert_eq!(unit.scale(), 0);
    }

    #[test]
    fn test_get_inner() {
        let doc = SBMLDocument::new(3, 2);
        let model = doc.create_model("test");
        let unit_definition = model.build_unit_definition("test", "test").build();
        let unit = Unit::new(&unit_definition, UnitKind::Ampere);
        unit.set_exponent(1);
        unit.set_multiplier(1.0);
        unit.set_scale(0);
        unit.set_offset(0.0);

        let inner = unit.inner();

        assert_eq!(inner.borrow().getExponent(), c_int(1));
        assert_eq!(inner.borrow().getMultiplier(), 1.0);
        assert_eq!(inner.borrow().getScale(), c_int(0));
        assert_eq!(inner.borrow().getOffset(), 0.0);
    }

    #[test]
    fn test_unit_builder() {
        let doc = SBMLDocument::new(3, 2);
        let model = doc.create_model("test");
        let unit_definition = model.build_unit_definition("test", "test").build();
        let unit = UnitBuilder::new(&unit_definition, UnitKind::Ampere)
            .exponent(1)
            .multiplier(1.0)
            .scale(0)
            .offset(0.0)
            .build();

        assert_eq!(unit.kind(), UnitKind::Ampere);
        assert_eq!(unit.exponent(), 1);
        assert_eq!(unit.multiplier(), 1.0);
        assert_eq!(unit.scale(), 0);
        assert_eq!(unit.offset(), 0.0);
    }

    #[test]
    fn test_unit_kind_from_str() {
        assert_eq!(UnitKind::from_str("ampere").unwrap(), UnitKind::Ampere);
        assert_eq!(
            UnitKind::from_str("becquerel").unwrap(),
            UnitKind::Becquerel
        );
        assert_eq!(UnitKind::from_str("candela").unwrap(), UnitKind::Candela);
        assert_eq!(UnitKind::from_str("coulomb").unwrap(), UnitKind::Coulomb);
        assert_eq!(
            UnitKind::from_str("dimensionless").unwrap(),
            UnitKind::Dimensionless
        );
        assert_eq!(UnitKind::from_str("farad").unwrap(), UnitKind::Farad);
        assert_eq!(UnitKind::from_str("avogadro").unwrap(), UnitKind::Avogadro);
        assert_eq!(UnitKind::from_str("gram").unwrap(), UnitKind::Gram);
        assert_eq!(UnitKind::from_str("gray").unwrap(), UnitKind::Gray);
        assert_eq!(UnitKind::from_str("henry").unwrap(), UnitKind::Henry);
        assert_eq!(UnitKind::from_str("hertz").unwrap(), UnitKind::Hertz);
        assert_eq!(UnitKind::from_str("item").unwrap(), UnitKind::Item);
        assert_eq!(UnitKind::from_str("joule").unwrap(), UnitKind::Joule);
        assert_eq!(UnitKind::from_str("katal").unwrap(), UnitKind::Katal);
        assert_eq!(UnitKind::from_str("kelvin").unwrap(), UnitKind::Kelvin);
        assert_eq!(UnitKind::from_str("kilogram").unwrap(), UnitKind::Kilogram);
        assert_eq!(UnitKind::from_str("liter").unwrap(), UnitKind::Liter);
        assert_eq!(UnitKind::from_str("lumen").unwrap(), UnitKind::Lumen);
        assert_eq!(UnitKind::from_str("lux").unwrap(), UnitKind::Lux);
        assert_eq!(UnitKind::from_str("metre").unwrap(), UnitKind::Metre);
        assert_eq!(UnitKind::from_str("mole").unwrap(), UnitKind::Mole);
        assert_eq!(UnitKind::from_str("newton").unwrap(), UnitKind::Newton);
        assert_eq!(UnitKind::from_str("ohm").unwrap(), UnitKind::Ohm);
        assert_eq!(UnitKind::from_str("pascal").unwrap(), UnitKind::Pascal);
        assert_eq!(UnitKind::from_str("radian").unwrap(), UnitKind::Radian);
        assert_eq!(UnitKind::from_str("second").unwrap(), UnitKind::Second);
        assert_eq!(UnitKind::from_str("siemens").unwrap(), UnitKind::Siemens);
        assert_eq!(UnitKind::from_str("sievert").unwrap(), UnitKind::Sievert);
        assert_eq!(
            UnitKind::from_str("steradian").unwrap(),
            UnitKind::Steradian
        );
        assert_eq!(UnitKind::from_str("tesla").unwrap(), UnitKind::Tesla);
        assert_eq!(UnitKind::from_str("volt").unwrap(), UnitKind::Volt);
        assert_eq!(UnitKind::from_str("watt").unwrap(), UnitKind::Watt);
        assert_eq!(UnitKind::from_str("weber").unwrap(), UnitKind::Weber);
        assert!(UnitKind::from_str("invalid").is_err());
    }

    #[test]
    fn test_unit_kind_conversion() {
        let kind = UnitKind::Ampere;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Becquerel;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Candela;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Celsius;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Coulomb;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Dimensionless;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Farad;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Gram;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Gray;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Henry;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Hertz;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Invalid;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Item;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Joule;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Katal;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Kelvin;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Kilogram;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Liter;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Litre;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Lumen;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Lux;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Meter;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Metre;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Mole;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Newton;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Ohm;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Pascal;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Radian;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Second;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Siemens;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Sievert;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Steradian;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Tesla;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Volt;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Watt;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Weber;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Invalid;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));

        let kind = UnitKind::Avogadro;
        let sbml_kind: sbmlcxx::UnitKind_t = kind.into();
        assert_eq!(kind, UnitKind::from(sbml_kind));
    }
}
