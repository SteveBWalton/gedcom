// Attempt create a library that will expose a family tree object library from a gedcom file.
use std::io::BufRead;

// Application modules.
mod person;

enum GedComObjects {
    Individual,
    Family,
    Media,
    Unknown,
}

// Member variables for the FamilyTree class.
pub struct FamilyTree {
    pub people: Vec<person::Person>
}

// Methods for the FamilyTree class.
impl FamilyTree {
    // Create a new 'FamilyTree' structure.
    pub fn new() -> FamilyTree {
        let mut people = Vec::new();

        let my_person = person::Person{first_name: "Steve".to_string(), last_name: "Walton".to_string()};
        people.push(my_person);

        FamilyTree{people: people}
    }



    pub fn add_individual(&mut self, gedcom: &Vec<String>) {
        println!("add_individual()");
        for line in gedcom {
            println!("\t{}", line);
        }
    }



    // Open a gedcom file and create objects from it.
    pub fn open(&mut self, file_name: &str) {
        match std::fs::File::open(file_name) {
            Ok(file) => {
                let reader = std::io::BufReader::new(file);
                let mut object_lines: Vec<String> = Vec::new();
                let mut object_type: GedComObjects = GedComObjects::Unknown;

                for line_or_error in reader.lines() {
                    match line_or_error {
                        Ok(line) => {
                            if line.starts_with('0') {
                                // Deal with the exsting object.
                                if object_lines.len() != 0 {
                                    match object_type {
                                        GedComObjects::Individual => {
                                            println!("\tIndividual Object.");
                                            self.add_individual(&object_lines);
                                        }
                                        _ => {
                                            println!("\tUnknown Object.");
                                        }
                                    }
                                }

                                // Start a new object.
                                object_lines = Vec::new();
                                if line.ends_with("INDI") {
                                    object_type = GedComObjects::Individual;
                                } else if line.ends_with("FAM") {
                                    object_type = GedComObjects::Family;
                                } else {
                                    object_type = GedComObjects::Unknown;
                                }
                                object_lines.push(line);
                            }
                            else
                            {
                                object_lines.push(line);
                            }
                        }

                        Err(error) => {
                            println!("Error with line.");
                            println!("{}", error);
                        }
                    }
                }
            }

            Err(error) => {
                println!("Error opening {}", file_name);
                println!("{}", error);
            }

        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = FamilyTree::new();
        result.open("/home/steve/Documents/Personal/Family Tree/walton.ged");
        assert_eq!(result.people.len(), 1);
    }
}
