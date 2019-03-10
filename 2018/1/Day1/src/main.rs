fn main() {
    println!("Hello, world!");

    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];

    println!("Filename is {:?}", filename);

    let contents = std::fs::read_to_string(filename)
    .expect("Error reading {:?}", filename);
}
