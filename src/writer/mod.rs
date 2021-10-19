use std::fs;
use crate::DictionaryEntry;

fn json_stringify(entries: &[DictionaryEntry]) -> Result<String, &'static str> {
    match serde_json::to_string(&entries) {
        Ok(json) => Ok(json),
        Err(_e) => Err("Could not stringify entries"),
    }
}

pub fn write_entries_to_json(path: &str, entries: &[DictionaryEntry]) -> Result<(), &'static str> {
    let json =  json_stringify(entries).unwrap_or_default();

    match fs::write(path, json) {
        Ok(_res) => Ok(()),
        Err(_e) => Err("Could not write JSON file")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn stringifies_entries() {
        let entry = DictionaryEntry {
            word: String::from("Foo"),
            definitions: vec![String::from("Bar baz")]
        };
        let entries = vec![entry];

        let result = json_stringify(&entries).unwrap();
        let expected = "[{\"word\":\"Foo\",\"definitions\":[\"Bar baz\"]}]";

        assert_eq!(result, expected);
    }

    #[test]
    fn writes_entries_to_json_file() {
        let entry = DictionaryEntry {
            word: String::from("Foo"),
            definitions: vec![String::from("Bar baz")]
        };
        let entries = vec![entry];
        let path = "./src/writer/test.json";

        // Ensure JSON file does not already exist.
        let path_exists = Path::new(&path).exists();
        assert_eq!(path_exists, false);

        write_entries_to_json(&path, &entries).unwrap();
        
        // Assert file has now been created.
        let json_exists = Path::new(&path).exists();
        assert_eq!(json_exists, true);

        // Clean up created file.
        if json_exists {
            fs::remove_file(path).unwrap();
        }
    }
}