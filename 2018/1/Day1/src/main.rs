use std::error::Error;
use std::fs::File;
//use std::io::prelude::*;
use std::path::{Path, Display};
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];

    println!("Filename is {:?}", filename);

    // This section adapted from:
    // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

    let path = Path::new(filename);
    let display = path.display();

    let mut total: i32 = 0;

    if let Ok(lines) = read_lines(&path, display) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(str_input) = line {
                match str_input.parse::<i32>() {
                    Ok(n) => total = total + n,
                    Err(e) => println!("Erroneous input: {}", e),
                }
            }
        }
    }

    println!("Total: {}", total);

}

fn read_lines<P>(path: P, display: Display) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = match File::open(&path) {
        Err(why) => panic!("Error opening {}: {}", display, why.description()),
        Ok(file) => file,
    };
    Ok(io::BufReader::new(file).lines())
}
