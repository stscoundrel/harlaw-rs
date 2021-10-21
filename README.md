# Harlaw

Transform DSL (Lingvo Dictionary File) files to JSON. Formatting options available for custom output.

There are many dictionaries available as .dsl, but very few in easily consumable formats. Harlaw formats the dsl files to json with decent search/replace/remove options.

Rust port of original [Node.js library](https://github.com/stscoundrel/harlaw).

### Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
harlaw = "1.0.0"
```

### Usage

Harlaw can transform DSL dictionaries either to JSON files, or to in-memory structures.

#### Reading DSL dictionary into memory.

Dictionaries can be read with two default transform: markup, or no markup.

```rust
// Default methods come with two variations: with or without markup.
use harlaw::{get_dictionary, get_dictionary_without_markup, DictionaryEntry};

// Path to DSL file to transform.
let my_dictionary = "./my-dictionary.dsl";

// Standard method transforms [i], [b] etc tags to their html equilevants <i>, <strong>
let result = get_dictionary(my_dictionary);

// No-markup version. Removes all additional markup.
let no_markup_result = get_no_markup_dictionary();

// Both methods return Result, which either contains Vec<DictionaryEntry> or error message.
let dictionary_content: Vec<DictionaryEntry> = result.unwrap();
let no_markup_dictionary_content: Vec<DictionaryEntry> = no_markup_result.unwrap();

// Do what you want with dictionary data.
```

#### Creating JSON file from DSL file.

JSON files can be created with two default settings: markup, or no markup.

```rust
// Default methods comes with two variations: with or without markup.
use harlaw::{to_json, to_json_no_markup};

// Paths to original DSL library & JSON file to be created.
let input = "./my-dictionary.dsl";
let ouput = "./my-dictionary.json";

// Standard method with default markup transforms.
let result = to_json(input, output);

// No-markup version. Removes all additional markup.
let result_no_markup = to_json_no_markup(input, output);

// Both methods return Result -> Either Ok or Err message.
if result.is_ok() {
    // JSON was created in output location.
}
```

#### Custom transform settings.

If you have custom formatting needs, you can also create custom settings object for transforms. It allows user to set custom search/replaces and removes.

Custom settings can be used with both JSON & in-memory methods.

```rust
// Custom settings methods available for both getters.
use harlaw::{get_dictionary_with_custom_settings, to_json_with_custom_settings};

// Structs used to build custom settings.
use harlaw::{ContentReplace, HarlawSettings};

// Paths to original DSL library & JSON file to be created.
let input = "./my-dictionary.dsl";
let ouput = "./my-dictionary.json";

// Settings accept vectors of removes, and search/replace structs.
let settings = HarlawSettings {
    removes: vec!["[m1]", "[m2]", "[/m]", "\t", "my-custom-remove"],
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

// In memory.
let result = get_dictionary_with_custom_settings(input, settings);

// To json.
let json_result = to_json_with_custom_settings(input, output, settings);

// Method returns match their non-custom counterparts.
```

You can also use default settings as a base for your own settings.

```rust
use harlaw::{get_default_settings, get_no_markup_settings, ContentReplace};

// Use default settings, or no-markup settings as a base.
let mut settings = get_default_settings();

// Append some custom search replace.
settings.replaces.push(
    ContentReplace {
        search: "foo",
        replace: "bar",
    }
);

// Append some custom remove
settings.removes.push("baz");

// Use your own settings in one of the custom methods.
```


#### What's in the name?

In G.R.R Martins "A Song Of Ice And Fire", there is a character named Rodrik Harlaw. He is mockingly called "The Reader". That is what my Harlaw does too; reads things no one else cares about.
