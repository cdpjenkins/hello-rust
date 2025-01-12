
pub fn minigrep() {
    let args: Vec<String> = std::env::args().collect();

    dbg!(&args);

    let config = Config::build_from_args(&args);

    println!("Query: {}", config.query);
    println!("File path: {}", config.file_path);

    let file_contents = std::fs::read_to_string(config.file_path)
        .expect("Unable to read file");

    println!("File contents: {}", file_contents);
}

#[derive(PartialEq, Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {

    fn build_from_args(args: &[String]) -> Config {
        Config {
            query: args[2].clone(),
            file_path: args[3].clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grep::Config;

    #[test]
    fn can_parse_config_from_command_line_args() {
        let args = args_of(&["target/debug/hello-rust", "grep", "This", "hello.txt"]);
        assert_eq!(
            Config::build_from_args(&args),
            Config::of("This", "hello.txt")
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
