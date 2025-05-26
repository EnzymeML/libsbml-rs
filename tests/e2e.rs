#[cfg(test)]
mod tests {
    use sbml::prelude::*;

    #[test]
    fn test_sbmldoc_debug() {
        let doc = create_doc();
        let debug_string = format!("{:?}", doc);
        insta::assert_snapshot!(debug_string, @r#"SBMLDocument { level: 3, version: 2, model: Some(Model { id: "test_model", name: "", list_of_species: [Species { id: "species", name: Some("species"), compartment: Some("compartment"), initial_amount: None, initial_concentration: Some(1.0), unit: Some("mole"), boundary_condition: Some(false), constant: false, has_only_substance_units: Some(false) }, Species { id: "product", name: Some("product"), compartment: Some("compartment"), initial_amount: None, initial_concentration: Some(1.0), unit: Some("mole"), boundary_condition: Some(false), constant: false, has_only_substance_units: Some(false) }], list_of_compartments: [Compartment { id: "compartment", name: Some("compartment"), spatial_dimensions: None, unit: Some("ml"), size: Some(1.0), volume: Some(1.0), outside: None, constant: Some(true) }], list_of_unit_definitions: [UnitDefinition { id: "ml", name: Some("milliliter"), units: [Unit { kind: Litre, exponent: 1, multiplier: 1.0, scale: -3, offset: 0.0 }] }, UnitDefinition { id: "mole", name: Some("mole"), units: [Unit { kind: Mole, exponent: 1, multiplier: 1.0, scale: 0, offset: 0.0 }, Unit { kind: Litre, exponent: -1, multiplier: 1.0, scale: 0, offset: 0.0 }] }, UnitDefinition { id: "kelvin", name: Some("kelvin"), units: [Unit { kind: Kelvin, exponent: 1, multiplier: 1.0, scale: 0, offset: 0.0 }] }], list_of_reactions: [Reaction { id: "reaction", name: Some("reaction"), reversible: None, compartment: None, reactants: RefCell { value: [SpeciesReference { species: "species", stoichiometry: 1.0, constant: false }] }, products: RefCell { value: [SpeciesReference { species: "product", stoichiometry: 1.0, constant: false }] }, modifiers: RefCell { value: [] } }], list_of_parameters: [Parameter { id: "T", name: None, value: Some(310.0), units: Some("kelvin"), constant: Some(true) }, Parameter { id: "Km", name: None, value: Some(1.0), units: Some("mole"), constant: Some(true) }], list_of_rate_rules: [Rule { type: Ok(RateRule), variable: "product", formula: "kcat * substrate / (substrate + Km)" }], list_of_assignment_rules: [Rule { type: Ok(AssignmentRule), variable: "x", formula: "T * kcat * substrate / (T + Km)" }], list_of_objectives: [], list_of_flux_bounds: [FluxBound { id: Some("fb1"), reaction: Some("reaction"), operation: LessEqual }] }) }"#);
    }

    #[test]
    fn test_sbmldoc_write_and_read() {
        // Arrange
        let doc = create_doc();

        // Act
        let xml_string = doc.to_xml_string();
        let doc2 = SBMLReader::from_xml_string(&xml_string);
        let xml_string2 = doc2.to_xml_string();

        // Assert
        assert_eq!(xml_string, xml_string2);
    }

    fn create_doc() -> SBMLDocument<'static> {
        let doc = SBMLDocument::default();
        let model = doc.create_model("test_model");

        // Add a unit definition
        let ml = model
            .build_unit_definition("ml", "milliliter")
            .unit(UnitKind::Litre, Some(1), Some(-3), None, None)
            .build();

        let mole_l = model
            .build_unit_definition("mole", "mole")
            .unit(UnitKind::Mole, Some(1), Some(0), Some(1.0), None)
            .unit(UnitKind::Litre, Some(-1), Some(0), Some(1.0), None)
            .build();

        let kelvin = model
            .build_unit_definition("kelvin", "kelvin")
            .unit(UnitKind::Kelvin, Some(1), Some(0), Some(1.0), None)
            .build();

        // Add a compartment
        let compartment = model
            .build_compartment("compartment")
            .name("compartment")
            .constant(true)
            .unit(&ml)
            .volume(1.0)
            .build();

        // Add a species
        let substrate = model
            .build_species("species")
            .name("species")
            .constant(false)
            .has_only_substance_units(false)
            .compartment(&compartment)
            .initial_concentration(1.0)
            .unit(&mole_l)
            .build();

        let product = model
            .build_species("product")
            .name("product")
            .constant(false)
            .has_only_substance_units(false)
            .compartment(&compartment)
            .initial_concentration(1.0)
            .unit(&mole_l)
            .build();

        // Add a reaction
        let reaction = model
            .build_reaction("reaction")
            .name("reaction")
            .reactant(&substrate, 1.0)
            .product(&product, 1.0)
            .build();

        // Add a kinetic law
        let kinetic_law = reaction.create_kinetic_law("substrate * kcat / (substrate + Km)");

        // Add local parameters
        kinetic_law
            .build_local_parameter("kcat")
            .units(&mole_l.id())
            .value(1.0)
            .build();

        kinetic_law
            .build_local_parameter("Km")
            .units(&mole_l.id())
            .value(1.0)
            .build();

        // Add a parameter (global)
        model
            .build_parameter("T")
            .units(&kelvin)
            .value(310.0)
            .build();

        model
            .build_parameter("Km")
            .units(&mole_l)
            .value(1.0)
            .build();

        // Add a rate rule
        model
            .build_rate_rule(&product, "kcat * substrate / (substrate + Km)")
            .build();

        // Add an assignment rule
        model
            .build_assignment_rule("x", "T * kcat * substrate / (T + Km)")
            .build();

        // Add an objective
        let objective = Objective::new(&model, "objective", ObjectiveType::Maximize)
            .expect("Failed to create objective");

        objective
            .create_flux_objective("fo1", "reaction", 1.0)
            .expect("Failed to create flux objective");

        // Create the flux bound
        model
            .create_flux_bound("fb1", "reaction", FluxBoundOperation::LessEqual)
            .expect("Failed to create flux bound");

        doc
    }
}
