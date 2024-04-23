// Module to define the 'Individual' class.
// Rust does not have classes!

// Application modules.
use crate::tags;
use tags::Tags;
use crate::tag;
use tag::Tag;


// Members variables for the 'Individual' class.
pub struct Individual {
    // The gedcom data that created this individual.
    pub gedcom: Vec<String>,
    // The gedcom tags attached to this individual.
    pub tags: Tags,
    // The index for this individual.
    pub idx: String,
}



impl Individual {
    // Initialise a new individual from gedcom data.
    pub fn new(gedcom: &Vec<String>) -> Individual {
        // Make a new individual.
        Individual { gedcom: gedcom.to_vec(), tags: Tags::new(1, gedcom), idx: Tag::get_index(&gedcom[0]).to_string() }
    }

    // Do something better than this later.
    pub fn get_name(&self) -> Option<&str> {
        match self.tags.find_one("NAME") {
            Some(tag) => {
                return Some(&tag.value);
            }
            None => {
                return None;
            }
        }
    }


    // Debuging only.
    pub fn output_gedcom(&self) {
        for line in &self.gedcom {
            println!("\t{}", line);
        }
    }
}
