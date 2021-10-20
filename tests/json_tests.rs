use std::path::Path;
use std::fs;
use harlaw::{to_json, to_json_no_markup, to_json_with_custom_settings};
use harlaw::{HarlawSettings, ContentReplace};

#[test]
fn saves_default_dictionary_to_json() {
    let input = "./tests/fixtures/default_dictionary.dsl";
    let output = "./tests/fixtures/default_dictionary_1.json";

    // Ensure JSON file does not already exist.
    let path_exists = Path::new(&output).exists();
    assert_eq!(path_exists, false);

    let result = to_json(input, output);

    assert!(!result.is_err());

    // Assert file has now been created.
    let json_exists = Path::new(&output).exists();
    assert_eq!(json_exists, true);

    // Clean up created file.
    if json_exists {
        fs::remove_file(output).unwrap();
    }
}

#[test]
fn saves_no_markup_dictionary_to_json() {
    let input = "./tests/fixtures/default_dictionary.dsl";
    let output = "./tests/fixtures/default_dictionary_2.json";

    // Ensure JSON file does not already exist.
    let path_exists = Path::new(&output).exists();
    assert_eq!(path_exists, false);

    let result = to_json_no_markup(input, output);

    assert!(!result.is_err());

    // Assert file has now been created.
    let json_exists = Path::new(&output).exists();
    assert_eq!(json_exists, true);

    // Clean up created file.
    if json_exists {
        fs::remove_file(output).unwrap();
    }
}

#[test]
fn saves_custom_settings_dictionary_to_json() {
    let input = "./tests/fixtures/default_dictionary.dsl";
    let output = "./tests/fixtures/default_dictionary_3.json";
    let settings = HarlawSettings {
        removes: vec!["[m1]", "[m2]", "[/m]", "\t"],
        replaces: vec![
            ContentReplace {
                search: "[b]",
                replace: "<TUHTI>",
            },
            ContentReplace {
                search: "[/b]",
                replace: "</TUHTI>",
            },
            ContentReplace {
                search: "[i]",
                replace: "<VINO>",
            },
            ContentReplace {
                search: "[/i]",
                replace: "</VINO>",
            }
        ],
    };

    // Ensure JSON file does not already exist.
    let path_exists = Path::new(&output).exists();
    assert_eq!(path_exists, false);

    let result = to_json_with_custom_settings(input, output, settings);

    assert!(!result.is_err());

    // Assert file has now been created.
    let json_exists = Path::new(&output).exists();
    assert_eq!(json_exists, true);

    // Clean up created file.
    if json_exists {
        fs::remove_file(output).unwrap();
    }
}