use serde::{Deserialize, Serialize};
use crate::settings::{HarlawSettings, get_default_settings, get_no_markup_settings};
use crate::formatter;
use crate::reader;

/// Individual dictionary entry.
/// Each entry contains word, and at least one definition for the word.
///
#[derive(Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub word: String,
    pub definitions: Vec<String>
}


fn to_dictionary(filepath: &str, settings: HarlawSettings) -> Result<Vec<DictionaryEntry>, &'static str> {
    let lines = reader::read_dsl_file(filepath)?;

    Ok(formatter::format_entries(lines, settings))
}

pub fn get_dictionary(filepath: &str) -> Result<Vec<DictionaryEntry>, &'static str> {
    let settings = get_default_settings();

    to_dictionary(filepath, settings)
}

pub fn get_dictionary_without_markup(filepath: &str) -> Result<Vec<DictionaryEntry>, &'static str> {
    let settings = get_no_markup_settings();

    to_dictionary(filepath, settings)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_if_not_readable_dictionary() {
        let filename = "./imaginary-dictionary.dsl";
        
        let result = get_dictionary(filename);

        assert!(result.is_err());
    }

    #[test]
    fn gets_default_dictionary_from_dsl() {
        let filename = "./src/dictionary/fixtures/zoega-excerpt.dsl";
        
        let result = get_dictionary(filename).unwrap();
    
    
        assert_eq!(result[0].word, "a");
        assert_eq!(result[0].definitions[0], "a negative suffix to verbs, <i>not</i>;");
        assert_eq!(result[0].definitions[1], "era útmakligt, <i>at it is not unmeet that</i>.");

        assert_eq!(result[1].word, "abbadis");
        assert_eq!(result[1].definitions[0], "(pl. -ar), f. <i>abbess</i>.");
    }

    #[test]
    fn gets_no_markup_dictionary_from_dsl() {
        let filename = "./src/dictionary/fixtures/zoega-excerpt.dsl";
        
        let result = get_dictionary_without_markup(filename).unwrap();
    
    
        assert_eq!(result[0].word, "a");
        assert_eq!(result[0].definitions[0], "a negative suffix to verbs, not;");
        assert_eq!(result[0].definitions[1], "era útmakligt, at it is not unmeet that.");

        assert_eq!(result[1].word, "abbadis");
        assert_eq!(result[1].definitions[0], "(pl. -ar), f. abbess.");
    }
}