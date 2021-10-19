use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_dsl_file(filename: &str) -> bool {
    Path::new(&filename).extension().unwrap().eq("dsl")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_dsl_file(filename: String) -> Result<Vec<String>, &'static str> {
    if is_dsl_file(&filename) {
        return match read_lines(&filename) {
            Ok(line_results) => {
                let mut lines = vec![];
                for line_result in line_results {
                   lines.push(line_result.unwrap_or_else(|_| String::from("Could not read all DSL lines")));
                }

                Ok(lines)
            }, 
            Err(_e) => Err("Could not read the given DSL file"),
        }
    }

    Err("Given file was not a DSL file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_dsl_files() {
        let filename1 = "foo.dsl";
        let filename2 = "bar.dsl";
        let filename3 = "foo.json";
        let filename4 = "virus.exe";

        let result1 = is_dsl_file(&filename1);
        let result2 = is_dsl_file(&filename2);
        let result3 = is_dsl_file(&filename3);
        let result4 = is_dsl_file(&filename4);

        assert_eq!(result1, true);
        assert_eq!(result2, true);
        assert_eq!(result3, false);
        assert_eq!(result4, false);
    }

    #[test]
    fn errors_on_non_dsl_files() {
        let filename = String::from("undefined.json");

        let result = read_dsl_file(filename);

        assert_eq!(result, Err("Given file was not a DSL file"));
    }

    #[test]
    fn errors_on_invalid_dsl_files() {
        let filename = String::from("undefined.dsl");

        let result = read_dsl_file(filename);

        assert_eq!(result, Err("Could not read the given DSL file"));
    }

    #[test]
    fn reads_dsl_file() {
        let filename = String::from("src/reader/fixtures/dummy.dsl");

        let result = read_dsl_file(filename).unwrap();

        assert_eq!(result[0], "#NAME	\"Test Dictionary fixture\"");
        assert_eq!(result[1], "#INDEX_LANGUAGE	\"English\"");
        assert_eq!(result[2], "#CONTENTS_LANGUAGE\t\"Latin\"");
        assert_eq!(result[3], "foo");
        assert_eq!(result[4], "	[m1]Lorem ipsum dolor sit amet, dolor sit igitur[/m]");
    }
}