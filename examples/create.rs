use sbml::{prelude::*, unit::UnitKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doc = SBMLDocument::new(3, 2);

    // Create a model
    let model = doc.create_model("example");

    // Create a unit definition
    let mole = model
        .build_unit_definition("mole", "mole")
        .unit(UnitKind::Mole, Some(1), None, None, None)
        .build();

    let ml = model
        .build_unit_definition("ml", "ml")
        .unit(UnitKind::Litre, Some(1), Some(-3), None, None)
        .build();

    // Create a compartment
    let compartment = model
        .build_compartment("cytosol")
        .name("cytosol")
        .unit(&ml.id())
        .build();

    // Create the ethanol species
    let ethanol = model
        .build_species("ethanol")
        .name("Ethanol")
        .compartment(&compartment)
        .initial_concentration(0.5)
        .unit(&mole.id())
        .has_only_substance_units(false)
        .build();

    // Create the aldehyde species
    let aldehyde = model
        .build_species("aldehyde")
        .name("Aldehyde")
        .compartment(&compartment)
        .initial_concentration(0.5)
        .unit(&mole.id())
        .has_only_substance_units(false)
        .build();

    // Create the reaction
    model
        .build_reaction("reaction")
        .name("Reaction")
        .reactant(&ethanol, 1.0)
        .product(&aldehyde, 1.0)
        .build();

    // Serialize the document to an SBML string
    let sbml_string = doc.to_xml_string();

    // Print the SBML string
    println!("{}", sbml_string);

    Ok(())
}
