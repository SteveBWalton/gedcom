// Module to define the 'Object' class.
// Rust does not have classes!

// Application modules.
use crate::tags;
use tags::Tags;


// Members variables for the media object class.
pub struct Object {
    // The gedcom data that created this media object.
    pub gedcom: Vec<String>,
    // The gedcom tags attached to this media object.
    pub tags: Tags,
}



impl Object {
    // Initialise a new media object from gedcom data.
    pub fn new(gedcom: &Vec<String>) -> Object {
        Object { gedcom: gedcom.to_vec(), tags: Tags::new(1, gedcom) }
    }

    // Debuging only.
    pub fn output_gedcom(&self) {
        for line in &self.gedcom {
            println!("\t{}", line);
        }
    }
}
