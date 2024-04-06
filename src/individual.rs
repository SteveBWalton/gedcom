// Module to define the 'Individual' class.
// Rust does not have classes!

// Application modules.
use crate::tags;
use tags::Tags;


// Members variables for the 'Individual' class.
pub struct Individual {
    // The gedcom data that created this individual.
    pub gedcom: Vec<String>,
    // The gedcom tags attached to this individual.
    pub tags: Tags,
}



impl Individual {
    // Initialise a new individual from gedcom data.
    pub fn new(gedcom: &Vec<String>) -> Individual {
        // Get the index from the first line.
        // This is expected to be in the form 0 @I0001@ INDI
        let _first_line = &gedcom[0];

        // Make a new individual.
        Individual { gedcom: gedcom.to_vec(), tags: Tags::new(1, gedcom) }
    }

    // Debuging only.
    pub fn output_gedcom(&self) {
        for line in &self.gedcom {
            println!("\t{}", line);
        }
    }
}
