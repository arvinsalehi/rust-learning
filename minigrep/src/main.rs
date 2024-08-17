use std::error::Error;
use std::fs;
use std::env;
use std::process;

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("Not enough input arguments"))
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename})
    }
}

fn search_txt_file(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    println!("{} with {}", content,config.query);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e | {
        println!("Problem Parsing arguments : {}", e);
        process::exit(1);
    });

    if let Err(err) = search_txt_file(config) {
        println!("Application Exited because of: {}", err);
        process::exit(-1)
    }
}
