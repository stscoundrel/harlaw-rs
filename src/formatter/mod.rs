use crate::DictionaryEntry;
use crate::settings::{HarlawSettings, TAB, SKIPS};

fn format_line(line: &str, settings: &HarlawSettings) -> String {
    let mut formatted_line = String::from(line);

    for remove in &settings.removes {
       formatted_line =  formatted_line.replace(remove, "");
    }

    for pattern in &settings.replaces {
        formatted_line =  formatted_line.replace(pattern.search, pattern.replace);
     }

    formatted_line  
}

pub fn format_entries(lines: Vec<String>, settings: HarlawSettings) -> Vec<DictionaryEntry> {
    let mut dictionary_entries: Vec<DictionaryEntry> = vec![];
    let mut index = 0;

    for (line_index, line) in lines.iter().enumerate() {
        let first_character = line.chars().next().unwrap().to_string();

        // Skip metadata lines.
        if SKIPS.contains(&&first_character[..]) {
            continue;
        }

        // If line startes with tab, it is definition of previous entry.
        if first_character.eq(TAB) {
            let formatted_line = format_line(line, &settings);
            dictionary_entries[index -1].definitions.push(formatted_line);
            continue;
        }

        /*
         * DSL files may contain many headwords in a row.
         * This means the headwords use identical definition, which comes after them.
         * Loop deeper to get the definition, if it is not present immediartely.
         */
        if !dictionary_entries.is_empty() && dictionary_entries[index - 1].definitions.is_empty() {
            let mut not_found = true;
            let mut lower_index = line_index;

            while not_found {
                lower_index += 1;

                let start_character = lines[lower_index].chars().next().unwrap().to_string();
                if start_character.eq(TAB) {
                    not_found = false;
                    let formatted_line = format_line(&lines[lower_index], &settings);
                    dictionary_entries[index - 1].definitions.push(formatted_line);
                }
            }
        }

        // The line is a headword, form a new entry. 
        dictionary_entries.push(DictionaryEntry {
            word: format_line(line, &settings),
            definitions: vec![],
        });
        index += 1;
    }


    dictionary_entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::get_default_settings;

    #[test]
    fn formats_line() {
        let line = "	[m1]Lorem ipsum [b]dolor[/b] sit amet, dolor sit igitur[/m]";        
        
        let result = format_line(line, &get_default_settings());

        assert_eq!(result, String::from("Lorem ipsum <strong>dolor</strong> sit amet, dolor sit igitur"));
    }

    #[test]
    fn formats_simple_lines_to_entries() {
        let lines = vec![
            String::from("#NAME	\"Test Dictionary fixture\""),
            String::from("foo"),
            String::from("	[m1]Lorem ipsum dolor sit amet, dolor sit igitur[/m]")
        ];
        
        let result = format_entries(lines, get_default_settings());

        assert_eq!(result[0].word, "foo");
        assert_eq!(result[0].definitions[0], "Lorem ipsum dolor sit amet, dolor sit igitur");
    }

    #[test]
    fn formats_grouped_definitions_to_entries() {
        let lines = vec![
            String::from("#NAME	\"Test Dictionary fixture\""),
            String::from("foo"),
            String::from("	[m1]Lorem ipsum dolor sit amet, dolor sit igitur[/m]"),
            String::from("bar"),
            String::from("bar-like-word-with-same-defs"),
            String::from("	[m1][b]Dolor[/b] sit igitur.[/m]"),
            String::from("baz"),
            String::from("	[m1]Lorem ipsum dolor sit amet, consectetur adipiscing elit[/m]"),
            String::from("	[m2]Lorem ipsum dolor sit amet.[/m]"),
        ];
        
        let result = format_entries(lines, get_default_settings());

        assert_eq!(result[0].word, "foo");
        assert_eq!(result[0].definitions[0], "Lorem ipsum dolor sit amet, dolor sit igitur");

        assert_eq!(result[1].word, "bar");
        assert_eq!(result[1].definitions[0], "<strong>Dolor</strong> sit igitur.");

        assert_eq!(result[2].word, "bar-like-word-with-same-defs");
        assert_eq!(result[2].definitions[0], "<strong>Dolor</strong> sit igitur.");


        assert_eq!(result[3].word, "baz");
        assert_eq!(result[3].definitions[0], "Lorem ipsum dolor sit amet, consectetur adipiscing elit");
        assert_eq!(result[3].definitions[1], "Lorem ipsum dolor sit amet.");
    }
}