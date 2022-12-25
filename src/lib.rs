// Attempt create a library that will expose a family tree object library from a gedcom file.

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
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = FamilyTree::new();
        result.open("test_file.gedcom");
        assert_eq!(result.people.len(), 1);
    }
}
