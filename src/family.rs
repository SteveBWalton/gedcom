// Module to define the 'family' class.
// Rust does not have classes!

// Members variables for the 'Family' class.
pub struct Family {
    // The gedcom data that created this individual.
    pub gedcom: Vec<String>,
}



impl Family {
    // Initialise a new individual from gedcom data.
    pub fn new(gedcom: &Vec<String>) -> Family {
        let mut gedcom_copy: Vec<String> = gedcom.to_vec();

        Family { gedcom: gedcom_copy }
    }

    // Debuging only.
    pub fn output_gedcom(&self) {
        for line in &self.gedcom {
            println!("\t{}", line);
        }
    }
}
