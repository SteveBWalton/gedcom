// Attempt create a library that will expose a family tree object library from a gedcom file.
use std::io::BufRead;

// Application modules.
mod person;

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

    // Open a gedcom file and create objects from it.
    pub fn open(&mut self, file_name: &str) {
        match std::fs::File::open(file_name) {
            Ok(file) => {
                let reader = std::io::BufReader::new(file);

                for line_or_error in reader.lines() {
                    match line_or_error {
                        Ok(line) => {
                            if line.starts_with('0') {
                                println!("{}", line);
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
