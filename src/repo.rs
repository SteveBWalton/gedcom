// Module to define the 'Repo' class.
// Rust does not have classes!

// Application modules.
use crate::tags;
use tags::Tags;


// Members variables for the 'Repo' class.
pub struct Repo {
    // The gedcom data that created this repo.
    pub gedcom: Vec<String>,
    // The gedcom tags attached to this repo.
    pub tags: Tags,
}



impl Repo {
    // Initialise a new 'Repo' from gedcom data.
    pub fn new(gedcom: &Vec<String>) -> Repo {
        Repo { gedcom: gedcom.to_vec(), tags: Tags::new(1, gedcom) }
    }

    // Debuging only.
    pub fn output_gedcom(&self) {
        for line in &self.gedcom {
            println!("\t{}", line);
        }
    }
}
