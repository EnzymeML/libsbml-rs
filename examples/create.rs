use sbml::{combine::KnownFormats, prelude::*, unit::UnitKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doc = SBMLDocument::default();

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
        .unit(&ml)
        .build();

    // Create the ethanol species
    let ethanol = model
        .build_species("ethanol")
        .name("Ethanol")
        .compartment(&compartment)
        .initial_concentration(0.5)
        .unit(&mole)
        .has_only_substance_units(false)
        .build();

    // Create the aldehyde species
    let aldehyde = model
        .build_species("aldehyde")
        .name("Aldehyde")
        .compartment(&compartment)
        .initial_concentration(0.5)
        .unit(&mole)
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
    println!("{sbml_string}");

    // Save as a string to a file
    std::fs::write("./model.xml", &sbml_string).expect("Failed to write file");

    // Alternatively, save in a COMBINE archive
    let mut archive = CombineArchive::new();
    archive
        .add_entry(
            "./model.xml",
            KnownFormats::SBML,
            true,
            sbml_string.as_bytes(),
        )
        .expect("Failed to add entry to archive");
    archive
        .save("./model.omex")
        .expect("Failed to save archive");

    Ok(())
}
