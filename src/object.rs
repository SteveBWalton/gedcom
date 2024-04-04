// Module to define the 'Object' class.
// Rust does not have classes!

// Members variables for the 'Family' class.
pub struct Object {
    // The gedcom data that created this individual.
    pub gedcom: Vec<String>,
}



impl Object {
    // Initialise a new individual from gedcom data.
    pub fn new(gedcom: &Vec<String>) -> Object {
        let mut gedcom_copy: Vec<String> = gedcom.to_vec();

        Object { gedcom: gedcom_copy }
    }

    // Debuging only.
    pub fn output_gedcom(&self) {
        for line in &self.gedcom {
            println!("\t{}", line);
        }
    }
}
