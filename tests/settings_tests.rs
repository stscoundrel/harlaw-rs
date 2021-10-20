use harlaw::{get_default_settings, get_no_markup_settings};
use insta::assert_json_snapshot;

#[test]
fn default_settings_snapshot() {
    assert_json_snapshot!(get_default_settings())
}

#[test]
fn no_markup_settings_snapshot() {
    let settings = get_no_markup_settings();
    assert!(settings.replaces.is_empty());
    assert_json_snapshot!(settings)
}