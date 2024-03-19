use std::env;
use std::error::Error;
use std::fs;

mod parser;
mod searcher_functions;

//enviroment variable for print tree
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let mut parsed_tree = parser::parse_query(&config.query);

    if config.display_tree {
        ptree::print_tree(&parsed_tree).unwrap();
    }
    loop {
        // Determine next query to search
        let next_query = parsed_tree.breadth_first_node().unwrap();
        // Check if it is in the document
        let mut result: bool =
            searcher_functions::search(&next_query, &contents, config.ignore_case);
        // Update the tree
        while parsed_tree.update_tree(&next_query, result) {}
        //last leaf might be either already searched or not

        if parsed_tree.is_leaf() {
            if next_query != parsed_tree.breadth_first_node().unwrap() {
                result = searcher_functions::search(
                    &parsed_tree.breadth_first_node().unwrap(),
                    &contents,
                    config.ignore_case,
                );
            }

            println!("This document fulfills the query: {}", result);
            break;
        }
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub display_tree: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        let display_tree = env::var("DISPLAY_TREE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
            display_tree,
        })
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(true, searcher_functions::search(query, contents, false));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(true, searcher_functions::search(query, contents, true));
    }

    #[test]
    #[ignore]
    fn read_complicated_pdf() {
        let query = "Transformer";
        let file_path = "2403.05530.pdf";

        match searcher_functions::search_pdf(query, file_path) {
            Ok(result) => {
                println!("{:?}", result);
                assert!(result.len() > 0);
            }
            Err(..) => (),
        }
    }
}
