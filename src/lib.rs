// Attempt create a library that will expose a family tree object library from a gedcom file.

// System modules.
use std::io::BufRead;

// Application modules.
mod individual;
use individual::Individual;
mod family;
use family::Family;
mod source;
use source::Source;
mod object;
use object::Object;



enum GedComObjects {
    Individual,
    Family,
    Source,
    Object,
    Unknown,
}



// Member variables for the FamilyTree class.
pub struct FamilyTree {
    pub individuals: Vec<Individual>,
    pub families: Vec<Family>,
    pub sources:Vec<Source>,
    pub objects:Vec<Object>
}



// Methods for the FamilyTree class.
impl FamilyTree {
    // Create a new 'FamilyTree' structure.
    pub fn new() -> FamilyTree {
        let mut individuals = Vec::new();
        let mut families = Vec::new();
        let mut sources = Vec::new();
        let mut objects = Vec::new();

        FamilyTree{individuals: individuals, families: families, sources: sources, objects: objects}
    }



    // Add an individual to this gedcom from the specified gedcom files lines.
    pub fn add_individual(&mut self, gedcom: &Vec<String>) {
        let individual = Individual::new(gedcom);
        // println!("add_individual()");
        // individual.output_gedcom();
        self.individuals.push(individual);
    }



    // Add a family to this gedcom from the specified gedcom file lines.
    pub fn add_family(&mut self, gedcom: &Vec<String>) {
        let family = Family::new(gedcom);
        self.families.push(family);
    }



    // Add a source to this gedcom from the specified gedcom file lines.
    pub fn add_source(&mut self, gedcom: &Vec<String>) {
        let source = Source::new(gedcom);
        self.sources.push(source);
    }



    // Add a media to this gedcom from the specified gedcom file lines.
    pub fn add_object(&mut self, gedcom: &Vec<String>) {
        let object = Object::new(gedcom);
        self.objects.push(object);
    }



    // Report the unknown gedcom.
    pub fn report_unknown(&self, gedcom: &Vec<String>) {
        println!("Unknown.");
        for line in gedcom {
            println!("\t{}", line);
        }
    }


    // Open a gedcom file and create objects from it.
    pub fn open(&mut self, file_name: &str) {
        println!("open('{}')", file_name);
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
                                            self.add_individual(&object_lines);
                                        }

                                        GedComObjects::Family => {
                                            self.add_family(&object_lines);
                                        }

                                        GedComObjects::Source => {
                                            self.add_source(&object_lines);
                                        }

                                        GedComObjects::Object => {
                                            self.add_object(&object_lines);
                                        }

                                        _ => {
                                            self.report_unknown(&object_lines);
                                        }
                                    }
                                }

                                // Start a new object.
                                object_lines = Vec::new();
                                if line.ends_with("INDI") {
                                    object_type = GedComObjects::Individual;
                                } else if line.ends_with("FAM") {
                                    object_type = GedComObjects::Family;
                                } else if line.ends_with("SOUR") {
                                    object_type = GedComObjects::Source;
                                } else if line.ends_with("OBJE") {
                                    object_type = GedComObjects::Object;
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
                println!("Error opening '{}'", file_name);
                println!("{}", error);
            }

        }
        println!("There are {} individuals.", self.individuals.len());
        println!("There are {} families.", self.families.len());
        println!("There are {} sources.", self.sources.len());
        println!("There are {} objects.", self.objects.len());
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
