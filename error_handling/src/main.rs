use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, ErrorKind},
};

fn main() {
    let f = File::open("hello.txt");

    let mut map = HashMap::new();

    let f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            _ => panic!("Error without custom message"),
        },
    };

    let reader = BufReader::new(f);
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let count = map.entry(line).or_insert(0);
                *count += 1;
            },
            Err(e) => panic!("Error reading line: {:?}", e),
        }
    }

    // Print out the map to see the results
    for (line, count) in &map {
        println!("Line: '{}', Count: {}", line, count);
    }
}
