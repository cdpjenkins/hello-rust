
pub fn minigrep() {
    let args: Vec<String> = std::env::args().collect();

    dbg!(&args);

    let query = &args[2];
    let file_path = &args[3];

    println!("Query: {}", query);
    println!("File path: {}", file_path);
}
