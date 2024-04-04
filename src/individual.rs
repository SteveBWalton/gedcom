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

    pub first_name: String,
    pub last_name: String,
}



impl Individual {
    // Initialise a new individual from gedcom data.
    pub fn new(gedcom: &Vec<String>) -> Individual {
        // Make a copy of the gedcom line for this individual.
        let mut gedcom_copy: Vec<String> = gedcom.to_vec();

        // Get the index from the first line.
        let first_line = gedcom_copy.remove(0);

        // Make the gedcom tags for this individual.
        let mut tags = Tags::new(1, gedcom);

        // Make a new individual.
        Individual { gedcom: gedcom.to_vec(), tags: tags, first_name: "Steve".to_string(), last_name: "Walton".to_string() }
    }

    // Debuging only.
    pub fn output_gedcom(&self) {
        for line in &self.gedcom {
            println!("\t{}", line);
        }
    }
}
