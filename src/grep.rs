
pub fn minigrep() {
    let args: Vec<String> = std::env::args().collect();

    dbg!(&args);

    let query = &args[2];
    let file_path = &args[3];

    println!("Query: {}", query);
    println!("File path: {}", file_path);

    let file_contents = std::fs::read_to_string(file_path)
        .expect("Unable to read file");

    println!("File contents: {}", file_contents);
}
