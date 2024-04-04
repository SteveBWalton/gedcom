// Module to define the 'Source' class.
// Rust does not have classes!

// Members variables for the 'Family' class.
pub struct Source {
    // The gedcom data that created this individual.
    pub gedcom: Vec<String>,
}



impl Source {
    // Initialise a new individual from gedcom data.
    pub fn new(gedcom: &Vec<String>) -> Source {
        let mut gedcom_copy: Vec<String> = gedcom.to_vec();

        Source { gedcom: gedcom_copy }
    }

    // Debuging only.
    pub fn output_gedcom(&self) {
        for line in &self.gedcom {
            println!("\t{}", line);
        }
    }
}
