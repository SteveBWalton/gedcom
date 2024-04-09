// Module to define the 'Source' class.
// Rust does not have classes!

// Application modules.
use crate::tags;
use tags::Tags;
use crate::tag;
use tag::Tag;



// Members variables for the 'Source' class.
pub struct Source {
    // The gedcom data that created this source.
    pub gedcom: Vec<String>,
    // The gedcom tags attached to this Source.
    pub tags: Tags,
    // The index for this source.
    pub idx: String,
}



impl Source {
    // Initialise a new source from gedcom data.
    pub fn new(gedcom: &Vec<String>) -> Source {
        Source { gedcom: gedcom.to_vec(), tags: Tags::new(1, gedcom), idx: Tag::get_index(&gedcom[0]).to_string() }
    }

    // Debuging only.
    pub fn output_gedcom(&self) {
        for line in &self.gedcom {
            println!("\t{}", line);
        }
    }
}
