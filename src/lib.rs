pub mod dictionary;
pub mod settings;
pub mod writer;
mod reader;
mod formatter;

pub use dictionary::DictionaryEntry;
pub use settings::{get_default_settings, get_no_markup_settings};

pub fn get_dictionary(filepath: &str) -> Result<Vec<DictionaryEntry>, &'static str> {
    let settings = get_default_settings();

    dictionary::to_dictionary(filepath, settings)
}

pub fn get_dictionary_without_markup(filepath: &str) -> Result<Vec<DictionaryEntry>, &'static str> {
    let settings = get_no_markup_settings();

    dictionary::to_dictionary(filepath, settings)
}

pub fn to_json(input: &str, output: &str) -> Result<(), &'static str> {
    let settings = get_default_settings();

    dictionary::to_json(input, output, settings)
}

pub fn to_json_no_markup(input: &str, output: &str) -> Result<(), &'static str> {
    let settings = get_no_markup_settings();

    dictionary::to_json(input, output, settings)
}