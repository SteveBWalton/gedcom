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

    // Returns this tag in lines for a gedcom file.
    pub fn to_gedcom_file(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        lines.push(format!("{} {} {}", self.level, self.key, self.value));
        for child_line in self.tags.to_gedcom_file() {
            lines.push(child_line);
        }
        return lines;
    }

    // Returns this tag in lines for decorated html.
    pub fn to_decorated_html(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        let mut space = "".to_string();
        for _i in 0..self.level-1 {
            space += "  ";
        }
        lines.push(format!("{}<b>{}</b> {}", space, self.key, self.value));
        for child_line in self.tags.to_decorated_html() {
            lines.push(child_line);
        }
        return lines;
    }
}

