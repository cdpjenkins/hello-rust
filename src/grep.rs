use std::error::Error;

pub fn minigrep() {
    let args: Vec<String> = std::env::args().collect();

    let config =
        Config::build_from_args(&args)
            .unwrap_or_else(|err| {
                println!("Error parsing arguments: {err}");
                std::process::exit(1);
        });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        std::process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = std::fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &file_contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

#[derive(PartialEq, Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {

    fn build_from_args(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            Err("Not enough arguments")
        } else {
            Ok(Config {
                query: args[2].clone(),
                file_path: args[3].clone()
            })
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::grep::Config;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn can_parse_config_from_command_line_args() {
        let args = args_of(&["target/debug/hello-rust", "grep", "This", "hello.txt"]);
        assert_eq!(
            Config::build_from_args(&args).unwrap(),
            Config::of("This", "hello.txt")
        );
    }

    #[test]
    fn returns_error_if_insufficient_command_line_args() {
        let args = args_of(&["one", "two"]);
        assert_eq!(
            Config::build_from_args(&args),
            Err("Not enough arguments")
        );
    }

    fn args_of(vec2: &[&str]) -> Vec<String> {
        let vec1 = vec2
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        vec1
    }

    impl Config {
        fn of(query: &str, file_path: &str) -> Config {
            Config {
                query: query.to_string(),
                file_path: file_path.to_string()
            }
        }
    }
}
