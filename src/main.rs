use std::env;
use std::process;
use obsidian_site::Config; 
// TODO: 
// - refactor lib.rs to match possible ConfigOption Enums
// - handle adding files to config with this


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing file! {err}");
        process::exit(1);
    });
    
    // println!(sdfsdfsdf"Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = obsidian_site::run(config) {
        println!("Problem parsing file! {e}");
        process::exit(1);
 
    }

}


