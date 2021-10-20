use crate::DictionaryEntry;

const TAB: &str = "\t";
const SKIPS: &[&str]= &["#"];

pub fn format_entries(lines: Vec<String>) -> Vec<DictionaryEntry> {
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
            dictionary_entries[index -1].definitions.push(String::from(line));
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
                    dictionary_entries[index - 1].definitions.push(String::from(&lines[lower_index]));
                }
            }
        }

        // The line is a headword, form a new entry. 
        dictionary_entries.push(DictionaryEntry {
            word: String::from(line),
            definitions: vec![],
        });
        index += 1;
    }


    dictionary_entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_simple_lines_to_entries() {
        let lines = vec![
            String::from("#NAME	\"Test Dictionary fixture\""),
            String::from("foo"),
            String::from("	[m1]Lorem ipsum dolor sit amet, dolor sit igitur[/m]")
        ];
        
        let result = format_entries(lines);

        assert_eq!(result[0].word, "foo");
        assert_eq!(result[0].definitions[0], "	[m1]Lorem ipsum dolor sit amet, dolor sit igitur[/m]");
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
        
        let result = format_entries(lines);

        assert_eq!(result[0].word, "foo");
        assert_eq!(result[0].definitions[0], "	[m1]Lorem ipsum dolor sit amet, dolor sit igitur[/m]");

        assert_eq!(result[1].word, "bar");
        assert_eq!(result[1].definitions[0], "	[m1][b]Dolor[/b] sit igitur.[/m]");

        assert_eq!(result[2].word, "bar-like-word-with-same-defs");
        assert_eq!(result[2].definitions[0], "	[m1][b]Dolor[/b] sit igitur.[/m]");


        assert_eq!(result[3].word, "baz");
        assert_eq!(result[3].definitions[0], "	[m1]Lorem ipsum dolor sit amet, consectetur adipiscing elit[/m]");
        assert_eq!(result[3].definitions[1], "	[m2]Lorem ipsum dolor sit amet.[/m]");
    }
}