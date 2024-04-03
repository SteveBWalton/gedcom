// Module to define the 'Individual' class.
// Rust does not have classes!

// Members variables for the 'Individual' class.
pub struct Individual {
    // The gedcom data that created this individual.
    pub gedcom: Vec<String>,
    pub first_name: String,
    pub last_name: String,
}



impl Individual {
    // Initialise a new individual from gedcom data.
    pub fn new(gedcom: &Vec<String>) -> Individual {
        let mut gedcom_copy: Vec<String> = gedcom.to_vec();

        Individual { gedcom: gedcom_copy, first_name: "Steve".to_string(), last_name: "Walton".to_string() }
    }

    // Debuging only.
    pub fn output_gedcom(&self) {
        for line in &self.gedcom {
            println!("\t{}", line);
        }
    }
}
