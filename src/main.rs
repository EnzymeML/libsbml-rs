use libsbml::SBMLDocument;

pub fn main() {
    let mut document = SBMLDocument::new(3, 2);
    let model = document.create_model("test");

    // Species
    let glucose = model.create_species("glucose");
    let ethanol = model.create_species("ethanol");

    // Compartment
    let cytosol = model.create_compartment("cytosol");

    println!("{}", model.id());
    println!("{}", glucose.id());
    println!("{}", ethanol.id());
    println!("{}", cytosol.id());
}
