// Module to define the 'family' class.
// Rust does not have classes!

// Application modules.
use crate::tags;
use tags::Tags;
use crate::tag;
use tag::Tag;



// Members variables for the 'Family' class.
pub struct Family {
    // The gedcom data that created this family.
    pub gedcom: Vec<String>,
    // The gedcom tags attached to this family.
    pub tags: Tags,
    // The index for this family.
    pub idx: String,
}



impl Family {
    // Initialise a new individual from gedcom data.
    pub fn new(gedcom: &Vec<String>) -> Family {
        Family { gedcom: gedcom.to_vec(), tags: Tags::new(1, gedcom), idx: Tag::get_index(&gedcom[0]).to_string() }
    }

    // Debuging only.
    pub fn output_gedcom(&self) {
        for line in &self.gedcom {
            println!("\t{}", line);
        }
    }
}
