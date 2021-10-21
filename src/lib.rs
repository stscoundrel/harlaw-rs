mod dictionary;
mod settings;
mod writer;
mod reader;
mod formatter;

pub use dictionary::DictionaryEntry;
pub use settings::{HarlawSettings, ContentReplace, get_default_settings, get_no_markup_settings};


/// Transform DSL dictionary into vector of DictionaryEntries with default markup.
/// Default Lingvo tags are transformed to their HTML equilevants.
/// 
/// # Examples
/// 
/// ```
/// use harlaw::{get_dictionary, DictionaryEntry};
/// 
/// let my_dictionary = "./my-dictionary.dsl";
/// 
/// // Result either contains Vec<DictionaryEntry> or error message
/// let result = get_dictionary(my_dictionary);
/// 
/// ```
pub fn get_dictionary(filepath: &str) -> Result<Vec<DictionaryEntry>, &'static str> {
    let settings = get_default_settings();

    dictionary::to_dictionary(filepath, settings)
}

/// Transform DSL dictionary into vector of DictionaryEntries with not markup.
/// This version removes all formatting tags.
/// 
/// # Examples
/// 
/// ```
/// use harlaw::{get_dictionary_without_markup, DictionaryEntry};
/// 
/// let my_dictionary = "./my-dictionary.dsl";
/// 
/// // Result either contains Vec<DictionaryEntry> or error message
/// let result = get_dictionary_without_markup(my_dictionary);
/// 
/// ```
pub fn get_dictionary_without_markup(filepath: &str) -> Result<Vec<DictionaryEntry>, &'static str> {
    let settings = get_no_markup_settings();

    dictionary::to_dictionary(filepath, settings)
}

/// Transform DSL dictionary into vector of DictionaryEntries with custom formatting settings.
/// 
/// # Examples
/// 
/// ```
/// use harlaw::{get_dictionary_with_custom_settings, ContentReplace, HarlawSettings, DictionaryEntry};
/// 
/// let my_dictionary = "./my-dictionary.dsl";
/// 
/// let settings = HarlawSettings {
///     removes: vec!["[m1]", "[m2]", "[/m]", "\t"],
///     replaces: vec![
///         ContentReplace {
///             search: "[b]",
///             replace: "<thick>",
///         },
///         ContentReplace {
///             search: "[/b]",
///             replace: "</thick>",
///         },
///         ContentReplace {
///             search: "[i]",
///             replace: "<skew>",
///         },
///         ContentReplace {
///             search: "[/i]",
///             replace: "</skew>",
///         }
///     ],
/// };
/// 
/// // Result either contains Vec<DictionaryEntry> or error message
/// let result = get_dictionary_with_custom_settings(my_dictionary, settings);
/// 
/// ```
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