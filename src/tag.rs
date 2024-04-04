// Modulus to define the 'Tag' class.
// Rust does not have classes!

// Application modules.
use crate::tags;
use tags::Tags;

// Member variables for the 'Tag' class.
pub struct Tag {
    // The level of this tag.
    pub level: usize,

    // The key of this tag.
    pub key: String,

    // The value of this tag.
    pub value: String,

    // The child tags of this tag.
    pub tags: Tags
}


// The methods for the 'Tag' struct.
impl Tag {
    pub fn new(level: usize, key: String, value: String, gedcom: &Vec<String>) -> Tag {
        // Tag {level: level, key: key, value: value, tags: Tags::new(Vec<String>::new())}
        Tag {level: level, key: key, value: value, tags: Tags::new(level+1, gedcom)}
    }
}

