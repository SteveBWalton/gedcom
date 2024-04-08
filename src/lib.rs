// Attempt create a library that will expose a family tree object library from a gedcom file.

// System modules.
use std::io::BufRead;
use std::collections::HashMap;

// Application modules.
mod individual;
mod family;
mod source;
mod object;
mod repo;
mod tags;
mod tag;

use individual::Individual;
use family::Family;
use source::Source;
use object::Object;
use repo::Repo;
use tags::Tags;


enum GedComObjects {
    Header,
    Individual,
    Family,
    Source,
    Object,
    Repo,
    Unknown,
}



// Member variables for the FamilyTree class.
pub struct FamilyTree {
    pub individuals: Vec<Individual>,
    pub families: Vec<Family>,
    pub sources: Vec<Source>,
    pub objects: Vec<Object>,
    pub repos: Vec<Repo>,
    pub tags: Tags,

    idx_individuals: HashMap<String, usize>
}



// Methods for the FamilyTree class.
impl FamilyTree {
    // Create a new 'FamilyTree' structure.
    pub fn new() -> FamilyTree {
        let individuals = Vec::new();
        let families = Vec::new();

        FamilyTree{individuals: individuals, families: families, sources: Vec::new(), objects: Vec::new(), repos: Vec::new(), tags: Tags::new_empty(), idx_individuals: HashMap::new()}
    }



    // Add an individual to this gedcom from the specified gedcom files lines.
    pub fn add_individual(&mut self, gedcom: &Vec<String>) {
        let individual = Individual::new(gedcom);
        // println!("add_individual()");
        // individual.output_gedcom();
        let idx: String = individual.idx.to_string();
        self.individuals.push(individual);
        self.idx_individuals.entry(idx).or_insert(self.individuals.len() - 1);
    }

    // Return the individual with the specified index.
    pub fn get_individual(&self, idx: &str) -> &Individual {
        match self.idx_individuals.get(idx) {
           Some(i) => return &self.individuals[*i],
           None => {
               println!("get_individual did not work.");
               return &self.individuals[0];
           }
        }
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



    // Add a media object to this gedcom from the specified gedcom file lines.
    pub fn add_object(&mut self, gedcom: &Vec<String>) {
        let object = Object::new(gedcom);
        self.objects.push(object);
    }



    // Add a repo to this gedcom from the specified gedcom file lines.
    pub fn add_repo(&mut self, gedcom: &Vec<String>) {
        let repo = Repo::new(gedcom);
        self.repos.push(repo);
    }



    // Add the header information for this gedcom.
    fn add_header(&mut self, gedcom: &Vec<String>) {
        self.tags = Tags::new(1, gedcom);
    }


    // Report the unknown gedcom.
    fn report_unknown(&self, gedcom: &Vec<String>) {
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

                                        GedComObjects::Repo => {
                                            self.add_repo(&object_lines);
                                        }

                                        GedComObjects::Header => {
                                            self.add_header(&object_lines);
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
                                } else if line.ends_with("REPO") {
                                    object_type = GedComObjects::Repo;
                                } else if line.ends_with("HEAD") {
                                    object_type = GedComObjects::Header;
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
        println!("There are {} repos.", self.repos.len());
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
