use serde::{Deserialize, Serialize};
use crate::settings::HarlawSettings;
use crate::formatter;
use crate::reader;
use crate::writer;

/// Individual dictionary entry.
/// Each entry contains word, and at least one definition for the word.
///
#[derive(Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub word: String,
    pub definitions: Vec<String>
}

pub fn to_dictionary(filepath: &str, settings: HarlawSettings) -> Result<Vec<DictionaryEntry>, &'static str> {
    let lines = reader::read_dsl_file(filepath)?;

    Ok(formatter::format_entries(lines, settings))
}

pub fn to_json(input: &str, output: &str, settings: HarlawSettings) -> Result<(), &'static str> {
    let dictionary = to_dictionary(input, settings)?;

    writer::write_entries_to_json(output, &dictionary)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use crate::settings::get_default_settings;

    #[test]
    fn errors_if_not_readable_dictionary() {
        let filename = "./imaginary-dictionary.dsl";
        
        let result = to_dictionary(filename, get_default_settings());

        assert!(result.is_err());
    }

    #[test]
    fn gets_dictionary_from_dsl() {
        let filename = "./src/dictionary/fixtures/zoega-excerpt.dsl";
        
        let result = to_dictionary(filename, get_default_settings()).unwrap();
    
    
        assert_eq!(result[0].word, "a");
        assert_eq!(result[0].definitions[0], "a negative suffix to verbs, <i>not</i>;");
        assert_eq!(result[0].definitions[1], "era útmakligt, <i>at it is not unmeet that</i>.");

        assert_eq!(result[1].word, "abbadis");
        assert_eq!(result[1].definitions[0], "(pl. -ar), f. <i>abbess</i>.");
    }

    #[test]
    fn saves_dictionary_to_json() {
        let input = "./src/dictionary/fixtures/zoega-excerpt.dsl";
        let output = "./src/dictionary/fixtures/zoega-excerpt.json";
        let settings = get_default_settings();

        // Ensure JSON file does not already exist.
        let path_exists = Path::new(&output).exists();
        assert_eq!(path_exists, false);
        
        let result = to_json(input, output, settings);
        
        assert_eq!(result, Ok(()));

        // Assert file has now been created.
        let json_exists = Path::new(&output).exists();
        assert_eq!(json_exists, true);

        // Clean up created file.
        if json_exists {
            fs::remove_file(output).unwrap();
        }
    }
}