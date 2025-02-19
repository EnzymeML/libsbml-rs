use libsbml::prelude::*;
use serde::{Deserialize, Serialize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doc = SBMLDocument::new(3, 2);

    // Create a model
    let model = doc.create_model("example");

    // Create a compartment
    let compartment = model.build_compartment("cytosol").name("cytosol").build();

    // Create an annotation for the glucose species
    let glucose_annotation = MyAnnotation {
        xmlns: "http://my.namespace.com".to_string(),
        key: "test".to_string(),
        value: 1,
    };

    // Create the glucose species with the annotation
    let glucose = model
        .build_species("glucose")
        .name("Glucose")
        .compartment(&compartment.id())
        .initial_amount(10.0)
        .boundary_condition(true)
        .annotation_serde(&glucose_annotation)?
        .build();

    // Serialize the document to an SBML string
    let sbml_string = doc.to_xml_string();

    // Print the SBML string
    println!("{}", sbml_string);

    // This is how you could also extract annotations from the model
    let glucose_annot: MyAnnotation = glucose.get_annotation_serde()?;
    println!("Glucose annotation: {:#?}", glucose_annot);

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct MyAnnotation {
    #[serde(rename = "@xmlns")]
    xmlns: String,
    key: String,
    value: i32,
}
