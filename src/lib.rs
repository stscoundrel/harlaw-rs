mod dictionary;
mod settings;
mod writer;
mod reader;
mod formatter;

pub use dictionary::DictionaryEntry;
pub use settings::{HarlawSettings, ContentReplace, get_default_settings, get_no_markup_settings};

pub fn get_dictionary(filepath: &str) -> Result<Vec<DictionaryEntry>, &'static str> {
    let settings = get_default_settings();

    dictionary::to_dictionary(filepath, settings)
}

pub fn get_dictionary_without_markup(filepath: &str) -> Result<Vec<DictionaryEntry>, &'static str> {
    let settings = get_no_markup_settings();

    dictionary::to_dictionary(filepath, settings)
}

pub fn get_dictionary_with_custom_settings(filename: &str, settings: HarlawSettings) -> Result<Vec<DictionaryEntry>, &'static str> {
    dictionary::to_dictionary(filename, settings)
}

pub fn to_json(input: &str, output: &str) -> Result<(), &'static str> {
    let settings = get_default_settings();

    dictionary::to_json(input, output, settings)
}

pub fn to_json_no_markup(input: &str, output: &str) -> Result<(), &'static str> {
    let settings = get_no_markup_settings();

    dictionary::to_json(input, output, settings)
}

pub fn to_json_with_custom_settings(input: &str, output: &str, settings: HarlawSettings) -> Result<(), &'static str> {
    dictionary::to_json(input, output, settings)
}