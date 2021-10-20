use harlaw::{get_dictionary, get_dictionary_without_markup, get_dictionary_with_custom_settings};
use harlaw::{HarlawSettings, ContentReplace};
use insta::assert_json_snapshot;

#[test]
fn gets_default_dictionary() {
    let input = "./tests/fixtures/default_dictionary.dsl";
    let result = get_dictionary(input).unwrap();

    assert_json_snapshot!(result)
}

#[test]
fn gets_dictionary_without_markup() {
    let input = "./tests/fixtures/default_dictionary.dsl";
    let result = get_dictionary_without_markup(input).unwrap();

    assert_json_snapshot!(result)
}

#[test]
fn gets_dictionary_with_custom_settings() {
    let input = "./tests/fixtures/default_dictionary.dsl";
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
    let result = get_dictionary_with_custom_settings(input, settings).unwrap();

    assert_json_snapshot!(result)
}