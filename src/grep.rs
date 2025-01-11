
pub fn minigrep() {
    let args: Vec<String> = std::env::args().collect();

    dbg!(&args);

    let config = parse_config(&args);

    println!("Query: {}", config.query);
    println!("File path: {}", config.file_path);

    let file_contents = std::fs::read_to_string(config.file_path)
        .expect("Unable to read file");

    println!("File contents: {}", file_contents);
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[2].clone();
    let file_path = args[3].clone();

    Config { query, file_path }
}

#[cfg(test)]
mod tests {
    use crate::grep::parse_config;

    #[test]
    fn can_parse_config_from_command_line_args() {
        let config = parse_config(&args_of(&["target/debug/hello-rust", "grep", "This", "hello.txt"]));
        assert_eq!(config.query, "This".to_string());
        assert_eq!(config.file_path, "hello.txt".to_string());
    }

    fn args_of(vec2: &[&str]) -> Vec<String> {
        let vec1 = vec2
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        vec1
    }
}
