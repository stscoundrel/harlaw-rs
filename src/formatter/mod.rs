use crate::DictionaryEntry;

const TAB: &str = "\t";
const SKIPS: &[&str]= &["#"];

pub fn format_entries(lines: Vec<String>) -> Vec<DictionaryEntry> {
    let mut dictionary_entries: Vec<DictionaryEntry> = vec![];
    let mut index = 0;

    for line in lines {
        let first_character = line.chars().next().unwrap().to_string();

        // Skip metadata lines.
        if SKIPS.contains(&&first_character[..]) {
            continue;
        }

        // If line startes with tab, it is definition of previous entry.
        if first_character.eq(TAB) {
            dictionary_entries[index -1].definitions.push(String::from(&line));
        }

        // The line is a headword, form a new entry. 
        dictionary_entries.push(DictionaryEntry {
            word: String::from(&line),
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
}