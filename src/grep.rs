
pub fn minigrep() {
    let args: Vec<String> = std::env::args().collect();

    dbg!(&args);

    let (query, file_path) = parse_config(&args);

    println!("Query: {}", query);
    println!("File path: {}", file_path);

    let file_contents = std::fs::read_to_string(file_path)
        .expect("Unable to read file");

    println!("File contents: {}", file_contents);
}

fn parse_config(args: &[String]) -> (&String, &String) {
    let query = &args[2];
    let file_path = &args[3];

    (query, file_path)
}

#[cfg(test)]
mod tests {
    use crate::grep::parse_config;

    #[test]
    // Note I wish I could move args into the parse_config function and therefore not have to
    // declare args in a let block. But my Rust is not yet good enough to know how to do that.
    fn can_parse_config_from_command_line_args() {
        let args = args_of(&["target/debug/hello-rust", "grep", "This", "hello.txt"]);
        let (query, file_path) = parse_config(&args);
        assert_eq!(query, &"This".to_string());
        assert_eq!(file_path, &"hello.txt".to_string());
    }

    fn args_of(vec2: &[&str]) -> Vec<String> {
        let vec1 = vec2
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        vec1
    }
}
